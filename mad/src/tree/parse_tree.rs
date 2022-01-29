use proc_macro::TokenStream;

use quote::quote;
use syn::{parse2, Expr,parse_macro_input};

struct Tree {}

impl Tree {
    fn parse()->Self {
        Self{}
    }
}

#[cfg(test)]
mod tests {
    use syn::ItemFn;

    use super::*;
     
    #[test]
    fn test_1() {
        let s = quote! {
            a*b+c*d
        };
        
        let ast: Expr = parse2(s).expect("unknow tokenstream");
        dbg!(ast);
    }
}

// ast = Binary(
//     ExprBinary {
//         attrs: [],
//         left: Path(
//             ExprPath {
//                 attrs: [],
//                 qself: None,
//                 path: Path {
//                     leading_colon: None,
//                     segments: [
//                         PathSegment {
//                             ident: Ident(
//                                 a,
//                             ),
//                             arguments: None,
//                         },
//                     ],
//                 },
//             },
//         ),
//         op: Add(
//             Add,
//         ),
//         right: Binary(
//             ExprBinary {
//                 attrs: [],
//                 left: Path(
//                     ExprPath {
//                         attrs: [],
//                         qself: None,
//                         path: Path {
//                             leading_colon: None,
//                             segments: [
//                                 PathSegment {
//                                     ident: Ident(
//                                         b,
//                                     ),
//                                     arguments: None,
//                                 },
//                             ],
//                         },
//                     },
//                 ),
//                 op: Mul(
//                     Star,
//                 ),
//                 right: Path(
//                     ExprPath {
//                         attrs: [],
//                         qself: None,
//                         path: Path {
//                             leading_colon: None,
//                             segments: [
//                                 PathSegment {
//                                     ident: Ident(
//                                         c,
//                                     ),
//                                     arguments: None,
//                                 },
//                             ],
//                         },
//                     },
//                 ),
//             },
//         ),
//     },
// )
