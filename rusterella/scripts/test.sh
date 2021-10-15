#!/bin/bash

set -e
set -u

# export RUSTDOCFLAGS="-Cpanic=abort"
export CARGO_INCREMENTAL=0
export LLVM_PROFILE_FILE="target/prof/rusterella-%p-%m.profraw"
# export RUSTFLAGS="-Zprofile -Ccodegen-units=1 -Copt-level=0 -Clink-dead-code -Coverflow-checks=off -Zpanic_abort_tests -Cpanic=abort"
#export RUSTFLAGS="-Zinstrument-coverage"
export RUSTUP_TOOLCHAIN=nightly
export RUSTFLAGS="-Zprofile -Ccodegen-units=1 -Cinline-threshold=0 -Clink-dead-code -Coverflow-checks=off -Cpanic=abort -Zpanic_abort_tests -Zinstrument-coverage "
export RUSTDOCFLAGS="-Zprofile -Ccodegen-units=1 -Cinline-threshold=0 -Clink-dead-code -Coverflow-checks=off -Cpanic=abort -Zpanic_abort_tests -Zinstrument-coverage"


mkdir -p temp
cd temp
curl -L https://github.com/mozilla/grcov/releases/latest/download/grcov-linux-x86_64.tar.bz2 | tar jxf -
cd -

cargo build
cargo test

zip target/prof/rusterella.zip target/prof/*.profraw

du -shc target/prof/rusterella.zip
du -shc target/debug

# ./temp/grcov target/prof/rusterella.zip -s ./ -t html --llvm --branch --ignore-not-existing --ignore "/*" -o target/coverage -b target/debug
./temp/grcov . -s ./ --binary-path target/debug --llvm --branch --ignore-not-existing --ignore "/*" -t html -o target/coverage
./temp/grcov . -s ./ --binary-path target/debug --llvm --branch --ignore-not-existing --ignore "/*" --token "${CODECOV_TOKEN}" -t coveralls -o target/coveralls.json

ls -la target/coverage
ls -la target/coveralls.json

bash <(curl -s https://codecov.io/bash) -f target/coveralls.json 

