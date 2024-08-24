# Memecycle.rs

## Intro

This is a refactor of an old side project I had called memecycle. You can find
that project [here](https://github.com/vincepmartin/memecycle).

The reason for this refactor was an excuse to dive into creating a basic web
app in Rust.

Main libraries used in this application are:

- [Diesel.rs](https://diesel.rs)
- [Rocket.rs](https://rocket.rs)

Persistence is handled via a PostgreSQL DB.

## Development Setup

You will have to obviously have rust and have the available build tools to get
diesel.rs to build.

On my system, a GNU/Linux (Ubuntu) I had to install `libpq-dev` so that diesel
could communicate with the DB. That was installed via `sudo apt install libpq-dev`.

Once this is built you will also have to start up the database docker image in
./db with `docker-compose up`.

With the DB up you will then have to run your database migration files. This
is done via the `diesel-cli` tool.

Install `diesel-cli` via

`cargo install diesel_cli --no-default-features --features postgres`

Then create your db and run the migrations via

`diesel database setup`
