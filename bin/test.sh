#!/usr/bin/env bash

RUST_TEST_THREADS=1 cargo test
# cargo test -- --show-output --ignored
# cargo test -- --show-output
# cargo test -- --show-output --ignored
# cargo test print -- --ignored
# cargo test print -- --show-output --ignored
