#!/bin/bash

source /etc/os-release


which python3 || {
  if [ "$ID_LIKE" == "arch" ]; then
    sudo pacman -S make python3 python-pip
  else
    apt-get install git make python3 python3-venv python3-pip
  fi
}

if [ ! -d ".venv" ]; then
  python3 -m venv .venv
fi

source .venv/bin/activate
python3 -m pip install --upgrade pip
pip install -r requirements.txt
ansible-galaxy install GROG.management-user
deactivate
