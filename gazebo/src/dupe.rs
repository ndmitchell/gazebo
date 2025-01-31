/*
 * Copyright (c) Facebook, Inc. and its affiliates.
 *
 * This source code is licensed under both the MIT license found in the
 * LICENSE-MIT file in the root directory of this source tree and the Apache
 * License, Version 2.0 found in the LICENSE-APACHE file in the root directory
 * of this source tree.
 */

//! A cheap version of [`Clone`](Clone).

pub use gazebo_derive::{Dupe, Dupe_};
use std::{cell::Cell, rc::Rc, sync::Arc};

/// Like [`Clone`](Clone), but should only be available if [`Clone`](Clone) is
/// constant time and zero allocation (e.g. a few [`Arc`](Arc) bumps)
pub trait Dupe: Clone {
    fn dupe(&self) -> Self {
        self.clone()
    }
}

// Smart pointer/wrapper types
impl<A: ?Sized> Dupe for Arc<A> {}
impl<A: ?Sized> Dupe for Rc<A> {}
impl<A: Copy> Dupe for Cell<A> {}

// Small containers
impl<A: Dupe> Dupe for Option<A> {}
impl<T: Dupe, E: Dupe> Dupe for Result<T, E> {}
impl<A: Dupe> Dupe for (A,) {}
// Not clear if Dupe should be implemented for pairs or not.
// Concern is deeply nested pairs could be exponentially more expensive than their inner dupes.

// Atomic types
impl Dupe for () {}
impl Dupe for bool {}
impl Dupe for char {}
impl Dupe for u8 {}
impl Dupe for u16 {}
impl Dupe for u32 {}
impl Dupe for u64 {}
impl Dupe for u128 {}
impl Dupe for usize {}
impl Dupe for i8 {}
impl Dupe for i16 {}
impl Dupe for i32 {}
impl Dupe for i64 {}
impl Dupe for i128 {}
impl Dupe for isize {}
impl Dupe for f32 {}
impl Dupe for f64 {}

// Other std types that are Copyable
impl Dupe for std::time::Instant {}
impl<T: ?Sized> Dupe for std::marker::PhantomData<T> {}

#[cfg(test)]
mod tests {
    use super::*;
    #[allow(unused_imports)] // Not actually unused, this makes testing the derive macro work
    use crate as gazebo;
    use gazebo_derive::Clone_;

    #[test]
    fn test_dupe_generic() {
        #[derive(Clone, Dupe, Debug, PartialEq, Eq)]
        struct Foo {}
        #[derive(Clone, Dupe, Debug, PartialEq, Eq)]
        struct FooT<T> {
            foo: T,
        }
        #[derive(Clone, Dupe, Debug, PartialEq, Eq)]
        struct Faz;

        let x = Foo {};
        assert_eq!(x, x.dupe());

        let x = FooT { foo: 1 };
        assert_eq!(x, x.dupe());

        let x = Faz;
        assert_eq!(x, x.dupe());
    }

    #[test]
    fn test_dupe_() {
        #[derive(Debug, PartialEq, Eq)]
        struct NoClone();
        #[derive(Clone_, Dupe_, Debug, PartialEq, Eq)]
        struct FooT<T> {
            foo: Arc<T>,
        }

        let x = FooT {
            foo: Arc::new(NoClone()),
        };
        assert_eq!(x, x.dupe());
    }

    #[test]
    fn test_dupe_enum() {
        #[derive(Clone, Dupe, Debug, PartialEq, Eq)]
        struct Foo();
        #[derive(Clone, Dupe, Debug, PartialEq, Eq)]
        struct Foo2;
        #[derive(Clone, Dupe, Debug, PartialEq, Eq)]
        struct Bar(i64, bool, Foo2);
        #[derive(Clone, Dupe, Debug, PartialEq, Eq)]
        struct Baz {
            foo: usize,
        }
        #[derive(Clone, Dupe, Debug, PartialEq, Eq)]
        enum Qux {
            Foo(),
            Foo2,
            Bar(Foo, Bar),
            Baz { foo: Foo, bar: Bar, baz: Baz },
        }

        let x = Qux::Bar(Foo(), Bar(8, true, Foo2));
        assert_eq!(x, x.dupe());
        let x = Qux::Baz {
            foo: Foo(),
            bar: Bar(7, false, Foo2),
            baz: Baz { foo: 9 },
        };
        assert_eq!(x, x.dupe());
    }
}
