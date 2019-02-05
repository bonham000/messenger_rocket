web: ROCKET_ADDRESS=0.0.0.0 ROCKET_PORT=$PORT ./target/release/messenger-rocket
release: cargo install diesel_cli && cp $(which diesel) target/release/ && ./target/release/diesel setup && ./target/release/diesel migration run
