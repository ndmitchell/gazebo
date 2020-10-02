/*
 * Copyright (c) Facebook, Inc. and its affiliates.
 *
 * This source code is licensed under both the MIT license found in the
 * LICENSE-MIT file in the root directory of this source tree and the Apache
 * License, Version 2.0 found in the LICENSE-APACHE file in the root directory
 * of this source tree.
 */

#![deny(intra_doc_link_resolution_failure)]

mod any_lifetime;
mod clone;
mod copy;
mod default;
mod dupe;
mod maybe_eq;
mod util;
mod variant;

extern crate proc_macro;

/// Derive the [`Dupe` trait](Dupe).
#[proc_macro_derive(Dupe)]
pub fn derive_dupe(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
    dupe::derive_dupe(input)
}

/// Derive the [`Dupe` trait](Dupe), but without requiring all type arguments to implement [`Dupe`](Dupe).
#[proc_macro_derive(Dupe_)]
pub fn derive_dupe_(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
    dupe::derive_dupe_(input)
}

/// Derive the [`Clone` trait](Clone), but without requiring all type arguments to implement [`Clone`](Clone).
#[proc_macro_derive(Clone_)]
pub fn derive_clone_(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
    clone::derive_clone_(input)
}

/// Derive the [`Copy` trait](Copy), but without requiring all type arguments to implement [`Copy`](Copy).
#[proc_macro_derive(Copy_)]
pub fn derive_copy_(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
    copy::derive_copy_(input)
}

/// Derive the [`Default` trait](Default), but without requiring all type arguments to implement [`Default`](Default).
#[proc_macro_derive(Default_)]
pub fn derive_default_(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
    default::derive_default_(input)
}

/// Derive the [`AnyLifetime` trait](AnyLifetime). Requires the type has no type arguments, no constant arguments,
/// and at most one lifetime argument.
#[proc_macro_derive(AnyLifetime)]
pub fn derive_any_lifetime(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
    any_lifetime::derive_any_lifetime(input)
}

#[proc_macro_derive(MaybeEq)]
pub fn derive_maybe_eq(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
    maybe_eq::derive_maybe_eq(input, true)
}

#[proc_macro_derive(MaybeEq_Never)]
pub fn derive_not_maybe_eq(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
    maybe_eq::derive_maybe_eq(input, false)
}

#[proc_macro_derive(VariantName)]
pub fn derive_variant_names(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
    variant::derive_variant_names(input)
}