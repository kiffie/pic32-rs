//! Internal implementation details of `mips-rt`.
//!
//! Do not use this crate directly.
//! Based on `cortex_m_rt_macros` (https://github.com/rust-embedded/cortex-m-rt/)

use proc_macro::TokenStream;
use proc_macro2::Span;
use quote::quote;
use syn::{
    parse,  parse_macro_input, spanned::Spanned, ItemFn, ReturnType, Type, Visibility,
};

/// Attribute to declare the entry point of the program
///
/// The specified function will be called by the reset handler *after* RAM has been initialized.
///
/// The type of the specified function must be `[unsafe] fn() -> !` (never ending function)
///
/// # Properties
///
/// The entry point will be called by the reset handler. The program can't reference to the entry
/// point, much less invoke it.
///
/// # Examples
///
/// - Simple entry point
///
/// ``` no_run
/// # #![no_main]
/// # use mips_rt_macros::entry;
/// #[entry]
/// fn main() -> ! {
///     loop {
///         /* .. */
///     }
/// }
/// ```
#[proc_macro_attribute]
pub fn entry(args: TokenStream, input: TokenStream) -> TokenStream {
    let f = parse_macro_input!(input as ItemFn);

    // check the function signature
    let valid_signature = f.sig.constness.is_none()
        && f.sig.asyncness.is_none()
        && f.vis == Visibility::Inherited
        && f.sig.abi.is_none()
        && f.sig.inputs.is_empty()
        && f.sig.generics.params.is_empty()
        && f.sig.generics.where_clause.is_none()
        && f.sig.variadic.is_none()
        && match f.sig.output {
            ReturnType::Default => false,
            ReturnType::Type(_, ref ty) => match **ty {
                Type::Never(_) => true,
                _ => false,
            },
        };

    if !valid_signature {
        return parse::Error::new(
            f.span(),
            "`#[entry]` function must have signature `[unsafe] fn() -> !`",
        )
        .to_compile_error()
        .into();
    }

    if !args.is_empty() {
        return parse::Error::new(Span::call_site(), "This attribute accepts no arguments")
            .to_compile_error()
            .into();
    }

    quote!(
        #[export_name = "main"]
        #f
    )
    .into()
}
