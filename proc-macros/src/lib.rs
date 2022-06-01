use quote::quote;
use proc_macro2::{Spacing, TokenStream, TokenTree};
use proc_macros_impl::print_tree::print_tree;

#[proc_macro]
pub fn print_tree_macro(body: proc_macro::TokenStream) -> proc_macro::TokenStream {
    print_tree(body.into()).into()
}