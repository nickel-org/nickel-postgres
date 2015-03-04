extern crate nickel;
extern crate postgres;
extern crate r2d2;
extern crate r2d2_postgres;
extern crate plugin;
extern crate typemap;

pub use middleware::{ PostgresMiddleware, PostgresRequestExtensions };

mod middleware;
