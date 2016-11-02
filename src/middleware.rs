use std::error::Error;
use std::result::Result;
use nickel::{Request, Response, Middleware, Continue, MiddlewareResult};
use nickel::status::StatusCode;
use r2d2_postgres::{PostgresConnectionManager, TlsMode};
use r2d2::{Config, Pool, PooledConnection, GetTimeout};
use typemap::Key;
use plugin::Extensible;

pub struct PostgresMiddleware {
    pub pool: Pool<PostgresConnectionManager>,
}

impl PostgresMiddleware {
    /// Create middleware using defaults
    ///
    /// The middleware will be setup with no ssl and the r2d2 defaults.
    pub fn new(db_url: &str) -> Result<PostgresMiddleware, Box<Error>> {
        let manager = try!(PostgresConnectionManager::new(db_url, TlsMode::None));
        let pool = try!(Pool::new(Config::default(), manager));

        Ok(PostgresMiddleware { pool: pool })
    }

    /// Create middleware using pre-built `r2d2::Pool`
    ///
    /// This allows the caller to create and configure the pool with specific settings.
    pub fn with_pool(pool: Pool<PostgresConnectionManager>) -> PostgresMiddleware {
        PostgresMiddleware { pool: pool }
    }
}

impl Key for PostgresMiddleware { type Value = Pool<PostgresConnectionManager>; }

impl<D> Middleware<D> for PostgresMiddleware {
    fn invoke<'mw, 'conn>(&self, req: &mut Request<'mw, 'conn, D>, res: Response<'mw, D>) -> MiddlewareResult<'mw, D> {
        req.extensions_mut().insert::<PostgresMiddleware>(self.pool.clone());

        Ok(Continue(res))
    }
}

/// Add `pg_conn()` helper method to `nickel::Request`
///
/// This trait must only be used in conjunction with `PostgresMiddleware`.
///
/// On error, the method returns a tuple per Nickel convention. This allows the route to use the
/// `try_with!` macro.
///
/// Example:
///
/// ```ignore
/// app.get("/my_counter", middleware! { |request, response|
/// 	let db = try_with!(response, request.pg_conn());
/// });
/// ```
pub trait PostgresRequestExtensions {
    fn pg_conn(&self) -> Result<PooledConnection<PostgresConnectionManager>, (StatusCode, GetTimeout)>;
}

impl<'a, 'b, D> PostgresRequestExtensions for Request<'a, 'b, D> {
    fn pg_conn(&self) -> Result<PooledConnection<PostgresConnectionManager>, (StatusCode, GetTimeout)> {
        self.extensions()
            .get::<PostgresMiddleware>()
            .expect("PostgresMiddleware must be registered before using PostgresRequestExtensions::pg_conn()")
            .get()
            .or_else(|err| Err((StatusCode::InternalServerError, err)))
    }
}
