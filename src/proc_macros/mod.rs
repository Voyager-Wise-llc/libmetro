extern crate proc_macro;
extern crate syn;
#[macro_use]
extern crate quote;

use proc_macro::TokenStream;
use syn::{parse_macro_input, DeriveInput};

#[proc_macro_derive(LookupName)]
pub fn name_macro_derive(input: TokenStream) -> TokenStream {
    // Parse the representation
    let ast = parse_macro_input!(input as DeriveInput);

    // Build the impl
    let gen = impl_name_macro(&ast);

    // Return the generated impl
    gen
}

fn impl_name_macro(ast: &syn::DeriveInput) -> TokenStream {
    let name = &ast.ident;
    let gen = quote! {
        impl<'b> crate::util::Lookup<'b, NameEntry, MetrowerksObject> for #name {
            fn get_reference(&self, index: &'b MetrowerksObject) -> Option<&'b NameEntry> {
                // not guaranteed that the Vec is in-order by id.
                index.names().iter().find(|x| x.id() == self.name_id)
            }
        }
    };
    gen.into()
}
