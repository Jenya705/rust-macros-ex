use proc_macro::{Delimiter, Group, Ident, Literal, Punct, Spacing, Span, TokenStream, TokenTree};
use proc_macro::Spacing::Joint;

#[proc_macro]
pub fn print_tree_macro(body: TokenStream) -> TokenStream {
    let mut tokens: Vec<TokenTree> = vec![];

    for tt in body.into_iter() {
        tokens.push(TokenTree::Ident(Ident::new("println", Span::call_site())));
        tokens.push(TokenTree::Punct(Punct::new('!', Spacing::Joint)));
        let tt_tokens = match tt {
            TokenTree::Ident(ident) => vec![
                TokenTree::Literal(
                    Literal::string("Ident: {}")
                ),
                TokenTree::Punct(Punct::new(',', Spacing::Joint)),
                TokenTree::Literal(
                    Literal::string(ident.to_string().as_str())
                ),
            ],
            TokenTree::Literal(literal) => vec![
                TokenTree::Literal(
                    Literal::string("Literal: {}")
                ),
                TokenTree::Punct(Punct::new(',', Spacing::Joint)),
                TokenTree::Literal(literal)
            ],
            TokenTree::Group(group) => vec![
                TokenTree::Literal(Literal::string("Group with delimiter: {}")),
                TokenTree::Punct(Punct::new(',', Spacing::Joint)),
                TokenTree::Literal(Literal::string(
                    format!("{:?}", group.delimiter()).as_str()
                ))
            ],
            TokenTree::Punct(punct) => vec![
                TokenTree::Literal(Literal::string("Punct: {}")),
                TokenTree::Punct(Punct::new(',', Spacing::Joint)),
                TokenTree::Literal(Literal::character(punct.as_char()))
            ],
        };
        tokens.push(TokenTree::Group(Group::new(
            Delimiter::Parenthesis,
            TokenStream::from_iter(tt_tokens.into_iter()),
        )));
        tokens.push(TokenTree::Punct(Punct::new(';', Spacing::Joint)))
    }

    TokenStream::from_iter(tokens)
}