#!/bin/bash

set -e
set -u 

CURFILE=$( readlink -f $0 )
CURDIR=${CURFILE%/*}
. ${CURDIR}/lib.sh



if [ ! -f ./temp/upx ]; then
  mkdir -p temp
  cd temp
  curl -L https://github.com/upx/upx/releases/download/v3.96/upx-3.96-amd64_linux.tar.xz | tar xJf -
	mv upx-3.96-amd64_linux/upx .
  cd -
fi


cargo build --profile ${PROFILE}
if [ ! -f "${RUST_TARGET_FILE}" ]; then
  exit 0
fi

strip ${RUST_TARGET_FILE}
./temp/upx ${RUST_TARGET_FILE}
du -sh ${RUST_TARGET_FILE}
