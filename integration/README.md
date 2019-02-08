
**Server Integration Tests**

This is a Rust program which just contains a test suite to run against a live instance of the server. The tests can be run with the `integration_test` script in the parent directory, which run the server in a docker environment with a new Postgres database. Then, this test suite executes against the server.

