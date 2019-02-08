#!/usr/bin/env bash

pwd
ls -a
docker run -ti -v $PWD:/usr/src/project -w /usr/src/project rustlang/rust:nightly cargo build --release
docker-compose up -d
cd integration && cargo test -- --nocapture
docker-compose down