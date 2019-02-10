#!/usr/bin/env bash

docker run -ti -v $PWD:/usr/src/messenger_rocket -w /usr/src/messenger_rocket rustlang/rust:nightly cargo build --release
export ENV=TEST
docker-compose up -d
cd integration && cargo test -- --nocapture