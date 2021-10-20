#!/bin/bash

set -e
set -u 

PROJECT_NAME="devproxy"

if [ ! -f ./temp/upx ]; then
    mkdir -p temp
    cd temp
    curl -L https://github.com/upx/upx/releases/download/v3.96/upx-3.96-amd64_linux.tar.xz | tar xJf -
    mv upx-3.96-amd64_linux/upx .
    cd -
fi

cargo build --release
du -sh target/release/${PROJECT_NAME}
strip target/release/${PROJECT_NAME}
./temp/upx target/release/${PROJECT_NAME}
du -sh target/release/${PROJECT_NAME}
