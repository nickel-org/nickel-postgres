extern crate nickel;
extern crate postgres;

use nickel::{ Request, Response, Middleware, Action, Continue };
use postgres::pool::{ PostgresConnectionPool };

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
    fn invoke (&self, req: &mut Request, _resp: &mut Response) -> Action {
        req.map.insert(self.pool.clone().get_connection());
        nickel::Continue
    }
}

//TODO find a way to add utility function to Request object without runnining into:
//error: cannot associate methods with a type outside the crate the type is defined in;
//define and implement a trait or new type instead [E0116]
/*
impl<'a> nickel::request::Request<'a> {
    pub fn db_conn(&self) -> &PooledPostgresConnection {
        return self.map.find::<PooledPostgresConnection>().unwrap();
    }
}
*/
