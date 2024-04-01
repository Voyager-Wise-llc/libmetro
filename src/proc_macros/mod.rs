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
        impl<'a> NameIdFromObject<'a> for #name {
            fn name(&'a self, obj: &'a crate::objects_m68k::MetrowerksObject) -> &str {
                obj.names()[(self.name_id + 1) as usize].name().as_str()
            }
        }
    };
    gen.into()
}
