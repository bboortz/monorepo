#!/bin/bash

source /etc/os-release
INSTALL_CMD="apt-get install"
if [ "$ID_LIKE" == "arch" ]; then
	INSTALL_CMD="pacman -S"
fi

echo "INFO: installing necessary OS packages ..."

which make || $INSTALL_CMD make
which python3 || $INSTALL_CMD python3
if [ "$ID_LIKE" == "arch" ]; then
	which python-pip || $INSTALL_CMD python-pip
else
	which python3-venv || $INSTALL_CMD python3-venv
	which python3-pip || $INSTALL_CMD python3-pip
fi
