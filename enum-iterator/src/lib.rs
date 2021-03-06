// Copyright (C) 2018 Stephane Raux. Distributed under the MIT license.

//! Tools to iterate over the variants of a field-less enum.

#![deny(missing_docs)]
#![deny(warnings)]

#[allow(unused_imports)]
#[macro_use]
extern crate enum_iterator_derive;

pub use enum_iterator_derive::*;

use std::iter;

/// Trait to iterate over the variants of a field-less enum.
///
/// Field-less (a.k.a. C-like) enums are enums whose variants don't have
/// additional data.
///
/// When deriving this trait for an enum named `Foo`, the associated type
/// `Iterator` is a generated type named `FooEnumIterator`. This generated
/// type has the same visibility as `Foo`. Variants are yielded in the order
/// they are defined in the enum.
///
/// # Example
///
/// ```
/// #[macro_use]
/// extern crate enum_iterator;
///
/// use enum_iterator::IntoEnumIterator;
///
/// #[derive(Clone, IntoEnumIterator, PartialEq)]
/// enum Direction {North, South, West, East}
///
/// fn main() {
///     assert!(Direction::into_enum_iter().eq([Direction::North,
///         Direction::South, Direction::West, Direction::East].iter()
///         .cloned()));
/// }
/// ```
pub trait IntoEnumIterator: Sized {
    /// Type of the iterator over the variants.
    type Iterator: Iterator<Item = Self> + iter::ExactSizeIterator
        + iter::FusedIterator;

    /// Returns an iterator over the variants.
    fn into_enum_iter() -> Self::Iterator;
}
