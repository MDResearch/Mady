use std::iter::Skip;

use proc_macro::{token_stream, Ident, TokenStream, TokenTree};

pub struct Parser<T>
where
    T: Iterator<Item = fn(Skip<token_stream::IntoIter>) -> usize>,
{
    register: Vec<fn() -> bool>,
    ast_tree: T,
}

// impl<T> Parser<T>
// where
//     T: Iterator<Item = fn(Skip<token_stream::IntoIter>) -> usize>,
// {
//     pub fn parse(&self, input: TokenStream) -> Result<T, ()> {
//         let (mut offset, max) = (0, input.into_iter().count().clone());
//         loop {
//             let mut n;
//             for f in self.ast_tree.into_iter() {
//                 n = f(input.into_iter().skip(offset));
//                 if n != 0 {
//                     offset += n;
//                     break;
//                 }
//             }

//             // return if we hit the end of the input.into_iter()
//             // that means no one match the token_stream
//             if n==0 {
//                 break Err(());
//             }

//             // all token_stream be parsed
//             if offset >= max {
//                 break Ok(self.ast_tree);
//             }
            
//         }
//     }
// }

// fn __parse(ast: &mut AstTree, input: TokenStream) {
//     input.into_iter();
//     for l in input {
//         match l {
//             TokenTree::Group(t) => {
//                 dbg!(t.delimiter(), t.stream(), t.span(), t.to_string());
//                 __parse(ast, t.stream());
//                 // t.delimiter();
//                 // t.stream();
//                 // t.span();
//                 // t.to_string();
//             }
//             TokenTree::Ident(t) => {
//                 t.span();
//                 t.to_string();
//             }
//             TokenTree::Punct(t) => {
//                 t.as_char();
//                 t.spacing();
//                 t.to_string();
//             }
//             TokenTree::Literal(t) => {
//                 t.span();
//                 t.to_string();
//             }
//         }
//     }
// }
// ```rust
// TokenStream [
//     Ident {
//         ident: "fn",
//         span: #0 bytes(153..155),
//     },
//     Ident {
//         ident: "aplusb",
//         span: #0 bytes(156..162),
//     },
//     Group {
//         delimiter: Parenthesis,
//         stream: TokenStream [
//             Ident {
//                 ident: "a",
//                 span: #0 bytes(163..164),
//             },
//             Punct {
//                 ch: ':',
//                 spacing: Alone,
//                 span: #0 bytes(164..165),
//             },
//             Ident {
//                 ident: "f64",
//                 span: #0 bytes(166..169),
//             },
//             Punct {
//                 ch: ',',
//                 spacing: Alone,
//                 span: #0 bytes(169..170),
//             },
//             Ident {
//                 ident: "b",
//                 span: #0 bytes(171..172),
//             },
//             Punct {
//                 ch: ':',
//                 spacing: Alone,
//                 span: #0 bytes(172..173),
//             },
//             Ident {
//                 ident: "f64",
//                 span: #0 bytes(174..177),
//             },
//         ],
//         span: #0 bytes(162..178),
//     },
//     Punct {
//         ch: '-',
//         spacing: Joint,
//         span: #0 bytes(179..181),
//     },
//     Punct {
//         ch: '>',
//         spacing: Alone,
//         span: #0 bytes(179..181),
//     },
//     Ident {
//         ident: "f64",
//         span: #0 bytes(182..185),
//     },
//     Group {
//         delimiter: Brace,
//         stream: TokenStream [
//             Ident {
//                 ident: "a",
//                 span: #0 bytes(196..197),
//             },
//             Punct {
//                 ch: '*',
//                 spacing: Alone,
//                 span: #0 bytes(197..198),
//             },
//             Group {
//                 delimiter: Parenthesis,
//                 stream: TokenStream [
//                     Ident {
//                         ident: "a",
//                         span: #0 bytes(199..200),
//                     },
//                     Punct {
//                         ch: '+',
//                         spacing: Alone,
//                         span: #0 bytes(201..202),
//                     },
//                     Ident {
//                         ident: "b",
//                         span: #0 bytes(203..204),
//                     },
//                 ],
//                 span: #0 bytes(198..205),
//             },
//         ],
//         span: #0 bytes(186..211),
//     },
// ]
// ```
