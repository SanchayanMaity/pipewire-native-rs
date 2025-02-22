// SPDX-License-Identifier: MIT
// SPDX-FileCopyrightText: Copyright (c) 2025 Asymptotic Inc.
// SPDX-FileCopyrightText: Copyright (c) 2025 Arun Raghavan

use super::types::Type;

pub fn primitive_pod_size(t: Type) -> (usize, usize) {
    match t {
        Type::None => (0, 0),
        Type::Bool => (4, 4),
        Type::Id => (4, 4),
        Type::Int => (4, 4),
        Type::Long => (8, 0),
        Type::Float => (4, 4),
        Type::Double => (8, 0),
        Type::Fd => (8, 0),
        Type::Rectangle => (8, 0),
        Type::Fraction => (8, 0),
        _ => unreachable!(),
    }
}
