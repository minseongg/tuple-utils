//! Arity of a tuple.

use proc_macro2::{Span, TokenStream};
use quote::quote;
use syn::Ident;

/// Returns generated code for `Arity` trait.
pub fn gen_arity() -> TokenStream {
    let trait_def = quote! {
        pub trait Arity {
            /// Returns arity of a tuple.
            fn arity() -> usize;
        }
    };

    let mut trait_impls = quote! {
        impl Arity for () {
            fn arity() -> usize {
                0
            }
        }
    };

    for i in 0usize..32usize {
        let impl_generics = (0..=i).map(|j| Ident::new(&format!("T{j}"), Span::call_site()));
        let ty_generics = (0..=i).map(|j| Ident::new(&format!("T{j}"), Span::call_site()));
        let arity = i + 1;

        let trait_impl = quote! {
            impl<#(#impl_generics,)*> Arity for (#(#ty_generics,)*) {
                fn arity() -> usize {
                    #arity
                }
            }
        };

        trait_impls.extend(trait_impl);
    }

    let mut out = trait_def;
    out.extend(trait_impls);

    out
}
