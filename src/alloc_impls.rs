extern crate alloc;
use crate::NoStdCow;
use alloc::borrow::{Cow, ToOwned};
use core::borrow::Borrow;
impl<'a, B: ?Sized> NoStdCow<'a, <B as ToOwned>::Owned, B>
where
    <B as ToOwned>::Owned: Borrow<B>,
    B: Clone,
{
    /// Convert this [`NoStdCow`] into a [`alloc::borrow::Cow`].
    /// 
    /// A [`From`] implementation is also available.
    pub fn into_std_cow(self) -> Cow<'a, B> {
        match self {
            Self::Borrowed(b) => Cow::Borrowed(b),
            Self::Owned(o) => Cow::Owned(o)
        }
    }
    /// Convert a [`alloc::borrow::Cow`] into a [`NoStdCow`].
    /// 
    /// A [`From`] implementation is also available.
    pub fn from_std_cow(cow: Cow<'a, B>) -> Self {
        match cow {
            Cow::Borrowed(b) => Self::Borrowed(b),
            Cow::Owned(o) => Self::Owned(o),
        }
    }
}

impl<'a, B: ?Sized> From<Cow<'a, B>> for NoStdCow<'a, <B as ToOwned>::Owned, B>
where
    <B as ToOwned>::Owned: Borrow<B>,
    B: Clone
{
    fn from(value: Cow<'a, B>) -> Self {
        Self::from_std_cow(value)
    }
}
impl<'a, B: ?Sized> From<NoStdCow<'a, <B as ToOwned>::Owned, B>> for Cow<'a, B>
where
    <B as ToOwned>::Owned: Borrow<B>,
    B: Clone
{
    fn from(value: NoStdCow<'a, <B as ToOwned>::Owned, B>) -> Self {
        value.to_std_cow()
    }
}