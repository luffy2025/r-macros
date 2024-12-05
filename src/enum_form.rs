use proc_macro2::TokenStream;
use quote::quote;
use syn::DeriveInput;

pub fn process_enum_from(input: DeriveInput) -> TokenStream {
    // get the ident
    let ident = input.ident;
    // get enum variants
    let variants = match input.data {
        syn::Data::Enum(data) => data.variants,
        _ => panic!("EnumFrom only works with enums"),
    };

    // for each variant, generate a From impl
    let from_impls = variants.iter().map(|variant| {
        let var = &variant.ident;
        let gen = &input.generics;
        match &variant.fields {
            syn::Fields::Unnamed(fields) => {
                if fields.unnamed.len() != 1 {
                    quote! {}
                } else {
                    let field = &fields.unnamed.first().expect("should have 1 field");
                    let ty = &field.ty;
                    quote! {
                        impl #gen From<#ty> for #ident #gen {
                            fn from(v: #ty) -> Self {
                                #ident::#var(v)
                            }
                        }
                    }
                }
            }
            _ => quote! {},
        }
    });

    quote! {
        #(#from_impls)*
    }
}
