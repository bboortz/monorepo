#!/bin/bash

set -e
set -u 


if [ ! -f ./temp/terraform ]; then
    mkdir -p temp
    cd temp
    curl -L https://releases.hashicorp.com/terraform/1.0.11/terraform_1.0.11_linux_amd64.zip --output terraform.zip
    unzip terraform.zip
    cd -
fi
