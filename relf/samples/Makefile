
help:               ## printing out the help
	@echo
	@echo *samples*
	@echo
	@echo "--- TARGETS ---"
	@fgrep -h "##" $(MAKEFILE_LIST) | fgrep -v fgrep | sed -e 's/\\$$//' | sed -e 's/##//'
	@echo "------"
	@echo

clean:              ## clean up this project
	rm -f *.o entry_point hello small1 small2 small3 small4 small5 small6 small7 small8
c: clean

entry_point:
	gcc -o entry_point entry_point.c

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

small4:
	nasm -o small4 small4.asm

small5:
	nasm -o small5 small5.asm

small6:
	nasm -o small6 small6.asm

small7:
	nasm -o small7 small7.asm

small8:
	nasm -o small8 small8.asm

build: entry_point hello small1 small2 small3 small4 small5 small6 small7 small8             ## build the software
b: build
