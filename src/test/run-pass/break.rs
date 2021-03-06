// Copyright 2012 The Rust Project Developers. See the COPYRIGHT
// file at the top-level directory of this distribution and at
// http://rust-lang.org/COPYRIGHT.
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.



pub fn main() {
    let mut i = 0;
    while i < 20 { i += 1; if i == 10 { break; } }
    fail_unless!((i == 10));
    loop { i += 1; if i == 20 { break; } }
    fail_unless!((i == 20));
    for vec::each(~[1, 2, 3, 4, 5, 6]) |x| {
        if *x == 3 { break; } fail_unless!((*x <= 3));
    }
    i = 0;
    while i < 10 { i += 1; if i % 2 == 0 { loop; } fail_unless!((i % 2 != 0)); }
    i = 0;
    loop { 
        i += 1; if i % 2 == 0 { loop; } fail_unless!((i % 2 != 0)); 
        if i >= 10 { break; }
    }
    for vec::each(~[1, 2, 3, 4, 5, 6]) |x| {
        if *x % 2 == 0 { loop; }
        fail_unless!((*x % 2 != 0));
    }
}
