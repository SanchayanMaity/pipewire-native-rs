// SPDX-License-Identifier: MIT
// SPDX-FileCopyrightText: Copyright (c) 2025 Asymptotic Inc.
// SPDX-FileCopyrightText: Copyright (c) 2025 Arun Raghavan

use proc_macro::TokenStream;

mod derive_enum_u32;

#[proc_macro_derive(EnumU32)]
pub fn proc_macro_enum_u32(item: TokenStream) -> TokenStream {
    derive_enum_u32::derive_enum_u32(item)
}
