// Copyright 2013 The Rust Project Developers. See the COPYRIGHT
// file at the top-level directory of this distribution and at
// http://rust-lang.org/COPYRIGHT.
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

// xfail-fast
// aux-build:cci_const.rs

extern mod cci_const;
const foo: &'static str = cci_const::foopy;
const a: uint = cci_const::uint_val;
const b: uint = cci_const::uint_expr + 5;

fn main() {
    fail_unless!(a == 12);
    let foo2 = a;
    fail_unless!(foo2 == cci_const::uint_val);
    fail_unless!(b == cci_const::uint_expr + 5);
    fail_unless!(foo == cci_const::foopy);
}
