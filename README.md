<div align="center">

# MADY (Macro-gen Automatic Differentiation)

[![Rust Test](https://github.com/MDResearch/research/actions/workflows/rust.yml/badge.svg)](https://github.com/MDResearch/research/actions/workflows/rust.yml) [![Project](https://img.shields.io/badge/Project-WIP-brightgreen)](https://github.com/orgs/MDResearch/projects/3) [![Crates.io](https://img.shields.io/crates/d/mady)](https://crates.io/crates/mady) ![docs.rs](https://img.shields.io/docsrs/mady) [![Crates.io](https://img.shields.io/crates/v/mady)](https://crates.io/crates/mady)

> Note: Mady to move to [MadyLab](https://github.com/MadyLab/Mady)

MADY is open source tool for ahead-of-time automatic differentiation.
In addition to ahead-of-time differentiation, MADY also provide some basic math structures, functions and operations with differentiation support.

  
</div>

## Feature

- Ahead-of-time gen
    
    generate by proc-macro
- Static
    
    no tree or graph struct in runtime, just normal function call
- Multithreading
    
    because it's just a normal function, no hacky code or unsafe
- Fast
    
    just static code, llvm can optimize all the code

## Supported Differentiation

- functions: `min`, `max`
- operations: `add`, `sub`, `mul`, `div`

## Get Started

First, set up project and add ``mady`` as dependency in your Cargo.yml
```toml
[dependencies]
mady = "version here"
```

or
```bash
cargo add mady
```

Write a simple fn (only [differentiation support](#differentiation-support) operation/function can be used)

```rust
fn simple(a:isize, b:isize)-> isize{
  a + b
}
```

Finally, add ``#[derive_grad()]`` (attribute macro) to your function.
```rust
// isize here, because the input grad type of simple is isize
#[derive_grad(isize, isize)]
fn simple(a:isize, b:isize)-> isize{
  a + b
}
```

expect output
```rust
fn simple(a:isize, b:isize)-> isize{
  a + b
}
fn grad_simple(a:isize, b:isize)-> (isize,(isize,isize)){
  (a + b, (1, 1))
}
```

$$
grad\\_simple(a,b)=(simple(a,b),({d simple \over d a},{d simple \over d b}))
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

- [complex number](https://github.com/MDResearch/Mady/blob/main/examples/complex.rs)
- [more example](https://github.com/MDResearch/Mady/tree/main/examples)

