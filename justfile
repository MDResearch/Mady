alias c := check
alias b := build
alias t := test
alias f := fmt
alias a := all

# check all code in workspace
check:
    cargo check

# test all code in workspace
test: check
    cargo test --all

# build all code in workspace
build: test
    cargo build --release

# fmt all code in workspace
fmt:
    cargo fmt --all

# run all task
all: build fmt