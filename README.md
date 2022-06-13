# MADY (macro-auto differentiation)

[![Rust Test](https://github.com/MDResearch/research/actions/workflows/rust.yml/badge.svg)](https://github.com/MDResearch/research/actions/workflows/rust.yml) [![Project](https://img.shields.io/badge/Project-WIP-brightgreen)](https://github.com/orgs/MDResearch/projects/3) ![Crates.io](https://img.shields.io/crates/d/mady) ![docs.rs](https://img.shields.io/docsrs/mady) ![GitHub release (latest SemVer including pre-releases)](https://img.shields.io/github/v/release/MDResearch/Mady?include_prereleases)

MADY is open source tool for ahead-of-time automatic differentiation.
In addition to ahead-of-time differentiation, MADY also provide some basic math structures, functions and operations with differentiation support.

## Documentation

see cargo

## differentiation support:

- functions: `min`, `max`
- operations: `add`, `sub`, `mul`, `div`

## Get Started

First, set up project and add ``mady`` as dependency in your Cargo.yml
```toml
[dependencies]
mady = "*"
```

Write a simple fn (only [differentiation support](#differentiation-support) operation/function can be used)

```rust
fn simple(a:isize, b:isize)-> isize{
  a + b
}
```

Finally, add ``#[derive_grad()]``(attribute macro) to your function.
```rust
// isize here, because the output type of simple is isize
#[derive_grad(isize)]
fn simple(a:isize, b:isize)-> isize{
  a + b
}
```

expect output
```rust
fn simple(a:isize, b:isize)-> isize{
  a+b
}
fn grad_simple(a:isize, b:isize)-> (isize,(isize,isize)){
  (a+b, (1, 1))
}
```

$$
grad/_simple(a,b)=(simple(a,b),({d simple \over d a},{d simple \over d b}))
$$

To use unsupported function like ``sin``, add fn named ``grad_{{fn name}}``.
```rust
impl GradSin for f64 {
    fn grad_sin(self) -> self {
        self.cos()
    }
}
```

## example

[hardcoded url](https://github.com/MDResearch/Mady/tree/main/examples)

