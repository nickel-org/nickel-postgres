use std::sync::Arc;
use std::error::Error as StdError;

use nickel::{Request, Response, Middleware, Continue, MiddlewareResult};
use r2d2_postgres::{PostgresConnectionManager, SslMode};
use r2d2::{Pool, HandleError, Config, PooledConnection};
use typemap::Key;
use plugin::{Pluggable, Extensible};

pub struct PostgresMiddleware {
    pub pool: Arc<Pool<PostgresConnectionManager>>
}

impl PostgresMiddleware {
    pub fn new(connect_str: &str,
               ssl_mode: SslMode,
               num_connections: u32,
               error_handler: Box<HandleError<::r2d2_postgres::Error>>)
                    -> Result<PostgresMiddleware, Box<StdError>> {
        let manager = try!(PostgresConnectionManager::new(connect_str, ssl_mode));

        let config = Config::builder()
          .pool_size(num_connections)
          .error_handler(error_handler)
          .build();

        let pool = try!(Pool::new(config, manager));

        Ok(PostgresMiddleware { pool: Arc::new(pool) })
    }
}

impl Key for PostgresMiddleware { type Value = Arc<Pool<PostgresConnectionManager>>; }

impl<D> Middleware<D> for PostgresMiddleware {
    fn invoke<'mw, 'conn>(&self, req: &mut Request<'mw, 'conn, D>, res: Response<'mw, D>) -> MiddlewareResult<'mw, D> {
        req.extensions_mut().insert::<PostgresMiddleware>(self.pool.clone());
        Ok(Continue(res))
    }
}

pub trait PostgresRequestExtensions {
    fn db_conn(&self) -> PooledConnection<PostgresConnectionManager>;
}

impl<'a, 'b, D> PostgresRequestExtensions for Request<'a, 'b, D> {
    fn db_conn(&self) -> PooledConnection<PostgresConnectionManager> {
        self.extensions().get::<PostgresMiddleware>().unwrap().get().unwrap()
    }
}
