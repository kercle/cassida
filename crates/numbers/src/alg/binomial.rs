use crate::integer::BigInteger;

pub fn binomial_coeff(n: &BigInteger, k: &BigInteger) -> BigInteger {
    if n < k || n.is_negative() || k.is_negative() {
        return BigInteger::zero();
    } else if n == k || k.is_one() {
        return BigInteger::one();
    }

    let mut product_numerator = BigInteger::one();
    let mut product_denominator = BigInteger::one();

    let mut n_plus_1_minus_i = n.clone();
    let mut i = BigInteger::one();
    while &i <= k {
        product_numerator = &product_numerator * &n_plus_1_minus_i;
        product_denominator = &product_denominator * &i;

        n_plus_1_minus_i = n_plus_1_minus_i.decremented();
        i = i.incremented();
    }

    let Some(res) = product_numerator / product_denominator else {
        unreachable!("product_denominator cannot become zero here.");
    };

    res
}

#[cfg(test)]
mod gcd_tests {
    use super::*;

    #[test]
    fn test_binomial_coeff() {
        let n = BigInteger::from_u64(30);
        let k = BigInteger::from_u64(3);
        let expected = BigInteger::from_u64(4060);

        assert_eq!(binomial_coeff(&n, &k), expected);

        let n = BigInteger::from_u64(1);
        let k = BigInteger::from_u64(4);
        let expected = BigInteger::from_u64(0);

        assert_eq!(binomial_coeff(&n, &k), expected);

        let n = BigInteger::from_i64(-5);
        let k = BigInteger::from_u64(4);
        let expected = BigInteger::from_u64(0);

        assert_eq!(binomial_coeff(&n, &k), expected);

        let n = BigInteger::from_u64(5);
        let k = BigInteger::from_i64(-4);
        let expected = BigInteger::from_u64(0);

        assert_eq!(binomial_coeff(&n, &k), expected);
    }
}
