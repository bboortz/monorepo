export PROFILE ?= dev
export RUST_TARGET ?= thumbv6m-none-eabi
export MCU ?= arturo182_serpente

help:               ## printing out the help
	@echo
	@echo *surfer*
	@echo
	@echo "--- TARGETS ---"
	@fgrep -h "##" $(MAKEFILE_LIST) | fgrep -v fgrep | sed -e 's/\\$$//' | sed -e 's/##//'
	@echo "------"
	@echo

clean:              ## clean up this project
	cargo clean
	rm -rf ./temp
c: clean

fmt:                ## format the sources
	./scripts/fmt.sh
f: fmt

test: fmt           ## test the software
	./scripts/test.sh
t: test

build: test         ## build the software
	./scripts/build.sh
b: build

release: private PROFILE = release
release: build
r: release

run: fmt
	cargo run

doc: release        ## create the documentation
	cargo doc


init:               ##
	rustup update
	rustup default nightly-x86_64-unknown-linux-gnu
	rustup update
	rustup component add clippy
	rustup component add rustfmt
	rustup component add rust-analysis rust-src
