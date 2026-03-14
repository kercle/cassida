use crate::integer::BigInteger;

pub fn gcd(a: BigInteger, b: BigInteger) -> BigInteger {
    let mut a = a.abs();
    let mut b = b.abs();

    if a.is_zero() {
        return b;
    }
    if b.is_zero() {
        return a;
    }

    let shift = a.trailing_zeros().min(b.trailing_zeros());
    a >>= shift;
    b >>= shift;

    a >>= a.trailing_zeros();

    loop {
        b >>= b.trailing_zeros();

        if a > b {
            std::mem::swap(&mut a, &mut b);
        }

        b -= &a;

        if b.is_zero() {
            return a << shift;
        }
    }
}

#[cfg(test)]
mod gcd_tests {
    use super::*;

    fn bi(n: i64) -> BigInteger {
        BigInteger::from_i64(n)
    }
    fn bu(n: u64) -> BigInteger {
        BigInteger::from_u64(n)
    }

    #[test]
    fn gcd_zero_cases() {
        assert_eq!(gcd(bi(0), bi(0)), bi(0));

        assert_eq!(gcd(bi(0), bi(5)), bi(5));
        assert_eq!(gcd(bi(5), bi(0)), bi(5));

        assert_eq!(gcd(bi(0), bi(-5)), bi(5));
        assert_eq!(gcd(bi(-5), bi(0)), bi(5));
    }

    #[test]
    fn gcd_basic_small() {
        assert_eq!(gcd(bi(1), bi(1)), bi(1));
        assert_eq!(gcd(bi(2), bi(4)), bi(2));
        assert_eq!(gcd(bi(12), bi(18)), bi(6));
        assert_eq!(gcd(bi(48), bi(180)), bi(12));
        assert_eq!(gcd(bi(17), bi(31)), bi(1));
        assert_eq!(gcd(bi(21), bi(14)), bi(7));
    }

    #[test]
    fn gcd_sign_invariance() {
        assert_eq!(gcd(bi(-12), bi(18)), bi(6));
        assert_eq!(gcd(bi(12), bi(-18)), bi(6));
        assert_eq!(gcd(bi(-12), bi(-18)), bi(6));
        assert_eq!(gcd(bi(-17), bi(31)), bi(1));
        assert_eq!(gcd(bi(17), bi(-31)), bi(1));
        assert_eq!(gcd(bi(-17), bi(-31)), bi(1));
    }

    #[test]
    fn gcd_symmetry() {
        let a = bi(48);
        let b = bi(180);
        let g1 = gcd(a.clone(), b.clone());
        let g2 = gcd(b, a);
        assert_eq!(g1, g2);
    }

    #[test]
    fn gcd_when_one_divides_the_other() {
        assert_eq!(gcd(bi(7), bi(21)), bi(7));
        assert_eq!(gcd(bi(21), bi(7)), bi(7));
        assert_eq!(gcd(bi(13), bi(13)), bi(13));
        assert_eq!(gcd(bi(-13), bi(13)), bi(13));
    }

    #[test]
    fn gcd_powers_of_two_and_large_carries() {
        let two_64 = BigInteger::from_str_radix("18446744073709551616", 10).unwrap();
        let two_63 = BigInteger::from_str_radix("9223372036854775808", 10).unwrap();

        assert_eq!(gcd(two_64.clone(), two_63.clone()), two_63);

        let three_two_63 = BigInteger::mul(&two_63, &bu(3));
        assert_eq!(gcd(two_64, three_two_63), two_63);
    }

    #[test]
    fn gcd_large_decimal_inputs() {
        let a = BigInteger::from_str_radix("123456789012345678901234567890", 10).unwrap();
        let b = BigInteger::from_str_radix("9876543210", 10).unwrap();
        let g = BigInteger::from_u64(90);
        assert_eq!(gcd(a, b), g);

        let a = BigInteger::from_str_radix("1000000000000000000000000000000", 10).unwrap();
        let b = BigInteger::from_str_radix("1000000000000000000", 10).unwrap();
        assert_eq!(gcd(a, b.clone()), b);
    }

    #[test]
    fn gcd_euclid_step_property_small_set() {
        let pairs = [
            (48, 180),
            (180, 48),
            (99, 78),
            (78, 99),
            (7, 21),
            (21, 7),
            (17, 31),
            (31, 17),
            (-48, 180),
            (48, -180),
            (-99, -78),
        ];

        for (a, b) in pairs {
            if b == 0 {
                continue;
            }
            let a_bi = bi(a);
            let b_bi = bi(b);

            let g1 = gcd(a_bi.clone(), b_bi.clone());

            let r =
                (a_bi.clone() % b_bi.clone()).expect("remainder should exist for nonzero divisor");
            let g2 = gcd(b_bi, r);

            assert_eq!(g1, g2, "Euclid step failed for a={}, b={}", a, b);
        }
    }

    #[test]
    fn gcd_divides_both_small_set() {
        let pairs = [
            (0, 0),
            (0, 15),
            (15, 0),
            (12, 18),
            (48, 180),
            (17, 31),
            (-12, 18),
            (-48, -180),
        ];

        for (a, b) in pairs {
            let g = gcd(bi(a), bi(b));
            if g.is_zero() {
                assert!(a == 0 && b == 0);
                continue;
            }

            let ra = (bi(a) % g.clone()).unwrap();
            let rb = (bi(b) % g.clone()).unwrap();
            assert!(
                ra.is_zero(),
                "g doesn't divide a: a={}, b={}, g={}, ra={}",
                a,
                b,
                g,
                ra
            );
            assert!(
                rb.is_zero(),
                "g doesn't divide b: a={}, b={}, g={}, rb={}",
                a,
                b,
                g,
                rb
            );
        }
    }

    #[test]
    fn gcd_is_non_negative() {
        let g = gcd(bi(-48), bi(180));
        assert!(g.is_positive() || g.is_zero());
    }

    #[test]
    fn test_gcd_more_cases() {
        fn bi(x: i64) -> BigInteger {
            BigInteger::from_i64(x)
        }

        let cases = [
            (0, 0, 0),
            (0, 15, 15),
            (15, 0, 15),
            (12, 18, 6),
            (48, 180, 12),
            (-48, 180, 12),
            (48, -180, 12),
            (-48, -180, 12),
            (17, 31, 1),
            (270, 192, 6),
        ];

        for (a, b, g) in cases {
            let got = gcd(bi(a), bi(b));
            assert!(
                got.eq_inner(&bi(g)),
                "gcd({}, {}) = {}, expected {}",
                a,
                b,
                got,
                g
            );
        }
    }
}
