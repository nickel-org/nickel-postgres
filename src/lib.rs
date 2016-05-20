extern crate nickel;
extern crate r2d2;
extern crate r2d2_postgres;
extern crate typemap;
extern crate plugin;

pub use middleware::{ PostgresMiddleware, PostgresRequestExtensions };

mod middleware;
