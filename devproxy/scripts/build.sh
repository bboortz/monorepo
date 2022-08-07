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
strip target/${OUTPUT_PROFILE}/rustolution
./temp/upx target/${OUTPUT_PROFILE}/rustolution
du -sh target/${OUTPUT_PROFILE}/rustolution
