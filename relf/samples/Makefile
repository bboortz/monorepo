
help:               ## printing out the help
	@echo
	@echo *samples*
	@echo
	@echo "--- TARGETS ---"
	@fgrep -h "##" $(MAKEFILE_LIST) | fgrep -v fgrep | sed -e 's/\\$$//' | sed -e 's/##//'
	@echo "------"
	@echo

clean:              ## clean up this project
	rm -f *.o hello small1 small2 small3
c: clean

hello: hello.o
	ld hello.o -o hello
hello.o:
	nasm -f elf64 hello.asm

small1: small1.o
	ld small1.o -o small1 
small1.o:
	nasm -f elf64 small1.asm

small2: 
	nasm -o small2 small2.asm

small3:
	nasm -o small3 small3.asm

build: hello small1 small2 small3             ## build the software
b: build
