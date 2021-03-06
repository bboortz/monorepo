#!/bin/bash

set -e
set -u 

mkdir -p temp
cd temp
curl -L https://github.com/upx/upx/releases/download/v3.96/upx-3.96-amd64_linux.tar.xz | tar xJf -
mv upx-3.96-amd64_linux/upx .
cd -

cargo build --release
strip target/release/crackme1
./temp/upx target/release/crackme1
du -sh target/release/crackme1
