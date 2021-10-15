#!/bin/bash

export RUSTDOCFLAGS="-Cpanic=abort"
export CARGO_INCREMENTAL=0
export LLVM_PROFILE_FILE="target/prof/rusterella-%p-%m.profraw"
# export RUSTFLAGS="-Zprofile -Ccodegen-units=1 -Copt-level=0 -Clink-dead-code -Coverflow-checks=off -Zpanic_abort_tests -Cpanic=abort"
export RUSTFLAGS="-Zinstrument-coverage"
export RUSTUP_TOOLCHAIN=nightly


mkdir -p temp
cd temp
curl -L https://github.com/mozilla/grcov/releases/latest/download/grcov-linux-x86_64.tar.bz2 | tar jxf -
cd -

cargo version
cargo build
cargo test

zip target/prof/rusterella.zip target/prof/*.profraw

du -shc target/prof/rusterella.zip
du -shc target/debug

./temp/grcov target/prof/rusterella.zip -s ./ -t html --llvm --branch --ignore-not-existing --ignore "/*" -o target/coverage -b target/debug

