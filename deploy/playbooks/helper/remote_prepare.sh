#!/bin/bash
 
set -e
set -u

groupadd -g 2000 deploy
useradd -u 2000 -g 2000 deploy
mkdir /home/deploy

mkdir /home/deploy/.ssh
chmod 700 /home/deploy/.ssh
touch /home/deploy/.ssh/authorized_keys
echo "ssh-ed25519 AAAAC3NzaC1lZDI1NTE5AAAAIMKgs6U2Xe0JuWoPW1UWXz21W5weSMu7sJkTaTXy3Hbk benjamin.boortz@secure.mailbox.org" > /home/deploy/.ssh/authorized_keys
chmod 400 /home/deploy/.ssh/authorized_keys
chown deploy:deploy /home/deploy -R
echo 'deploy ALL=(ALL) NOPASSWD: ALL' > /etc/sudoers.d/20deploy

source /etc/os-release

if [ "$ID_LIKE" == "arch" ]; then
	usermod -G deploy,wheel deploy

else
	usermod -G deploy,sudo deploy

fi
