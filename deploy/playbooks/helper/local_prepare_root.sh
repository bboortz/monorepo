#!/bin/bash

source /etc/os-release


which python3 || {
  echo "INFO: installing necessary OS packages"
  if [ "$ID_LIKE" == "arch" ]; then
    sudo pacman -S make python3 python-pip
  else
    apt-get install git make python3 python3-venv python3-pip
  fi
}
