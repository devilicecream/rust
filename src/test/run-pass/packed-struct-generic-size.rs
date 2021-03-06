// Copyright 2013 The Rust Project Developers. See the COPYRIGHT
// file at the top-level directory of this distribution and at
// http://rust-lang.org/COPYRIGHT.
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

use std::mem;

#[packed]
struct S<T, S> {
    a: T,
    b: u8,
    c: S
}

pub fn main() {
    assert_eq!(mem::size_of::<S<u8, u8>>(), 3);

    assert_eq!(mem::size_of::<S<u64, u16>>(), 11);

    assert_eq!(mem::size_of::<S<~str, Vec<int> >>(),
               1 + mem::size_of::<~str>() + mem::size_of::<Vec<int> >());
}
