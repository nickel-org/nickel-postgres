extern crate r2d2;
extern crate postgres;
extern crate openssl;
#[macro_use] extern crate nickel;
extern crate nickel_postgres;

use std::env;
use r2d2::NopErrorHandler;
use postgres::SslMode;
use nickel::{Nickel, HttpRouter};
use nickel_postgres::{PostgresMiddleware, PostgresRequestExtensions};

fn main() {
    let mut app = Nickel::new();

    let postgres_url = env::var("DATABASE_URL").unwrap();
    let dbpool = PostgresMiddleware::new(&*postgres_url,
                                         SslMode::None,
                                         5,
                                         Box::new(NopErrorHandler)).unwrap();
    app.utilize(dbpool);
    app.get("/my_counter", middleware! { |request|
        let _connection = request.db_conn();

        // use connection
    });

    app.get("**", middleware! { println!("!!!") });
}
