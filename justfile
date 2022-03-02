alias c := check
alias b := build
alias t := test
alias f := fmt
alias l := lint
alias a := all

# check all code in workspace
check:
    cargo check

# test all code in workspace
test:
    cargo test --all

# build all code in workspace
build:
    cargo build --release

# fmt all code in workspace
fmt:
    cargo fmt --all

lint:
    cargo clippy --all --fix

# run all task
all: lint fmt check test build