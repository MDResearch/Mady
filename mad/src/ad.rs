use proc_macro::TokenStream;
use syn::{parse_quote, Expr};

pub fn parse(attr: TokenStream, input: TokenStream) -> TokenStream {
    // crate::parser::parse(input);
    // TokenStream::new()
    input
}

// struct Tracker {}

// impl Tracker {
//     fn fold_expr(&mut self, e: Expr) -> Expr {
//         match e {
//             Expr::Assign(e) => {
//                 parse_quote!({
//                     e;
//                     println!("op:{:?}", #e.op);
//                     println!("right:{:?}", #e.right);
//                     println!("left:{:?}", #e.left);
//                 })
//             }
//             Expr::AssignOp(e) => {
//                 parse_quote!({
//                     e;
//                     println!("op:{:?}", #e.op);
//                     println!("right:{:?}", #e.right);
//                     println!("left:{:?}", #e.left);
//                 })
//             }
//         }
//     }
// }

// #[proc_macro_attribute]
// pub fn trace_var(args: TokenStream, input: TokenStream) -> TokenStream {
//     let input = parse_macro_input!(input as ItemFn);

//     let mut args = parse_macro_input!(args as Tracker);

//     let output = args.fold_item_fn(input);

//     TokenStream::from(quote!(#output))
// }
