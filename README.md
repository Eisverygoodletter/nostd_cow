# Std-less Cow
This library provides `NoStdCow`, which is an implementation of a copy-on-write smarter pointer which doesn't rely on `std` or `alloc`.

[![Crates.io Version](https://img.shields.io/crates/v/nostd_cow)](https://docs.rs/nostd_cow/latest/nostd_cow/)
[![docs.rs](https://img.shields.io/docsrs/nostd_cow)](https://docs.rs/nostd_cow/latest/nostd_cow/)
![Crates.io License](https://img.shields.io/crates/l/nostd_cow) 

If you have `std` or `alloc` available, use [`alloc::borrow::Cow`](https://doc.rust-lang.org/alloc/borrow/enum.Cow.html) instead. `NoStdCow` is more
targeted towards embedded systems and [`alloc::borrow::Cow`](https://doc.rust-lang.org/alloc/borrow/enum.Cow.html) provides more functionality.
`into_alloc_cow` and `from_alloc_cow` can be used to convert between the two if needed.

### Overview
`NoStdCow` is defined as 
```rust
pub enum NoStdCow<'a, T: Borrow<B>, B> {
    Owned(T),
    Borrowed(&'a B),
}
```
where `&B` is the borrowed form of `T`. In most cases, `T == B` and you will want to use `NoStdCow<'a, T, T>`. It is highly recommended that `T` also has a `Clone` implementation.

# Example
```rust
use nostd_cow::NoStdCow;

/// Convert a string to uppercase if it isn't already uppercase, otherwise
/// just return a reference to the original source.
fn to_uppercase<'a>(source: &'a str) -> NoStdCow<'a, String, str> {
    for c in source.chars() {
        if !c.is_uppercase() {
            return NoStdCow::Owned(source.to_uppercase());
        }
    }
    NoStdCow::Borrowed(source)
}
// This string is already uppercase, so the function will not allocate a new [`String`].
let already_uppercase = "HELLOWORLD";
assert_eq!(to_uppercase(already_uppercase), NoStdCow::Borrowed(already_uppercase));
// This string needs to be converted to uppercase, so a new owned value is constructed
// and returned.
let not_uppercase = "helloworld";
assert_eq!(to_uppercase(not_uppercase), NoStdCow::Owned(String::from("HELLOWORLD")));
```