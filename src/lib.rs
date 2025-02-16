//! A `no_std` and no `alloc` [`Cow`](https://doc.rust-lang.org/alloc/borrow/enum.Cow.html) (copy-on-write) implementation.
//! 
//! This crate provides [`NoStdCow`] which is extremely similar to [`Cow`](https://doc.rust-lang.org/alloc/borrow/enum.Cow.html)
//! except that it doesn't rely on the `ToOwned` trait from `alloc` and uses [`Clone`] instead.
//! 
//! Use [`Cow`](https://doc.rust-lang.org/alloc/borrow/enum.Cow.html) from std/alloc instead if you have access to them.
//! This library is useful for when you want [`Cow`](https://doc.rust-lang.org/alloc/borrow/enum.Cow.html) but don't have
//! `alloc` or `std`.
//! # Example
//! ```
//! use nostd_cow::NoStdCow;
//! 
//! /// Convert a string to uppercase if it isn't already uppercase, otherwise
//! /// just return a reference to the original source.
//! fn to_uppercase<'a>(source: &'a str) -> NoStdCow<'a, String, str> {
//!     for c in source.chars() {
//!         if !c.is_uppercase() {
//!             return NoStdCow::Owned(source.to_uppercase());
//!         }
//!     }
//!     NoStdCow::Borrowed(source)
//! }
//! // This string is already uppercase, so the function will not allocate a new [`String`].
//! let already_uppercase = "HELLOWORLD";
//! assert_eq!(to_uppercase(already_uppercase), NoStdCow::Borrowed(already_uppercase));
//! // This string needs to be converted to uppercase, so a new owned value is constructed
//! // and returned.
//! let not_uppercase = "helloworld";
//! assert_eq!(to_uppercase(not_uppercase), NoStdCow::Owned(String::from("HELLOWORLD")));
//! ```
#![no_std]
#![cfg_attr(docsrs, feature(doc_auto_cfg))]
#![cfg_attr(not(docsrs), allow(rustdoc::broken_intra_doc_links))]



#[cfg(feature = "alloc")]
#[doc(hidden)]
mod alloc_impls;

use core::{borrow::Borrow, ops::Deref};

/// A type alias of [`NoStdCow`] that can either store `T` or `&T`. If `T` is [`Clone`],
/// `to_mut` and `into_owned` will be available.
pub type RefCow<'a, T> = NoStdCow<'a, T, T>;

/// A `no_std` clone-on-write smart pointer.
/// 
/// The type [`NoStdCow`] is no std and no alloc
/// version of [`std::borrow::Cow`](https://doc.rust-lang.org/std/borrow/enum.Cow.html).
/// It performs the same row except that [`Clone`] is relied on instead of `ToOwned`,
/// which requires `alloc`. The type signature doesn't indicate this, but the generic
/// parameter `T` should preferably be [`Clone`]. For most use cases you should use
/// the [`RefCow`] type alias instead.
/// 
/// [`NoStdCow`] implements [`Deref`] just like `Cow`, so you can call non-mutating
/// methods directly on the data it encloses. If mutation is desired, `to_mut` will
/// obtain a mutable reference to an owned value, cloning if necessary.
/// 
/// You should consider just using [`alloc::borrow::Cow`](https://doc.rust-lang.org/alloc/borrow/enum.Cow.html)
/// from `alloc` if you have access to an allocator. [`NoStdCow::into_alloc_cow`] and
/// [`NoStdCow::from_alloc_cow`]. [`From`] implementations are also available in
/// both directions.
/// 
/// # Example
/// ```
/// use nostd_cow::NoStdCow;
/// 
/// /// Convert a string to uppercase if it isn't already uppercase, otherwise
/// /// just return a reference to the original source.
/// fn to_uppercase<'a>(source: &'a str) -> NoStdCow<'a, String, str> {
///     for c in source.chars() {
///         if !c.is_uppercase() {
///             return NoStdCow::Owned(source.to_uppercase());
///         }
///     }
///     NoStdCow::Borrowed(source)
/// }
/// // This string is already uppercase, so the function will not allocate a new [`String`].
/// let already_uppercase = "HELLOWORLD";
/// assert_eq!(to_uppercase(already_uppercase), NoStdCow::Borrowed(already_uppercase));
/// // This string needs to be converted to uppercase, so a new owned value is constructed
/// // and returned.
/// let not_uppercase = "helloworld";
/// assert_eq!(to_uppercase(not_uppercase), NoStdCow::Owned(String::from("HELLOWORLD")));
/// ```
#[derive(Debug, Clone, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub enum NoStdCow<'a, T: Borrow<B>, B: ?Sized> {
    /// A borrowed version of `T`. In the most cases, `T` and `B` are the same type.
    Borrowed(&'a B),
    /// An owned value `T`
    Owned(T),
}

impl<T: Borrow<B>, B: ?Sized> Deref for NoStdCow<'_, T, B> {
    type Target = B;
    fn deref(&self) -> &Self::Target {
        match self {
            Self::Borrowed(b) => b,
            Self::Owned(v) => v.borrow(),
        }
    }
}
impl<T: Borrow<B>, B: ?Sized> NoStdCow<'_, T, B> {
    /// Returns true if the data is borrowed, i.e. if `to_mut` would require additional work.
    pub const fn is_borrowed(&self) -> bool {
        match self {
            Self::Borrowed(_) => true,
            Self::Owned(_) => false,
        }
    }
    /// Returns true if the data is owned, i.e. if `to_mut` would be a no-op.
    pub const fn is_owned(&self) -> bool {
        match self {
            Self::Borrowed(_) => false,
            Self::Owned(_) => true,
        }
    }
}
impl<T: Clone + Borrow<T>> RefCow<'_, T> {
    /// Acquires a mutable reference to the owned form of the data.
    ///
    /// Clones the data if it is not already owned.
    /// 
    /// Note that since we don't have access to [`alloc::borrow::ToOwned`],
    /// this method is only available for cases where generic types `B == T`.
    pub fn to_mut(&mut self) -> &mut T {
        match *self {
            Self::Owned(ref mut v) => v,
            Self::Borrowed(ref mut v) => {
                *self = Self::Owned((*v).clone());
                match *self {
                    Self::Borrowed(_) => unreachable!(),
                    Self::Owned(ref mut v) => v,
                }
            }
        }
    }
    /// Extracts the owned data.
    ///
    /// Clones the data if it is not already owned.
    /// 
    /// Note that since we don't have access to [`alloc::borrow::ToOwned`],
    /// this method is only available for cases where generic types `B == T`.
    pub fn into_owned(self) -> T {
        match self {
            Self::Owned(v) => v,
            Self::Borrowed(v) => v.clone(),
        }
    }
}

impl<T: Borrow<B> + Default, B: ?Sized> Default for NoStdCow<'_, T, B> {
    fn default() -> Self {
        Self::Owned(T::default())
    }
}

impl<'a, T: Borrow<B>, B: ?Sized> From<&'a B> for NoStdCow<'a, T, B> {
    fn from(value: &'a B) -> Self {
        Self::Borrowed(value)
    }
}