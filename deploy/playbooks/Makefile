

help:                     ## printing out the help
	@echo
	@echo MISChaufen Ansible Playbooks
	@echo
	@echo --- TARGETS ---
	@fgrep -h "##" $(MAKEFILE_LIST) | fgrep -v fgrep | sed -e 's/\\$$//' | sed -e 's/##//'

clean:                    ## cleanup all temporary/work data
	rm -rf .venv

bootstrap:                ## prepare local environment
	sudo ./helper/local_prepare_root.sh
	./helper/local_prepare_nonroot.sh

local.deploy:             ## local deployment using ansible playbooks
	ansible-playbook -i ./hosts -l local local.site.yml

home.deploy:              ## home deployment using ansible playbooks
	ansible-playbook -i ./hosts -l home site.yml

server.home.deploy:      ## server2.home deployment using ansible playbooks
	ansible-playbook -i ./hosts -l server_home site.yml

t540p.home.deploy:        ## t540p.home deployment using ansible playbooks
	ansible-playbook -i ./hosts -l t540p_home site.yml

benni.tech.deploy:        ## benni.tech deployment using ansible playbooks
	ansible-playbook -i ./hosts -l benni_tech site.yml

test.benni.tech.deploy:   ## test deployment using ansible playbooks
	ansible-playbook -i ./hosts -l test_benni_tech site.yml

prod.benni.tech.deploy:    ## prod deployment using ansible playbooks
	ansible-playbook -i ./hosts -l prod_benni_tech site.yml

deploy:                   ## deployment using ansible playbooks
	ansible-playbook -i ./hosts site.yml

conn.test:                ## run connetion tests
	ansible -i ./hosts -m ping -u deploy all
