// Copyright 2012 The Rust Project Developers. See the COPYRIGHT
// file at the top-level directory of this distribution and at
// http://rust-lang.org/COPYRIGHT.
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

// run-pass
#![allow(dead_code)]
#![allow(non_camel_case_types)]


// pretty-expanded FIXME #23616

struct font<'a> {
    fontbuf: &'a Vec<u8> ,
}

impl<'a> font<'a> {
    pub fn buf(&self) -> &'a Vec<u8> {
        self.fontbuf
    }
}

fn font(fontbuf: &Vec<u8> ) -> font {
    font {
        fontbuf: fontbuf
    }
}

pub fn main() { }
