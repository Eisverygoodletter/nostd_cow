# Std-less Cow
This library provides [`NoStdCow`], which is an implementation of a copy-on-write smarter pointer which doesn't rely on `std` or `alloc`.

[![Crates.io Version](https://img.shields.io/crates/v/nostd_cow)](https://docs.rs/nostd_cow/latest/nostd_cow/)
[![docs.rs](https://img.shields.io/docsrs/nostd_cow)](https://docs.rs/nostd_cow/latest/nostd_cow/)
![Crates.io License](https://img.shields.io/crates/l/nostd_cow) 

If you have `std` or `alloc` available, use [`alloc::borrow::Cow`](https://doc.rust-lang.org/alloc/borrow/enum.Cow.html) instead.
