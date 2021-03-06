// Copyright 2012 The Rust Project Developers. See the COPYRIGHT
// file at the top-level directory of this distribution and at
// http://rust-lang.org/COPYRIGHT.
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

//! Operations and constants for `int`

use num::NumCast;

pub use self::inst::pow;

mod inst {
    pub type T = int;
    pub const bits: uint = ::uint::bits;

    /// Returns `base` raised to the power of `exponent`
    pub pure fn pow(base: int, exponent: uint) -> int {
        if exponent == 0u {
            //Not mathemtically true if ~[base == 0]
            return 1;
        }
        if base == 0 { return 0; }
        let mut my_pow  = exponent;
        let mut acc     = 1;
        let mut multiplier = base;
        while(my_pow > 0u) {
            if my_pow % 2u == 1u {
                acc *= multiplier;
            }
            my_pow     /= 2u;
            multiplier *= multiplier;
        }
        return acc;
    }

    #[test]
    fn test_pow() {
        fail_unless!((pow(0, 0u) == 1));
        fail_unless!((pow(0, 1u) == 0));
        fail_unless!((pow(0, 2u) == 0));
        fail_unless!((pow(-1, 0u) == 1));
        fail_unless!((pow(1, 0u) == 1));
        fail_unless!((pow(-3, 2u) == 9));
        fail_unless!((pow(-3, 3u) == -27));
        fail_unless!((pow(4, 9u) == 262144));
    }

    #[test]
    fn test_overflows() {
        fail_unless!((::int::max_value > 0));
        fail_unless!((::int::min_value <= 0));
        fail_unless!((::int::min_value + ::int::max_value + 1 == 0));
    }
}

impl NumCast for int {
    /**
     * Cast `n` to a `int`
     */
    #[inline(always)]
    static pure fn from<N:NumCast>(n: N) -> int { n.to_int() }

    #[inline(always)] pure fn to_u8(&self)    -> u8    { *self as u8    }
    #[inline(always)] pure fn to_u16(&self)   -> u16   { *self as u16   }
    #[inline(always)] pure fn to_u32(&self)   -> u32   { *self as u32   }
    #[inline(always)] pure fn to_u64(&self)   -> u64   { *self as u64   }
    #[inline(always)] pure fn to_uint(&self)  -> uint  { *self as uint  }

    #[inline(always)] pure fn to_i8(&self)    -> i8    { *self as i8    }
    #[inline(always)] pure fn to_i16(&self)   -> i16   { *self as i16   }
    #[inline(always)] pure fn to_i32(&self)   -> i32   { *self as i32   }
    #[inline(always)] pure fn to_i64(&self)   -> i64   { *self as i64   }
    #[inline(always)] pure fn to_int(&self)   -> int   { *self          }

    #[inline(always)] pure fn to_f32(&self)   -> f32   { *self as f32   }
    #[inline(always)] pure fn to_f64(&self)   -> f64   { *self as f64   }
    #[inline(always)] pure fn to_float(&self) -> float { *self as float }
}

#[test]
fn test_numcast() {
    fail_unless!((20u   == 20i.to_uint()));
    fail_unless!((20u8  == 20i.to_u8()));
    fail_unless!((20u16 == 20i.to_u16()));
    fail_unless!((20u32 == 20i.to_u32()));
    fail_unless!((20u64 == 20i.to_u64()));
    fail_unless!((20i   == 20i.to_int()));
    fail_unless!((20i8  == 20i.to_i8()));
    fail_unless!((20i16 == 20i.to_i16()));
    fail_unless!((20i32 == 20i.to_i32()));
    fail_unless!((20i64 == 20i.to_i64()));
    fail_unless!((20f   == 20i.to_float()));
    fail_unless!((20f32 == 20i.to_f32()));
    fail_unless!((20f64 == 20i.to_f64()));

    fail_unless!((20i == NumCast::from(20u)));
    fail_unless!((20i == NumCast::from(20u8)));
    fail_unless!((20i == NumCast::from(20u16)));
    fail_unless!((20i == NumCast::from(20u32)));
    fail_unless!((20i == NumCast::from(20u64)));
    fail_unless!((20i == NumCast::from(20i)));
    fail_unless!((20i == NumCast::from(20i8)));
    fail_unless!((20i == NumCast::from(20i16)));
    fail_unless!((20i == NumCast::from(20i32)));
    fail_unless!((20i == NumCast::from(20i64)));
    fail_unless!((20i == NumCast::from(20f)));
    fail_unless!((20i == NumCast::from(20f32)));
    fail_unless!((20i == NumCast::from(20f64)));

    fail_unless!((20i == num::cast(20u)));
    fail_unless!((20i == num::cast(20u8)));
    fail_unless!((20i == num::cast(20u16)));
    fail_unless!((20i == num::cast(20u32)));
    fail_unless!((20i == num::cast(20u64)));
    fail_unless!((20i == num::cast(20i)));
    fail_unless!((20i == num::cast(20i8)));
    fail_unless!((20i == num::cast(20i16)));
    fail_unless!((20i == num::cast(20i32)));
    fail_unless!((20i == num::cast(20i64)));
    fail_unless!((20i == num::cast(20f)));
    fail_unless!((20i == num::cast(20f32)));
    fail_unless!((20i == num::cast(20f64)));
}
