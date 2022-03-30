use std::fs::File;
use std::io::prelude::*;

use proc_macro2::TokenStream;
use quote::quote;
use serde_json::from_str;
use syn::parse2;
use syn_codegen::Definitions;

#[macro_use]
mod macros;
mod chain;

const SYN: &str = include_str!("syn.json");

fn main() {
    let defs: Definitions = from_str(SYN).unwrap();
    write_out("mady_macro/src/gen/chain.rs", chain::gen(&defs));
}

fn write_out(path: &str, content: TokenStream) {
    let mut file = File::create(path).unwrap();

    writeln!(
        file,
        "// codegen file by version {}",
        env!("CARGO_PKG_VERSION")
    )
    .unwrap();

    writeln!(file, "// don't edit this{}", "\n".repeat(3)).unwrap();

    let pretty = prettyplease::unparse(&parse2(content).unwrap());
    write!(file, "{}", pretty).unwrap();
}