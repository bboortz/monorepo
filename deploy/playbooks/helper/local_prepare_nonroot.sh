#!/bin/bash

if [ ! -d ".venv" ]; then
    echo "INFO: creating .venv ..."
    python3 -m venv .venv
fi

echo "INFO: installing/updating packages into .venv ..."
source .venv/bin/activate
pip3 install -U pip
pip3 install -U -r requirements.txt
ansible-galaxy install GROG.management-user
ansible-galaxy install Akman.timezone
deactivate


echo "INFO: installing further ansible plugins ..."
mkdir -p ~/.ansible/plugins/lookup && cd "$_"
curl https://raw.githubusercontent.com/viczem/ansible-keepass/master/keepass.py -o ./keepass.py
cd -
