// Copyright 2014 The Rust Project Developers. See the COPYRIGHT
// file at the top-level directory of this distribution and at
// http://rust-lang.org/COPYRIGHT.
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

macro_rules! int_module { ($T:ty, $T_i:ident) => (
#[cfg(test)]
mod tests {
    use core::$T_i::*;
    use core::isize;
    use core::num::{FromStrRadix, Int, SignedInt};
    use core::ops::{Shl, Shr, Not, BitXor, BitAnd, BitOr};
    use num;

    #[test]
    fn test_overflows() {
        assert!(MAX > 0);
        assert!(MIN <= 0);
        assert!(MIN + MAX + 1 == 0);
    }

    #[test]
    fn test_num() {
        num::test_num(10 as $T, 2 as $T);
    }

    #[test]
    pub fn test_abs() {
        assert!((1 as $T).abs() == 1 as $T);
        assert!((0 as $T).abs() == 0 as $T);
        assert!((-1 as $T).abs() == 1 as $T);
    }

    #[test]
    fn test_signum() {
        assert!((1 as $T).signum() == 1 as $T);
        assert!((0 as $T).signum() == 0 as $T);
        assert!((-0 as $T).signum() == 0 as $T);
        assert!((-1 as $T).signum() == -1 as $T);
    }

    #[test]
    fn test_is_positive() {
        assert!((1 as $T).is_positive());
        assert!(!(0 as $T).is_positive());
        assert!(!(-0 as $T).is_positive());
        assert!(!(-1 as $T).is_positive());
    }

    #[test]
    fn test_is_negative() {
        assert!(!(1 as $T).is_negative());
        assert!(!(0 as $T).is_negative());
        assert!(!(-0 as $T).is_negative());
        assert!((-1 as $T).is_negative());
    }

    #[test]
    fn test_bitwise_operators() {
        assert!(0b1110 as $T == (0b1100 as $T).bitor(0b1010 as $T));
        assert!(0b1000 as $T == (0b1100 as $T).bitand(0b1010 as $T));
        assert!(0b0110 as $T == (0b1100 as $T).bitxor(0b1010 as $T));
        assert!(0b1110 as $T == (0b0111 as $T).shl(1));
        assert!(0b0111 as $T == (0b1110 as $T).shr(1));
        assert!(-(0b11 as $T) - (1 as $T) == (0b11 as $T).not());
    }

    static A: $T = 0b0101100;
    static B: $T = 0b0100001;
    static C: $T = 0b1111001;

    static _0: $T = 0;
    static _1: $T = !0;

    #[test]
    fn test_count_ones() {
        assert!(A.count_ones() == 3);
        assert!(B.count_ones() == 2);
        assert!(C.count_ones() == 5);
    }

    #[test]
    fn test_count_zeros() {
        assert!(A.count_zeros() == BITS - 3);
        assert!(B.count_zeros() == BITS - 2);
        assert!(C.count_zeros() == BITS - 5);
    }

    #[test]
    fn test_rotate() {
        assert_eq!(A.rotate_left(6).rotate_right(2).rotate_right(4), A);
        assert_eq!(B.rotate_left(3).rotate_left(2).rotate_right(5), B);
        assert_eq!(C.rotate_left(6).rotate_right(2).rotate_right(4), C);

        // Rotating these should make no difference
        //
        // We test using 124 bits because to ensure that overlong bit shifts do
        // not cause undefined behaviour. See #10183.
        assert_eq!(_0.rotate_left(124), _0);
        assert_eq!(_1.rotate_left(124), _1);
        assert_eq!(_0.rotate_right(124), _0);
        assert_eq!(_1.rotate_right(124), _1);

        // Rotating by 0 should have no effect
        assert_eq!(A.rotate_left(0), A);
        assert_eq!(B.rotate_left(0), B);
        assert_eq!(C.rotate_left(0), C);
        // Rotating by a multiple of word size should also have no effect
        assert_eq!(A.rotate_left(64), A);
        assert_eq!(B.rotate_left(64), B);
        assert_eq!(C.rotate_left(64), C);
    }

    #[test]
    fn test_swap_bytes() {
        assert_eq!(A.swap_bytes().swap_bytes(), A);
        assert_eq!(B.swap_bytes().swap_bytes(), B);
        assert_eq!(C.swap_bytes().swap_bytes(), C);

        // Swapping these should make no difference
        assert_eq!(_0.swap_bytes(), _0);
        assert_eq!(_1.swap_bytes(), _1);
    }

    #[test]
    fn test_le() {
        assert_eq!(Int::from_le(A.to_le()), A);
        assert_eq!(Int::from_le(B.to_le()), B);
        assert_eq!(Int::from_le(C.to_le()), C);
        assert_eq!(Int::from_le(_0), _0);
        assert_eq!(Int::from_le(_1), _1);
        assert_eq!(_0.to_le(), _0);
        assert_eq!(_1.to_le(), _1);
    }

    #[test]
    fn test_be() {
        assert_eq!(Int::from_be(A.to_be()), A);
        assert_eq!(Int::from_be(B.to_be()), B);
        assert_eq!(Int::from_be(C.to_be()), C);
        assert_eq!(Int::from_be(_0), _0);
        assert_eq!(Int::from_be(_1), _1);
        assert_eq!(_0.to_be(), _0);
        assert_eq!(_1.to_be(), _1);
    }

    #[test]
    fn test_signed_checked_div() {
        assert!(10.checked_div(2) == Some(5));
        assert!(5.checked_div(0) == None);
        assert!(isize::MIN.checked_div(-1) == None);
    }

    #[test]
    fn test_from_str() {
        fn from_str<T: ::std::str::FromStr>(t: &str) -> Option<T> {
            ::std::str::FromStr::from_str(t).ok()
        }
        assert_eq!(from_str::<$T>("0"), Some(0 as $T));
        assert_eq!(from_str::<$T>("3"), Some(3 as $T));
        assert_eq!(from_str::<$T>("10"), Some(10 as $T));
        assert_eq!(from_str::<i32>("123456789"), Some(123456789 as i32));
        assert_eq!(from_str::<$T>("00100"), Some(100 as $T));

        assert_eq!(from_str::<$T>("-1"), Some(-1 as $T));
        assert_eq!(from_str::<$T>("-3"), Some(-3 as $T));
        assert_eq!(from_str::<$T>("-10"), Some(-10 as $T));
        assert_eq!(from_str::<i32>("-123456789"), Some(-123456789 as i32));
        assert_eq!(from_str::<$T>("-00100"), Some(-100 as $T));

        assert_eq!(from_str::<$T>(""), None);
        assert_eq!(from_str::<$T>(" "), None);
        assert_eq!(from_str::<$T>("x"), None);
    }

    #[test]
    fn test_from_str_radix() {
        assert_eq!(FromStrRadix::from_str_radix("123", 10), Ok(123 as $T));
        assert_eq!(FromStrRadix::from_str_radix("1001", 2), Ok(9 as $T));
        assert_eq!(FromStrRadix::from_str_radix("123", 8), Ok(83 as $T));
        assert_eq!(FromStrRadix::from_str_radix("123", 16), Ok(291 as i32));
        assert_eq!(FromStrRadix::from_str_radix("ffff", 16), Ok(65535 as i32));
        assert_eq!(FromStrRadix::from_str_radix("FFFF", 16), Ok(65535 as i32));
        assert_eq!(FromStrRadix::from_str_radix("z", 36), Ok(35 as $T));
        assert_eq!(FromStrRadix::from_str_radix("Z", 36), Ok(35 as $T));

        assert_eq!(FromStrRadix::from_str_radix("-123", 10), Ok(-123 as $T));
        assert_eq!(FromStrRadix::from_str_radix("-1001", 2), Ok(-9 as $T));
        assert_eq!(FromStrRadix::from_str_radix("-123", 8), Ok(-83 as $T));
        assert_eq!(FromStrRadix::from_str_radix("-123", 16), Ok(-291 as i32));
        assert_eq!(FromStrRadix::from_str_radix("-ffff", 16), Ok(-65535 as i32));
        assert_eq!(FromStrRadix::from_str_radix("-FFFF", 16), Ok(-65535 as i32));
        assert_eq!(FromStrRadix::from_str_radix("-z", 36), Ok(-35 as $T));
        assert_eq!(FromStrRadix::from_str_radix("-Z", 36), Ok(-35 as $T));

        assert_eq!(FromStrRadix::from_str_radix("Z", 35).ok(), None::<$T>);
        assert_eq!(FromStrRadix::from_str_radix("-9", 2).ok(), None::<$T>);
    }

    #[test]
    fn test_pow() {
        let mut r = 2 as $T;

        assert_eq!(r.pow(2u32), 4 as $T);
        assert_eq!(r.pow(0u32), 1 as $T);
        r = -2 as $T;
        assert_eq!(r.pow(2u32), 4 as $T);
        assert_eq!(r.pow(3u32), -8 as $T);
    }
}

)}
