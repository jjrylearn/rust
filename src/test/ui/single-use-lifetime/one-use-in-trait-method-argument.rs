// Copyright 2016 The Rust Project Developers. See the COPYRIGHT
// file at the top-level directory of this distribution and at
// http://rust-lang.org/COPYRIGHT.
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

// Test that we DO warn for a lifetime on an impl used only in `&self`
// in a trait method.

#![deny(single_use_lifetimes)]
#![allow(dead_code)]
#![allow(unused_variables)]

struct Foo<'f> {
    data: &'f u32
}

impl<'f> Iterator for Foo<'f> {
    type Item = &'f u32;

    fn next<'g>(&'g mut self) -> Option<Self::Item> { //~ ERROR `'g` only used once
        None
    }
}

fn main() { }
