extern crate r2d2;
extern crate r2d2_postgres;
#[macro_use] extern crate nickel;
extern crate nickel_postgres;

use std::env;
use r2d2::{Config, Pool};
use r2d2_postgres::{PostgresConnectionManager, SslMode};
use nickel::{Nickel, HttpRouter};
use nickel_postgres::{PostgresMiddleware, PostgresRequestExtensions};

fn main() {
    let mut app = Nickel::new();

    let postgres_url = env::var("DATABASE_URL").unwrap();
    let db_mgr = PostgresConnectionManager::new(postgres_url.as_ref(), SslMode::None)
        .expect("Unable to connect to database");

    let db_pool = Pool::new(Config::default(), db_mgr)
        .expect("Unable to initialize connection pool");

    app.utilize(PostgresMiddleware::with_pool(db_pool));

    app.get("/my_counter", middleware! { |request, response|
        let _connection = try_with!(response, request.pg_conn());

        // use connection
    });

    app.get("**", middleware! { println!("!!!") });
}
