DEPL         ?= dev
VAR_FILE     ?= $(DEPL).tfvars
CREDS_FILE   ?= creds.tfvars

export PATH := ./temp:$(PATH)


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
	terraform plan -var-file="$(VAR_FILE)" -out="$(DEPL).tfplan"

apply: validate           ## apply terroform deployment
	terraform apply "$(DEPL).tfplan"

refresh: validate         ## fetch remote config statefle
	terraform apply -refresh-only -var-file="$(VAR_FILE)"

destroy: validate         ## destroy terrafor deployment
	terraform destroy -var-file="$(VAR_FILE)"

login:                    ## login into azure
	az login

account:                  ## show current azure accoint
	az account show

location:                 ## show locations
	az account list-locations -o table

init:                     ## initialize terraform
	./scripts/init.sh
	terraform init
