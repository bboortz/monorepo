#!/bin/bash

export RUSTDOCFLAGS="-Cpanic=abort"
export CARGO_INCREMENTAL=0
export LLVM_PROFILE_FILE="target/prof/rusterella-%p-%m.profraw"
# export RUSTFLAGS="-Zprofile -Ccodegen-units=1 -Copt-level=0 -Clink-dead-code -Coverflow-checks=off -Zpanic_abort_tests -Cpanic=abort"
export RUSTFLAGS="-Zinstrument-coverage"


cargo build
cargo test

zip target/prof/rusterella.zip target/prof/*.profraw
grcov target/prof/rusterella.zip -s ./ -t html --llvm --branch --ignore-not-existing --ignore "/*" -o target/coverage -b target/debug

