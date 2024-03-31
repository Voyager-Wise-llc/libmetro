extern crate proc_macro;
extern crate syn;
#[macro_use]
extern crate quote;

use proc_macro::TokenStream;
use syn::{parse_macro_input, DeriveInput};

#[proc_macro_derive(NameIdFromObject)]
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
        impl NameIdFromObject for #name {

            fn name(&self, obj: crate::objects_m68k::MetrowerksObject) -> String {
                obj.names()[(self.name_id + 1) as usize]
                    .name()
                    .clone()
                    .into_string()
                    .unwrap()
            }
        }
    };
    gen.into()
}
