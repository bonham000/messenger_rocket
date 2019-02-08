#!/usr/bin/env bash

docker run -ti -v $PWD:/usr/src/messenger_rocket -w /usr/src/messenger_rocket rustlang/rust:nightly cargo build --release
docker-compose -f docker-compose-test.yml up -d
cd integration && cargo test -- --nocapture