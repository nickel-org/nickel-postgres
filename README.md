nickel-postgres
=======

A postgres middleware for nickel.rs

## Usage

See examples for usage.

## Lib vs Unboxed closures

Much of the utility of this library can be gained from using unboxed
closures to capture a connection pool, this also removes the risk of
accessing a connection pool in a handler attached before the middleware.

### `Cargo.toml`:

    [dependencies.nickel_postgres]
    git = "https://github.com/nickel-org/nickel-postgres.git"

## Licence

MIT
