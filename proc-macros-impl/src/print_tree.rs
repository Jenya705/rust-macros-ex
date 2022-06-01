use quote::quote;
use proc_macro2::{Delimiter, TokenStream, TokenTree};

pub fn print_tree(body: TokenStream) -> TokenStream {
    print_tree_tab(body, String::new())
}

pub fn print_tree_tab(body: TokenStream, tab: String) -> TokenStream {
    let mut streams = vec![];
    for tt in body.into_iter() {
        match tt {
            TokenTree::Group(group) => {
                let header = format!(
                    "{}Group in {} {{",
                    tab,
                    match group.delimiter() {
                        Delimiter::Parenthesis => "()",
                        Delimiter::Brace => "{}",
                        Delimiter::Bracket => "[]",
                        Delimiter::None => "nothing",
                    }
                );
                let mut next_tab = tab.clone();
                next_tab.push('\t');
                let next_stream = print_tree_tab(group.stream(), next_tab);
                streams.push(quote! {
                    println!("{}", #header);
                    #next_stream;
                    println!("}}");
                })
            }
            TokenTree::Ident(ident) => {
                let header = format!(
                    "{}{} - Ident",
                    tab,
                    ident
                );
                streams.push(quote! {
                    println!("{}", #header);
                })
            }
            TokenTree::Punct(punct) => {
                let header = format!(
                    "{}{} - Punct",
                    tab,
                    punct
                );
                streams.push(quote! {
                    println!("{}", #header);
                })
            }
            TokenTree::Literal(literal) => {
                let header = format!(
                    "{}{} - Literal",
                    tab,
                    literal
                );
                streams.push(quote! {
                    println!("{}", #header);
                })
            }
        }
    }
    TokenStream::from_iter(streams)
}