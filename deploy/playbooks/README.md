## MISChaufen ansible playbooks

The playbooks for MISChaufen. Compatible with Arch Linux and Debian Linux.

## Preparation locally

```
apt-get install make
make bootstrap
cp public-ssh-key ~/.ssh/public-ssh-key
```

## Preparation on Server

1. issue an SSH key
2. bootstrap a server
3. note the servers IP and put it into the file `hosts`
4. configure the groups vars in directory `group_var`
5. login into the remote server as root
6. run all commands in file `helper/remote_prepare.sh` on the remote server as root


## Run the Deployment

```
source .venv/bin/activate
make deploy
```

After the first deployment you SSHD key signature on server side changes. So that you have to cleanup your local `~/.ssh/known_hosts` file.
