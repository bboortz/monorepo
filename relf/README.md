# relf

A ELF file analyzer written in rust.


# Usage

* help: `./relf --help`
* anaylize a file: `./relf <FILE>`
* anaylize a file in verbose mode: `./relf -v <FILE>`
* anaylize a file in debug mode: `./relf -v <FILE>`


## Environment Variables

* `APP_LOG_LEVEL` 
  * warn   - default
  * info   - same as `--verbose`
  * debug  - same as `--debug`
