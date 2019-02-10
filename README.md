
# Rocket Messenger App

### Rust server using Rocket, WS-RS, Diesel, PostgreSQL, and Docker to support a real-time messenging app

***

**Running the Server**

Install Docker and run the command `docker-compose up`.

Or, run the server directly with `cargo run` (use nightly Rust).

**Running Tests**

To run the project unit tests run `cargo test`.

To run integration tests run the script `test.sh`. This will build a release of the Rocket server and run it with docker compose, and then run a test suite against the live server. The integration tests live in the `integration`/ directory.

**Todo**

* Get integration tests to run in CI.
* Figure out why websockets doesn't work in deployment.

**Frontend**

Built with React Native: https://github.com/bonham000/messenger-app.