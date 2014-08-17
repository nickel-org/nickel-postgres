# `nickel-postgres`

A postgres middleware for nickel.rs

## Usage

### `Cargo.toml`:

    [dependencies.nickel_postgres]

    git = "https://github.com/nickel-org/nickel-postgres.git"
    #NOTE necessary alternative until pull request is merged; remove then
    #git = "https://github.com/bguiz/nickel-postgres.git"
    #rev = "feature/init"

### Imports

    extern crate nickel_postgres;
    use nickel_postgres::{ PostgresMiddleware, PostgresMiddlewareRequestConvenience };

### Use Middleware

    let mut server = Nickel::new();
    let postgres_middleware: PostgresMiddleware = PostgresMiddleware::new(
        "postgres://postgres:postgres@localhost", postgres::NoSsl, 5);
    server.utilize(postgres_middleware);

### Access Database Connection from `Request`

    fn a_handler_function(req: &Request, response: &mut Respose) {
        let db_conn = req.db_conn();
        // use db_conn
    }

## Example

An complete example of how to use this can be
[found here](https://github.com/bguiz/rust-scratch/tree/master/nickel-postgres).

## Licence

MIT
