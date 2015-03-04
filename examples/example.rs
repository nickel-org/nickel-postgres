extern crate r2d2;
extern crate postgres;
extern crate openssl;
extern crate nickel;
extern crate nickel_postgres;
#[macro_use] extern crate nickel_macros;

use std::env;
use r2d2::NoopErrorHandler;
use postgres::SslMode;
use openssl::ssl::{SslContext, SslMethod};
use nickel::{Nickel, HttpRouter};
use nickel_postgres::{PostgresMiddleware, PostgresRequestExtensions};

fn main() {
    let mut app = Nickel::new();

    let ssl_context = SslContext::new(SslMethod::Tlsv1).unwrap();
    let postgres_url = env::var("DATABASE_URL").unwrap();
    let dbpool = PostgresMiddleware::new(&*postgres_url,
                                         SslMode::Require(ssl_context),
                                         5,
                                         Box::new(NoopErrorHandler)).unwrap();
    app.utilize(dbpool);
    app.get("/my_counter", middleware! { |request|
        let connection = request.db_conn();

        // use connection
    });

    app.get("**", middleware! { println!("!!!") });
}
