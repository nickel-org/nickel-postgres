extern crate nickel;
extern crate postgres;

use nickel::{ Request, Response, Middleware, Action, Continue, NickelError };
use postgres::pool::{ PooledPostgresConnection, PostgresConnectionPool };

#[deriving(Clone)]
pub struct PostgresMiddleware {
    pub pool: PostgresConnectionPool
}

impl PostgresMiddleware {
    pub fn new (connect_str: &str, sslMode: postgres::SslMode, num_connections: uint) -> PostgresMiddleware {
        PostgresMiddleware {
            pool: PostgresConnectionPool::new(connect_str, sslMode, num_connections).unwrap()
        }
    }
}

impl Middleware for PostgresMiddleware {
    fn invoke (&self, req: &mut Request, _resp: &mut Response) -> Result<Action, NickelError> {
        req.map.insert(self.pool.clone().get_connection());
        Ok(Continue)
    }
}

pub trait PostgresRequestExtensions {
    fn db_conn(&self) -> &PooledPostgresConnection;
}

impl<'a, 'b> PostgresRequestExtensions for Request<'a, 'b> {
    fn db_conn(&self) -> &PooledPostgresConnection {
        return self.map.find::<PooledPostgresConnection>().unwrap();
    }
}
