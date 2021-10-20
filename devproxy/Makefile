DEPL		?= dev


help:               ## printing out the help
	@echo
	@echo *rusterella*
	@echo
	@echo "--- TARGETS ---"
	@fgrep -h "##" $(MAKEFILE_LIST) | fgrep -v fgrep | sed -e 's/\\$$//' | sed -e 's/##//'
	@echo "------"
	@echo

clean:              ## clean up this project
	cargo clean
	rm -rf ./temp

fmt:                ## format the sources
	cargo fmt
	cargo clippy

test: fmt           ## test the software
	./scripts/test.sh

build: clean fmt    ## build the software
	./scripts/build.sh

doc: build          ## create the documentation
	cargo doc


init:               ##
	rustup update
	rustup default nightly-x86_64-unknown-linux-gnu
	rustup update
	rustup component add clippy
	rustup component add rustfmt
	rustup component add rls rust-analysis rust-src
