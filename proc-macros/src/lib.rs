use quote::quote;
use proc_macro2::{Spacing, TokenStream, TokenTree};

#[proc_macro]
pub fn print_tree_macro(body: proc_macro::TokenStream) -> proc_macro::TokenStream {
    let body = proc_macro2::TokenStream::from(body);
    let mut streams = vec![];

    for tt in body.into_iter() {
        match tt {
            TokenTree::Literal(literal) => {
                let literal_string = literal.to_string();
                streams.push(quote! {
                    println!("Literal - {}", #literal_string);
                })
            },
            TokenTree::Punct(punct) => {
                let punct_char = punct.as_char();
                let punct_spacing = match punct.spacing() {
                    Spacing::Joint => "Joint",
                    Spacing::Alone => "Alone",
                };
                streams.push(quote! {
                    println!("Punct - '{}'; {}", #punct_char, #punct_spacing);
                })
            },
            TokenTree::Ident(ident) => {
                let ident_string = ident.to_string();
                streams.push(quote! {
                    println!("Ident - {}", #ident_string);
                })
            },
            TokenTree::Group(group) => {
                let group_delimiter = format!("{:?}", group.delimiter());
                let group_stream = TokenStream::from(
                    print_tree_macro(proc_macro::TokenStream::from(group.stream()))
                );
                streams.push(quote! {
                    println!("Group - {}", #group_delimiter);
                    #group_stream
                    println!("End.");
                })
            }
        }
    }

    proc_macro::TokenStream::from(TokenStream::from_iter(streams))
}