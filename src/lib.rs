#![cfg_attr(feature = "nightly",
    feature(external_doc),
    doc(include = "../README.md"),
)]
//!

extern crate proc_macro;

use ::proc_macro::{*,
    TokenTree as TT,
};

#[proc_macro] pub
fn bstringify (input: TokenStream)
  -> TokenStream
{
    TT::from(Literal::byte_string(input.to_string().as_bytes()))
        .into()
}
