use darling::ast::Data;
use darling::{FromDeriveInput, FromField};
use proc_macro2::TokenStream;
use quote::quote;
use syn::DeriveInput;

#[derive(Debug, FromDeriveInput)]
struct AutoDebugInfo {
    ident: syn::Ident,
    generics: syn::Generics,
    data: Data<(), AutoDerefFieldsInfo>,
}

#[allow(unused)]
#[derive(Debug, FromField)]
#[darling(attributes(debug))]
struct AutoDerefFieldsInfo {
    ident: Option<syn::Ident>,
    ty: syn::Type,
    #[darling(default)]
    skip: bool,
}

#[allow(unused)]
pub(crate) fn process_auto_debug(input: DeriveInput) -> TokenStream {
    let AutoDebugInfo {
        ident,
        generics,
        data: Data::Struct(fields),
    } = AutoDebugInfo::from_derive_input(&input).expect("can not parse input")
    else {
        panic!("AutoDebug only works on structs");
    };

    let fields = fields.iter().map(|field| {
        let ident = field.ident.as_ref().unwrap();
        if field.skip {
            quote! {}
        } else {
            quote! {
                .field(stringify!(#ident), &self.#ident)
            }
        }
    });

    quote! {
        impl ::core::fmt::Debug for #ident #generics {
            #[inline]
            fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
                f.debug_struct(stringify!(#ident))
                    #(#fields)*
                    .finish()
            }
        }
    }
}
