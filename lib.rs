// Copyright 2016 Torbjørn Birch Moltu.
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

//! Extra marker traits for iterators.
//!
//! # Feature flags:
//! * **unstable**: Implement for `Range` and `RangeInclusive`.
//! * **no_std**: Use `#[no_std]` and don't implement for the map and set iterators in std.

#![cfg_attr(feature="unstable", feature(inclusive_range, step_trait))]
#![cfg_attr(feature="no_std", no_std)]

#[cfg(not(feature="no_std"))]
extern crate core;
use core::iter::*;

/// Marker trait for iterators that will never return two equal items, like a set.
///
/// The trait is unsafe so consumers can rely on it.
pub unsafe trait UniqueIterator: Iterator where Self::Item: PartialEq
    {}

/// Marker trait for iterators that return items in a sorted ascending order.
/// It does not guarantee uniqueness, but equal items must come straight after each other.
/// Use `I: UniqueIterator+AscendingIterator` if you need both.
///
/// The trait is unsafe so consumers can rely on it.  
/// It is a logic error to implement both `AscendingIterator` and `DescendingIterator` for the same type.
///
/// # Examples:
/// ```
/// let _ = (1..10);
/// ```
pub unsafe trait AscendingIterator: Iterator where Self::Item: PartialOrd
    {}

/// Marker trait for iterators that return items in a sorted desscending order.
/// It does not guarantee uniqueness, but equal items must come straight after each other.
/// Use `I: UniqueIterator+DescendingIterator` if you need both.
///
/// The trait is unsafe so consumers can rely on it.  
/// It is a logic error to implement both `AscendingIterator` and `DescendingIterator` for the same type.
///
/// # Examples:
/// ```
/// let _ = (1..10).rev();
/// ```
pub unsafe trait DescendingIterator: Iterator where Self::Item: PartialOrd
    {}


unsafe impl<T:PartialEq, I:Iterator<Item=T>> UniqueIterator for Enumerate<I> {}
unsafe impl<T:PartialOrd, I:Iterator<Item=T>> AscendingIterator for Enumerate<I> {}

unsafe impl<T:PartialEq> UniqueIterator for Once<T> {}
unsafe impl<T:PartialOrd> AscendingIterator for Once<T> {}
unsafe impl<T:PartialOrd> DescendingIterator for Once<T> {}

unsafe impl<T:PartialEq> UniqueIterator for Empty<T> {}
unsafe impl<T:PartialOrd> AscendingIterator for Empty<T> {}
unsafe impl<T:PartialOrd> DescendingIterator for Empty<T> {}


#[cfg(feature="unstable")]
mod unstable {
    use super::{UniqueIterator,AscendingIterator};
    use core::ops::{Range,RangeInclusive, Add};
    use core::iter::Step;// is unstable and must be specified.

    /// Requires the feature `unstable`.
    unsafe impl<T:PartialEq+Step> UniqueIterator for Range<T> where for<'a> &'a T: Add<&'a T, Output=T> {}
    /// Requires the feature `unstable`.
    unsafe impl<T:PartialOrd+Step> AscendingIterator for Range<T> where for<'a> &'a T: Add<&'a T, Output=T> {}
    /// Requires the feature `unstable`.
    unsafe impl<T:PartialEq+Step> UniqueIterator for RangeInclusive<T> where for<'a> &'a T: Add<&'a T, Output=T> {}
    /// Requires the feature `unstable`.
    unsafe impl<T:PartialOrd+Step> AscendingIterator for RangeInclusive<T> where for<'a> &'a T: Add<&'a T, Output=T> {}
    // RangeTo and RangeInclusiveTo doesn't implement iterator!
    // RangeFrom and RangeFull wraps around in release mode.
}
#[cfg(feature="unstable")]
pub use unstable::*;


#[cfg(not(feature="no_std"))]
mod collections {
    use super::{UniqueIterator,AscendingIterator};
    use std::collections::{hash_map, btree_map, hash_set, btree_set};
    use std::hash::{Hash, BuildHasher};

    unsafe impl<'a, K:Eq, V> UniqueIterator for hash_map::Keys<'a,K,V> {}
    unsafe impl<'a, K:Eq, V:PartialEq> UniqueIterator for hash_map::Iter<'a,K,V> {}
    unsafe impl<'a, K:Eq, V:PartialEq> UniqueIterator for hash_map::IterMut<'a,K,V> {}
    unsafe impl<K:Eq, V:PartialEq> UniqueIterator for hash_map::IntoIter<K,V> {}

    unsafe impl<'a, K:Ord, V> UniqueIterator for btree_map::Keys<'a,K,V> {}
    unsafe impl<'a, K:Ord, V:PartialEq> UniqueIterator for btree_map::Iter<'a,K,V> {}
    unsafe impl<'a, K:Ord, V:PartialEq> UniqueIterator for btree_map::IterMut<'a,K,V> {}
    unsafe impl<K:Ord, V:PartialEq> UniqueIterator for btree_map::IntoIter<K,V> {}
    unsafe impl<'a, K:Ord, V> AscendingIterator for btree_map::Keys<'a,K,V> {}
    unsafe impl<'a, K:Ord, V:PartialOrd> AscendingIterator for btree_map::Iter<'a,K,V> {}
    unsafe impl<'a, K:Ord, V:PartialOrd> AscendingIterator for btree_map::IterMut<'a,K,V> {}
    unsafe impl<K:Ord, V:PartialOrd> AscendingIterator for btree_map::IntoIter<K,V> {}

    unsafe impl<T:Eq> UniqueIterator for hash_set::IntoIter<T> {}
    unsafe impl<'a, T:Eq+Hash> UniqueIterator for hash_set::Iter<'a,T> {}
    unsafe impl<'a, T:Eq+Hash, S:BuildHasher> UniqueIterator for hash_set::Union<'a,T,S> {}
    unsafe impl<'a, T:Eq+Hash, S:BuildHasher> UniqueIterator for hash_set::Intersection<'a,T,S> {}
    unsafe impl<'a, T:Eq+Hash, S:BuildHasher> UniqueIterator for hash_set::Difference<'a,T,S> {}
    unsafe impl<'a, T:Eq+Hash, S:BuildHasher> UniqueIterator for hash_set::SymmetricDifference<'a,T,S> {}

    unsafe impl<T:Ord> UniqueIterator for btree_set::IntoIter<T> {}
    unsafe impl<'a, T:Ord> UniqueIterator for btree_set::Iter<'a,T> {}
    unsafe impl<'a, T:Ord> UniqueIterator for btree_set::Intersection<'a,T> {}
    unsafe impl<'a, T:Ord> UniqueIterator for btree_set::Union<'a,T> {}
    unsafe impl<'a, T:Ord> UniqueIterator for btree_set::Difference<'a,T> {}
    unsafe impl<'a, T:Ord> UniqueIterator for btree_set::SymmetricDifference<'a,T> {}
    unsafe impl<T:Ord> AscendingIterator for btree_set::IntoIter<T> {}
    unsafe impl<'a, T:Ord> AscendingIterator for btree_set::Iter<'a,T> {}
    unsafe impl<'a, T:Ord> AscendingIterator for btree_set::Intersection<'a,T> {}
    // Are the others sorted?
}
#[cfg(not(feature="no_std"))]
pub use collections::*;


// Iterator adaptors that maintain guarantees:

macro_rules! it {($i:item) => {$i}}// workaround for issue #5846 fixed in nightly
macro_rules! simple_iter {($bound:tt, $marker:tt: $($typ:tt)*) => {
    $(it!{unsafe impl<T:$bound, I:Iterator<Item=T>+$marker> $marker for $typ<I> {}})*
}}
macro_rules! filter_iter {($bound:tt, $marker:tt: $($typ:tt)*) => {
    $(it!{unsafe impl<T:$bound, I:Iterator<Item=T>+$marker, F:FnMut(&T)->bool> $marker for $typ<I,F> {}})*
}}
simple_iter!{PartialEq, UniqueIterator: Peekable Skip Take Fuse}
simple_iter!{PartialOrd, AscendingIterator: Peekable Skip Take Fuse}
simple_iter!{PartialOrd, DescendingIterator: Peekable Skip Take Fuse}
filter_iter!{PartialEq, UniqueIterator: Filter SkipWhile TakeWhile}
filter_iter!{PartialOrd, AscendingIterator: Filter SkipWhile TakeWhile}
filter_iter!{PartialOrd, DescendingIterator: Filter SkipWhile TakeWhile}

unsafe impl<I:DoubleEndedIterator+UniqueIterator> UniqueIterator for Rev<I> where I::Item: PartialOrd {}
// Note the swap
unsafe impl<I:DoubleEndedIterator+AscendingIterator> DescendingIterator for Rev<I> where I::Item: PartialOrd {}
unsafe impl<I:DoubleEndedIterator+DescendingIterator> AscendingIterator for Rev<I> where I::Item: PartialOrd {}

unsafe impl<'a, T:'a+Clone+PartialEq, I:Iterator<Item=&'a T>+UniqueIterator> UniqueIterator for Cloned<I> {}
unsafe impl<'a, T:'a+Clone+PartialOrd, I:Iterator<Item=&'a T>+AscendingIterator> AscendingIterator for Cloned<I> {}
unsafe impl<'a, T:'a+Clone+PartialOrd, I:Iterator<Item=&'a T>+DescendingIterator> DescendingIterator for Cloned<I> {}

unsafe impl<T:PartialEq, I:Iterator<Item=T>+UniqueIterator, F:FnMut(&T)> UniqueIterator for Inspect<I,F> {}
unsafe impl<T:PartialOrd, I:Iterator<Item=T>+AscendingIterator, F:FnMut(&T)> AscendingIterator for Inspect<I,F> {}
unsafe impl<T:PartialOrd, I:Iterator<Item=T>+DescendingIterator, F:FnMut(&T)> DescendingIterator for Inspect<I,F> {}

// Implementing for `IB:UniqueIterator` creates a conflict; just swap the order.
unsafe impl<TA,TB,IA,IB> UniqueIterator for Zip<IA,IB>
    where TA:PartialEq, IA:Iterator<Item=TA>+UniqueIterator,
          TB:PartialEq, IB:Iterator<Item=TB>
    {}
// Cannot also implement for where both are sorted, I think sorted and not unique will be uncommon.
unsafe impl<TA,TB,IA,IB> AscendingIterator for Zip<IA,IB>
    where TA:PartialOrd, IA:Iterator<Item=TA>+AscendingIterator+UniqueIterator,
          TB:PartialOrd, IB:Iterator<Item=TB>
    {}
unsafe impl<TA,TB,IA,IB> DescendingIterator for Zip<IA,IB>
    where TA:PartialOrd, IA:Iterator<Item=TA>+DescendingIterator+UniqueIterator,
          TB:PartialOrd, IB:Iterator<Item=TB>
    {}
