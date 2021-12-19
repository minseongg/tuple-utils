//! Split a tuple into two tuples.

use proc_macro2::{Span, TokenStream};
use quote::quote;
use syn::{Ident, LitInt};

/// Returns generated code for `SplitFirst` trait.
pub fn gen_split_first() -> TokenStream {
    let trait_def = quote! {
        pub trait SplitFirst {
            type Left;
            type Right;

            /// Returns splitted tuples.
            fn split_first(self) -> (Self::Left, Self::Right);
        }
    };

    let mut trait_impls = quote! {};

    for i in 0usize..32usize {
        let impl_generics = (0..=i).map(|j| Ident::new(&format!("T{j}"), Span::call_site()));
        let ty_generics = (0..=i).map(|j| Ident::new(&format!("T{j}"), Span::call_site()));
        let ty_left = Ident::new(&"T0".to_string(), Span::call_site());
        let ty_right = (1..=i).map(|j| Ident::new(&format!("T{j}"), Span::call_site()));
        let right = (1..=i).map(|j| LitInt::new(&j.to_string(), Span::call_site())).map(|j| quote! { self.#j });

        let trait_impl = quote! {
            impl<#(#impl_generics,)*> SplitFirst for (#(#ty_generics,)*) {
                type Left = #ty_left;
                type Right = (#(#ty_right,)*);

                fn split_first(self) -> (Self::Left, Self::Right) {
                    (
                        self.0,
                        (#(#right,)*)
                    )
                }
            }
        };

        trait_impls.extend(trait_impl);
    }

    let mut out = trait_def;
    out.extend(trait_impls);

    out
}

/// Returns generated code for `SplitLast` trait.
pub fn gen_split_last() -> TokenStream {
    let trait_def = quote! {
        pub trait SplitLast {
            type Left;
            type Right;

            /// Returns splitted tuples.
            fn split_last(self) -> (Self::Left, Self::Right);
        }
    };

    let mut trait_impls = quote! {};

    for i in 0usize..32usize {
        let impl_generics = (0..=i).map(|j| Ident::new(&format!("T{j}"), Span::call_site()));
        let ty_generics = (0..=i).map(|j| Ident::new(&format!("T{j}"), Span::call_site()));
        let ty_left = (0..i).map(|j| Ident::new(&format!("T{j}"), Span::call_site()));
        let ty_right = Ident::new(&format!("T{i}"), Span::call_site());
        let left = (0..i).map(|j| LitInt::new(&j.to_string(), Span::call_site())).map(|j| quote! { self.#j });
        let right = {
            let j = LitInt::new(&i.to_string(), Span::call_site());
            quote! { self.#j }
        };

        let trait_impl = quote! {
            impl<#(#impl_generics,)*> SplitLast for (#(#ty_generics,)*) {
                type Left = (#(#ty_left,)*);
                type Right = #ty_right;

                fn split_last(self) -> (Self::Left, Self::Right) {
                    (
                        (#(#left,)*),
                        #right,
                    )
                }
            }
        };

        trait_impls.extend(trait_impl);
    }

    let mut out = trait_def;
    out.extend(trait_impls);

    out
}
