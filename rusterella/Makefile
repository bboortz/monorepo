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

fmt:                ## format the sources
	cargo fmt
	cargo clippy

test: fmt           ## test the software
	./scripts/test.sh

build: clean fmt    ## build the software
	cargo build --release
	strip target/release/rusterella
	~/Downloads/upx-3.96-amd64_linux/upx target/release/rusterella
	du -sh target/release/rusterella

doc: build          ## create the documentation
	cargo doc


init:               ##
	rustup update
	rustup component add clippy

