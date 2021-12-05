VAR_FILE     ?= dev-blue.tfvars


help:                     ## printing out the help
	@echo
	@echo blue-green-azure-terraform
	@echo
	@echo --- TARGETS ---
	@fgrep -h "##" $(MAKEFILE_LIST) | fgrep -v fgrep | sed -e 's/\\$$//' | sed -e 's/##//'

fmt:                      ## format terraform files
	terraform fmt -recursive

validate: fmt             ## validate terraform files
	terraform validate

plan: validate            ## plan terraform deployment
	terraform plan -var-file="$(VAR_FILE)"

apply: validate           ## apply terroform deployment
	terraform apply -var-file="$(VAR_FILE)"

refresh: validate         ## fetch remote config statefle
	terraform apply -refresh-only

destroy: validate         ## destroy terrafor deployment
	terraform destroy

login:                    ## login into azure
	az login

account:                  ## show current azure accoint
	az account show

location:                 ## show locations
	az account list-locations -o table
