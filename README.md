
# Rocket Messenger App

### Rust server using Rocket, WS-RS, Diesel, PostgreSQL, and Docker to support a real-time messenging app

***

**Running the Server**

Install Docker and run the command `docker-compose up`.

Or, run the server directly with `cargo run` (use nightly Rust). To do this you must have Postgres running locally and provide a `DATABASE_URL` environment variable for Diesel to connect to Postgres. For instance:

`DATABASE_URL=postgresql://user:password@localhost:5432/ rustup run nightly cargo run`

**Running Tests**

To run the project unit tests run `cargo test`.

To run integration tests run the script `test.sh`. This will build a release of the Rocket server and run it with docker compose, and then run a test suite against the live server. The integration tests live in the `integration`/ directory.

**CI**

Continuous integration setup with CircleCI (see `.circleci/config.yml`).

**Deploy**

Deployed with Heroku using the [Heroku Rust buildpack](https://github.com/emk/heroku-buildpack-rust). 

**Frontend**

Built with React Native: https://github.com/bonham000/messenger-app.

**Todo**

* Get integration tests to run in CI.
* Figure out why websockets doesn't work in deployment.