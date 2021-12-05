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
ansible-galaxy install elao.cron,2.0
ansible-galaxy install bertvv.samba
deactivate


echo "INFO: installing further ansible plugins ..."
mkdir -p ~/.ansible/plugins/lookup && cd "$_"
curl https://raw.githubusercontent.com/viczem/ansible-keepass/master/keepass.py -o ./keepass.py
cd -


if [ ! -d ~/.ssh ]; then
  echo "INFO: creating ~/.ssh ..."
  mkdir -p ~/.ssh
  chmod 700 ~/.ssh
fi

if [ -n "${GH_SSH_HOME_PUB}" ]; then
  echo "INFO: writingn ${GH_SSH_HOME_PUB} ..."
  echo "${GH_SSH_HOME_PUB}" > ~/.ssh/id_ed25519_home.pub
fi
