// Copyright 2014 The Rust Project Developers. See the COPYRIGHT
// file at the top-level directory of this distribution and at
// http://rust-lang.org/COPYRIGHT.
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

// run-pass
#![allow(unused_mut)]
#![allow(unused_variables)]
fn main() {
    let mut shrinker: Box<Iterator<Item=i32>> = Box::new(vec![1].into_iter());
    println!("{:?}", shrinker.next());
    for v in shrinker { assert!(false); }

    let mut shrinker: &mut Iterator<Item=i32> = &mut vec![1].into_iter();
    println!("{:?}", shrinker.next());
    for v in shrinker { assert!(false); }
}
