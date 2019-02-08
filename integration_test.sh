#!/usr/bin/env bash

pwd
ls -a
docker run -ti -v $PWD:/usr/src/messenger_rocket -w /usr/src/messenger_rocket rustlang/rust:nightly cargo build --release
docker-compose up -d
cd integration && cargo test -- --nocapture
docker-compose down