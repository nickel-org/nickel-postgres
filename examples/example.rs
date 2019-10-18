#[macro_use]
extern crate nickel;
extern crate nickel_postgres;

use nickel::{HttpRouter, Nickel};
use nickel_postgres::{PostgresMiddleware, PostgresRequestExtensions};
use std::env;

fn main() {
    let mut app = Nickel::new();

    let postgres_url = env::var("DATABASE_URL").unwrap();
    let mw = PostgresMiddleware::new(&postgres_url).unwrap();
    app.utilize(mw);

    app.get(
        "/my_counter",
        middleware! { |request, response|
            let _connection = try_with!(response, request.pg_conn());

            // use connection
        },
    );

    app.get("**", middleware! { println!("!!!") });
}
