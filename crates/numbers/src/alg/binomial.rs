use crate::integer::{self, BigInteger};

pub fn binomial_coeff(n: u64, mut k: u64) -> BigInteger {
    if n < k {
        return BigInteger::zero();
    } else if n == k || k == 0 {
        return BigInteger::one();
    }

    if n - k < k {
        k = n - k;
    }

    let mut product_numerator = BigInteger::one();
    let mut product_denominator = BigInteger::one();

    for i in 1..=k {
        product_numerator = &product_numerator * &BigInteger::from_u64(n + 1 - i);
        product_denominator = &product_denominator * &BigInteger::from_u64(i);
    }

    let Some(res) = product_numerator / product_denominator else {
        unreachable!("product_denominator cannot become zero here.");
    };

    res
}

#[derive(Default)]
pub struct BinomialGenerator {
    buffer: Vec<Option<BigInteger>>,
}

impl BinomialGenerator {
    fn buffer_pos(n: u64, k: u64) -> usize {
        // we store the deduplicated part of Pascal's
        // triangle:
        //
        //      0  1  2
        //    ---------> k
        //  0 | 2
        //  1 | 3
        //  3 | 4  6
        //  4 | 5 10
        //  5 | 6 15 20
        //  6 | 7 21 35
        // n  V
        //
        // buffer_pos maps coordinates (n,k) to a
        // position in the buffer by considering
        // even and odd n separately.

        // we don't store row and col 0
        let n = n - 1;
        let k = k - 1;

        let m = n / 2;

        // q = 2 * (m * (m + 1) / 2 + k);
        let q = m * (m + 1) + 2 * k;

        if (n & 1) == 0 {
            // if n is even, we take
            // the (even) q directly
            q as usize
        } else {
            // if n is odd, we take
            // the the next odd number
            (q + 1) as usize
        }
    }

    pub fn fill(&mut self, n: u64, k: u64) -> &BigInteger {
        let pos = Self::buffer_pos(n, k);

        let a = self.get(n - 1, k - 1).clone();
        let b = self.get(n - 1, k).clone();

        self.buffer[pos] = Some(a + b);
        self.buffer.get(pos).unwrap().as_ref().unwrap()
    }

    pub fn get(&mut self, n: u64, mut k: u64) -> &BigInteger {
        if n < k {
            return &integer::ZERO;
        } else if n == k || k == 0 {
            return &integer::ONE;
        }

        if n - k < k {
            k = n - k;
        }

        let pos = Self::buffer_pos(n, k);
        if pos >= self.buffer.len() {
            self.buffer.resize(pos + 1, None);
            self.fill(n, k)
        } else if self.buffer.get(pos).unwrap().is_none() {
            self.fill(n, k)
        } else {
            self.buffer.get(pos).unwrap().as_ref().unwrap()
        }
    }
}

#[cfg(test)]
mod binomial_tests {
    use super::*;

    #[test]
    fn test_binomial_coeff() {
        assert_eq!(binomial_coeff(30, 3), BigInteger::from_u64(4060));
        assert_eq!(binomial_coeff(25, 21), BigInteger::from_u64(12650));
        assert_eq!(binomial_coeff(1, 4), BigInteger::from_u64(0));
    }

    #[test]
    fn test_binomial_coeff_generator() {
        let mut generator = BinomialGenerator::default();

        assert_eq!(generator.get(30, 3), &BigInteger::from_u64(4060));
        assert_eq!(generator.get(25, 21), &BigInteger::from_u64(12650));
        assert_eq!(generator.get(20, 7), &BigInteger::from_u64(77520));
        assert_eq!(generator.get(1, 4), &BigInteger::from_u64(0));
        assert_eq!(generator.get(27, 7), &BigInteger::from_u64(888030));
    }
}
