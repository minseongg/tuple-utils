//! Push an element to the tuple.

use proc_macro2::{Span, TokenStream};
use quote::quote;
use syn::{Ident, LitInt};

/// Returns generated code for `PushFront` trait.
pub fn gen_push_front() -> TokenStream {
    let trait_def = quote! {
        pub trait PushFront {
            type Out<T>;

            /// Returns the tuple after pushing.
            fn push_front<T>(self, elt: T) -> Self::Out<T>;
        }
    };

    let mut trait_impls = quote! {
        impl PushFront for () {
            type Out<T> = (T,);

            fn push_front<T>(self, elt: T) -> Self::Out<T> {
                (elt,)
            }
        }
    };

    for i in 0usize..32usize {
        let impl_generics = (0..=i).map(|j| Ident::new(&format!("T{j}"), Span::call_site()));
        let ty_generics = (0..=i).map(|j| Ident::new(&format!("T{j}"), Span::call_site()));
        let ty_out = ::std::iter::empty()
            .chain([Ident::new(&"T".to_string(), Span::call_site())].into_iter())
            .chain(
                (0..=i)
                    .map(|j| Ident::new(&format!("T{j}"), Span::call_site()))
                    .into_iter(),
            );
        let out = ::std::iter::empty()
            .chain([quote! { elt }].into_iter())
            .chain(
                (0..=i)
                    .map(|j| LitInt::new(&j.to_string(), Span::call_site()))
                    .map(|j| quote! { self.#j })
                    .into_iter(),
            );

        let trait_impl = quote! {
            impl<#(#impl_generics,)*> PushFront for (#(#ty_generics,)*) {
                type Out<T> = (#(#ty_out,)*);

                fn push_front<T>(self, elt: T) -> Self::Out<T> {
                    (#(#out,)*)
                }
            }
        };

        trait_impls.extend(trait_impl);
    }

    let mut out = trait_def;
    out.extend(trait_impls);

    out
}

/// Returns generated code for `PushBack` trait.
pub fn gen_push_back() -> TokenStream {
    let trait_def = quote! {
        pub trait PushBack {
            type Out<T>;

            /// Returns the tuple after pushing.
            fn push_back<T>(self, elt: T) -> Self::Out<T>;
        }
    };

    let mut trait_impls = quote! {
        impl PushBack for () {
            type Out<T> = (T,);

            fn push_back<T>(self, elt: T) -> Self::Out<T> {
                (elt,)
            }
        }
    };

    for i in 0usize..32usize {
        let impl_generics = (0..=i).map(|j| Ident::new(&format!("T{j}"), Span::call_site()));
        let ty_generics = (0..=i).map(|j| Ident::new(&format!("T{j}"), Span::call_site()));
        let ty_out = ::std::iter::empty()
            .chain(
                (0..=i)
                    .map(|j| Ident::new(&format!("T{j}"), Span::call_site()))
                    .into_iter(),
            )
            .chain([Ident::new(&"T".to_string(), Span::call_site())].into_iter());
        let out = ::std::iter::empty()
            .chain(
                (0..=i)
                    .map(|j| LitInt::new(&j.to_string(), Span::call_site()))
                    .map(|j| quote! { self.#j })
                    .into_iter(),
            )
            .chain([quote! { elt }].into_iter());

        let trait_impl = quote! {
            impl<#(#impl_generics,)*> PushBack for (#(#ty_generics,)*) {
                type Out<T> = (#(#ty_out,)*);

                fn push_back<T>(self, elt: T) -> Self::Out<T> {
                    (#(#out,)*)
                }
            }
        };

        trait_impls.extend(trait_impl);
    }

    let mut out = trait_def;
    out.extend(trait_impls);

    out
}
