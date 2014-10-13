extern crate nickel;
extern crate postgres;
extern crate r2d2;
extern crate r2d2_postgres;

pub use middleware::{ PostgresMiddleware, PostgresRequestExtensions };

mod middleware;
