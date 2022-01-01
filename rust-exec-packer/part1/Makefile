export PROFILE ?= dev

help:               ## printing out the help
	@echo
	@echo *rust-exec-packer*
	@echo
	@echo "--- TARGETS ---"
	@fgrep -h "##" $(MAKEFILE_LIST) | fgrep -v fgrep | sed -e 's/\\$$//' | sed -e 's/##//'
	@echo "------"
	@echo

clean:              ## clean up this project
	rm -f hello.o hello entry_point
c: clean

build:              ## build the software
	nasm -f elf64 hello.asm
	ld hello.o -o hello
	gcc -o entry_point entry_point.c
b: build
