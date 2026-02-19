use std::fmt;

type Digit = u64;
type DoubleDigit = u128;

const DIGIT_BITS: usize = Digit::BITS as usize;
const MASK_LOWER: DoubleDigit = Digit::MAX as DoubleDigit;

#[derive(Debug, PartialEq, Clone, Copy)]
pub enum Sign {
    Positive,
    Negative,
}

impl fmt::Display for Sign {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Sign::Positive => write!(f, "+"),
            Sign::Negative => write!(f, "-"),
        }
    }
}

enum CompareFunction {
    Greater,
    Less,
    GreaterEqual,
    LessEqual,
    Equal,
    NotEqual,
}

#[derive(Clone)]
pub struct BigInteger {
    digits: Vec<Digit>,
    sign: Sign,
}

impl BigInteger {
    pub fn from_slice(sign: Sign, digits: &[Digit]) -> Self {
        Self::from_vec(sign, digits.to_vec())
    }

    pub fn from_vec(sign: Sign, digits: Vec<Digit>) -> Self {
        let mut res = BigInteger { digits, sign };
        res.trim_leading_zeros();

        if res.digits.len() == 1 && res.digits[0] == 0 {
            res.sign = Sign::Positive;
        }

        res
    }

    pub fn from_i64(value: i64) -> Self {
        if value < 0 {
            BigInteger::from_vec(Sign::Negative, vec![value.wrapping_abs() as Digit])
        } else {
            BigInteger::from_vec(Sign::Positive, vec![value as Digit])
        }
    }

    pub fn from_u64(value: u64) -> Self {
        BigInteger::from_vec(Sign::Positive, vec![value])
    }

    pub fn zero() -> Self {
        BigInteger::from_u64(0)
    }

    pub fn one() -> Self {
        BigInteger::from_u64(1)
    }

    pub fn minus_one() -> Self {
        BigInteger::from_i64(-1)
    }

    pub fn from_str_radix(s: &str, radix: u32) -> Result<Self, String> {
        if radix != 10 {
            return Err(format!("Unsupported radix: {}", radix));
        }

        if s.is_empty() {
            return Err("Input string is empty".to_string());
        }

        let s = s.trim();
        let mut chars_iter = s.chars();

        let sign = if s.starts_with('-') {
            chars_iter.next();
            Sign::Negative
        } else if s.starts_with('+') {
            chars_iter.next();
            Sign::Positive
        } else {
            Sign::Positive
        };

        let mut result = Self::zero();
        for chunk in chars_iter.collect::<Vec<_>>().chunks(18) {
            let chunk_str: String = chunk.iter().collect();
            let factor = 10u64.pow(chunk.len() as u32);

            if let Ok(value) = chunk_str.parse::<u64>() {
                result = BigInteger::mul(&result, &BigInteger::from_u64(factor));
                result = BigInteger::add(&result, &BigInteger::from_u64(value));
            } else {
                return Err(format!("Invalid number: {}", chunk_str));
            }
        }

        result.sign = sign;
        result.trim_leading_zeros();
        Ok(result)
    }

    fn trim_leading_zeros_from_digits(digits: &mut Vec<Digit>) {
        while let Some(&0) = digits.last() {
            digits.pop();
        }
    }

    fn trim_leading_zeros(&mut self) {
        Self::trim_leading_zeros_from_digits(&mut self.digits);
        if self.digits.is_empty() {
            self.digits.push(0);
            self.sign = Sign::Positive;
        }
    }

    pub fn is_zero(&self) -> bool {
        for &digit in &self.digits {
            if digit != 0 {
                return false;
            }
        }

        true
    }
    pub fn is_minus_one(&self) -> bool {
        self.eq_inner(&BigInteger::minus_one())
    }

    pub fn is_one(&self) -> bool {
        self.eq_inner(&BigInteger::one())
    }

    pub fn is_negative(&self) -> bool {
        self.sign == Sign::Negative
    }

    pub fn is_positive(&self) -> bool {
        self.sign == Sign::Positive
    }

    pub fn eq_inner(&self, other: &Self) -> bool {
        self.sign == other.sign
            && Self::cmp_digits(CompareFunction::Equal, &self.digits, &other.digits)
    }

    pub fn gt_inner(&self, other: &Self) -> bool {
        if self.sign != other.sign {
            return self.sign == Sign::Positive;
        }

        let ret = Self::cmp_digits(CompareFunction::Greater, &self.digits, &other.digits);

        if self.sign == Sign::Negative {
            !ret
        } else {
            ret
        }
    }

    pub fn lt_inner(&self, other: &Self) -> bool {
        if self.sign != other.sign {
            return self.sign == Sign::Negative;
        }

        let ret = Self::cmp_digits(CompareFunction::Less, &self.digits, &other.digits);

        if self.sign == Sign::Negative {
            !ret
        } else {
            ret
        }
    }

    pub fn ge_inner(&self, other: &Self) -> bool {
        if self.sign != other.sign {
            return self.sign == Sign::Positive;
        }

        Self::cmp_digits(CompareFunction::GreaterEqual, &self.digits, &other.digits)
    }

    pub fn le_inner(&self, other: &Self) -> bool {
        if self.sign != other.sign {
            return self.sign == Sign::Negative;
        }

        Self::cmp_digits(CompareFunction::LessEqual, &self.digits, &other.digits)
    }

    pub fn abs(&self) -> Self {
        BigInteger::from_vec(Sign::Positive, self.digits.clone())
    }

    pub fn sign(&self) -> Sign {
        self.sign
    }

    pub fn flip_sign(&mut self) {
        self.sign = match self.sign {
            Sign::Positive => Sign::Negative,
            Sign::Negative => Sign::Positive,
        };
    }

    pub fn digit(&self, index: usize) -> Digit {
        self.digits.get(index).cloned().unwrap_or_default()
    }

    pub fn digits(&self) -> &Vec<Digit> {
        &self.digits
    }

    fn cmp_digits(f: CompareFunction, lhs: &[Digit], rhs: &[Digit]) -> bool {
        if let CompareFunction::Equal = f {
            return !Self::cmp_digits(CompareFunction::NotEqual, lhs, rhs);
        } else if let CompareFunction::GreaterEqual = f {
            return Self::cmp_digits(CompareFunction::Greater, lhs, rhs)
                || Self::cmp_digits(CompareFunction::Equal, lhs, rhs);
        } else if let CompareFunction::LessEqual = f {
            return Self::cmp_digits(CompareFunction::Less, lhs, rhs)
                || Self::cmp_digits(CompareFunction::Equal, lhs, rhs);
        }

        for i in (0..lhs.len().max(rhs.len())).rev() {
            let da = lhs.get(i).cloned().unwrap_or_default();
            let db = rhs.get(i).cloned().unwrap_or_default();

            if da == db {
                continue;
            }

            match f {
                CompareFunction::Greater => return da > db,
                CompareFunction::Less => return da < db,
                CompareFunction::NotEqual => return da != db,
                _ => unreachable!(),
            }
        }

        false
    }

    fn add_digits_naive(lhs: &[Digit], rhs: &[Digit]) -> Vec<Digit> {
        let mut result = Vec::new();
        let mut carry_previous = false;

        for i in 0..lhs.len().max(rhs.len()) {
            let da = lhs.get(i).cloned().unwrap_or_default();
            let db = rhs.get(i).cloned().unwrap_or_default();

            let (a, carry_step_1) = if carry_previous {
                da.overflowing_add(1)
            } else {
                (da, false)
            };

            let (sum, carry_step_2) = a.overflowing_add(db);
            result.push(sum);

            carry_previous = carry_step_1 || carry_step_2;
        }

        if carry_previous {
            result.push(1);
        }

        result
    }

    fn sub_digits_larger_from_smaller_naive(first: &[Digit], second: &[Digit]) -> Vec<Digit> {
        let mut result_digits = Vec::new();
        let mut carry_previous = false;

        for i in 0..first.len().max(second.len()) {
            let da = first.get(i).cloned().unwrap_or_default();
            let db = second.get(i).cloned().unwrap_or_default();

            let (da, carry_step_1) = if carry_previous {
                da.overflowing_sub(1)
            } else {
                (da, false)
            };

            let (diff, carry_step_2) = da.overflowing_sub(db);
            result_digits.push(diff);

            carry_previous = carry_step_1 || carry_step_2;
        }

        result_digits
    }

    fn mul_by_digit_naive(lhs: &[Digit], rhs: Digit) -> Vec<Digit> {
        let mut result = Vec::new();
        let mut carry_previous = 0;

        for &digit in lhs {
            let a = digit as u128;
            let b = rhs as u128;
            let product = a.wrapping_mul(b);

            let low = (product & MASK_LOWER) as Digit;
            let high = (product >> DIGIT_BITS) as Digit;

            let (sum, carry) = low.overflowing_add(carry_previous);
            result.push(sum);

            if carry {
                carry_previous = high + 1;
            } else {
                carry_previous = high;
            }
        }

        if carry_previous > 0 {
            result.push(carry_previous);
        }

        result
    }

    fn mul_digits_naive(lhs: &[Digit], rhs: &[Digit]) -> Vec<Digit> {
        let mut result = Vec::new();

        for d in rhs.iter().rev() {
            result.insert(0, 0); // Shift result to the left for each new digit
            let product = Self::mul_by_digit_naive(lhs, *d);
            result = Self::add_digits_naive(&result, &product);
        }

        result
    }

    fn div_by_digit_with_reminder_naive(lhs: &[Digit], rhs: Digit) -> Option<(Vec<Digit>, Digit)> {
        if rhs == 0 {
            return None;
        }

        let mut result = Vec::with_capacity(lhs.len());
        let mut rem: DoubleDigit = 0;

        for &digit in lhs.iter().rev() {
            rem = (rem << DIGIT_BITS) | digit as DoubleDigit; // Shift left and add new digit
            let quotient = (rem / rhs as DoubleDigit) as Digit;
            rem %= rhs as DoubleDigit; // Update remainder

            result.push(quotient);
        }

        result.reverse();
        Some((result, rem as Digit))
    }

    fn estimate_quotient_digit(rem: &[Digit], rhs: &[Digit]) -> Digit {
        let n = rhs.len();
        let m = rem.len();

        // Use the top two digits of rem and the top digit of rhs
        let rem_hi = if m > n {
            ((rem[m - 1] as DoubleDigit) << DIGIT_BITS) | (rem[m - 2] as DoubleDigit)
        } else if m >= n {
            rem[m - 1] as DoubleDigit
        } else {
            0
        };

        let rhs_hi = rhs[n - 1] as DoubleDigit;

        let q_hat = rem_hi / rhs_hi;
        if q_hat >= Digit::MAX as DoubleDigit {
            Digit::MAX
        } else {
            q_hat as Digit + 1 // Round up to ensure we don't underestimate
        }
    }

    fn div_multi_digit_with_reminder_naive(
        lhs: &[Digit],
        rhs: &[Digit],
    ) -> Option<(Vec<Digit>, Vec<Digit>)> {
        if !Self::cmp_digits(CompareFunction::NotEqual, lhs, &[0]) {
            return None;
        }

        let mut result = Vec::new();
        let mut rem: Vec<Digit> = Vec::new();
        let mut rhs = rhs.to_vec();
        Self::trim_leading_zeros_from_digits(&mut rhs);

        for &digit in lhs.iter().rev() {
            rem.insert(0, digit); // Add new digit to the remainder
            let mut quotient = 0;

            if Self::cmp_digits(CompareFunction::GreaterEqual, &rem, &rhs) {
                quotient = Self::estimate_quotient_digit(&rem, &rhs);

                let mut prod = Self::mul_by_digit_naive(&rhs, quotient);

                while Self::cmp_digits(CompareFunction::Greater, &prod, &rem) && quotient > 0 {
                    quotient -= 1;
                    prod = Self::sub_digits_larger_from_smaller_naive(&prod, &rhs);
                }

                rem = Self::sub_digits_larger_from_smaller_naive(&rem, &prod);
                Self::trim_leading_zeros_from_digits(&mut rem);
            }

            result.push(quotient);
        }

        result.reverse();
        Some((result, rem))
    }

    pub fn add(first: &Self, second: &Self) -> Self {
        match (first.sign, second.sign) {
            (Sign::Positive, Sign::Positive) => BigInteger::from_vec(
                Sign::Positive,
                Self::add_digits_naive(&first.digits, &second.digits),
            ),
            (Sign::Positive, Sign::Negative) => Self::sub(first, &second.abs()),
            (Sign::Negative, Sign::Positive) => Self::sub(second, &first.abs()),
            (Sign::Negative, Sign::Negative) => BigInteger::from_vec(
                Sign::Negative,
                Self::add_digits_naive(&first.digits, &second.digits),
            ),
        }
    }

    pub fn increment(&mut self) -> Self {
        Self::add(self, &BigInteger::one())
    }

    pub fn sub(lhs: &Self, rhs: &Self) -> Self {
        let (lhs, rhs, flipped) =
            if Self::cmp_digits(CompareFunction::Greater, &lhs.digits, &rhs.digits) {
                (lhs, rhs, false)
            } else {
                (rhs, lhs, true)
            };

        let mut res = match (lhs.sign, rhs.sign) {
            (Sign::Positive, Sign::Positive) => BigInteger::from_vec(
                Sign::Positive,
                Self::sub_digits_larger_from_smaller_naive(&lhs.digits, &rhs.digits),
            ),
            (Sign::Positive, Sign::Negative) => BigInteger::from_vec(
                Sign::Positive,
                Self::add_digits_naive(&lhs.digits, &rhs.digits),
            ),
            (Sign::Negative, Sign::Positive) => BigInteger::from_vec(
                Sign::Negative,
                Self::add_digits_naive(&lhs.digits, &rhs.digits),
            ),
            (Sign::Negative, Sign::Negative) => BigInteger::from_vec(
                Sign::Negative,
                Self::sub_digits_larger_from_smaller_naive(&lhs.digits, &rhs.digits),
            ),
        };

        if flipped {
            res.sign = match res.sign {
                Sign::Positive => Sign::Negative,
                Sign::Negative => Sign::Positive,
            };
        }

        res
    }

    pub fn decrement(&mut self) -> Self {
        Self::sub(self, &BigInteger::one())
    }

    pub fn mul(lhs: &Self, rhs: &Self) -> Self {
        if lhs.is_one() {
            return rhs.clone();
        } else if rhs.is_one() {
            return lhs.clone();
        } else if lhs.is_zero() {
            return BigInteger::zero();
        }

        let (lhs, rhs) = if Self::cmp_digits(CompareFunction::Greater, &lhs.digits, &rhs.digits) {
            (lhs, rhs)
        } else {
            (rhs, lhs)
        };

        let digits = Self::mul_digits_naive(&lhs.digits, &rhs.digits);
        let sign = match (lhs.sign, rhs.sign) {
            (Sign::Positive, Sign::Positive) => Sign::Positive,
            (Sign::Negative, Sign::Negative) => Sign::Positive,
            _ => Sign::Negative,
        };

        BigInteger::from_vec(sign, digits)
    }

    pub fn div(lhs: &Self, rhs: &Self) -> Option<(Self, Self)> {
        if rhs.is_one() {
            return Some((lhs.clone(), Self::zero()));
        } else if lhs.is_zero() {
            if rhs.is_zero() {
                return None;
            } else {
                return Some((Self::zero(), Self::zero()));
            }
        }

        let (digits, rem) = if rhs.digits.len() > 1 {
            Self::div_multi_digit_with_reminder_naive(&lhs.digits, &rhs.digits)?
        } else {
            let (digits, rem) = Self::div_by_digit_with_reminder_naive(&lhs.digits, rhs.digit(0))?;
            (digits, vec![rem])
        };

        let sign = match (lhs.sign, rhs.sign) {
            (Sign::Positive, Sign::Positive) => Sign::Positive,
            (Sign::Negative, Sign::Negative) => Sign::Positive,
            _ => Sign::Negative,
        };

        Some((
            BigInteger::from_vec(sign, digits),
            BigInteger::from_vec(lhs.sign, rem),
        ))
    }

    pub fn pow(&self, exp: &Self) -> Result<Self, String> {
        if exp.gt_inner(&BigInteger::from_u64(1000)) {
            return Err("Failsafe: Exponent too large".to_string());
        }

        if exp.is_negative() {
            return Err("Integers not closed under powers with negative exponents.".to_string());
        }

        let mut exp = exp.clone();
        let mut result = BigInteger::one();
        while !exp.is_zero() {
            result = BigInteger::mul(&result, self);
            exp = exp.decrement();
        }

        Ok(result)
    }

    pub fn to_hex_string(&self) -> String {
        let mut s = String::with_capacity(self.digits.len() * 16);
        use std::fmt::Write;

        let mut iter = self.digits.iter().rev();
        for &digit in iter.by_ref() {
            if digit != 0 {
                if s.is_empty() {
                    if self.sign == Sign::Negative {
                        write!(s, "-").unwrap();
                    };
                    write!(s, "0x").unwrap();
                }
                write!(s, "{:x}", digit).unwrap();
                break;
            }
        }

        if s.is_empty() {
            return "0x0".to_string();
        }

        for &n in iter {
            write!(s, "{:016x}", n).unwrap();
        }
        s
    }
}

impl fmt::Display for BigInteger {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let mut digits_str: String = String::new();
        let mut num = self.clone();

        loop {
            if num.eq_inner(&BigInteger::zero()) {
                break;
            }

            let (quotient, remainder) = Self::div(&num, &BigInteger::from_u64(10)).unwrap();

            num = quotient;
            digits_str.insert(0, char::from_digit(remainder.digit(0) as u32, 10).unwrap());
        }

        if digits_str.is_empty() {
            digits_str.push('0');
        }

        if self.sign == Sign::Negative {
            write!(f, "{}{}", self.sign, digits_str)
        } else {
            write!(f, "{}", digits_str)
        }
    }
}

impl fmt::Debug for BigInteger {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_unsigned_int_addition() {
        let a = BigInteger::from_u64(18446744073709551615);
        let b = BigInteger::from_u64(1000);

        let result = BigInteger::add(&a, &b);
        assert_eq!(result.digits, vec![999, 1]);
    }

    #[test]
    fn test_integer_subtraction() {
        let a = BigInteger::from_slice(Sign::Positive, &[9223372036854775809, 2, 1]);
        let b = BigInteger::from_slice(Sign::Positive, &[3, 2, 1]);

        let result = BigInteger::sub(&a, &b);
        assert_eq!(result.digits, vec![9223372036854775806]);
    }

    #[test]
    fn test_int_sub_with_different_signs() {
        let a = BigInteger::from_i64(8);
        let b = BigInteger::from_i64(5);
        let res = BigInteger::sub(&a, &b);
        assert_eq!(res.digits, vec![3]);
        assert_eq!(res.sign, Sign::Positive);

        let a = BigInteger::from_i64(5);
        let b = BigInteger::from_i64(8);
        let res = BigInteger::sub(&a, &b);
        assert_eq!(res.digits, vec![3]);
        assert_eq!(res.sign, Sign::Negative);

        let a = BigInteger::from_i64(-8);
        let b = BigInteger::from_i64(-5);
        let res = BigInteger::sub(&a, &b);
        assert_eq!(res.digits, vec![3]);
        assert_eq!(res.sign, Sign::Negative);

        let a = BigInteger::from_i64(-5);
        let b = BigInteger::from_i64(-8);
        let res = BigInteger::sub(&a, &b);
        assert_eq!(res.digits, vec![3]);
        assert_eq!(res.sign, Sign::Positive);

        let a = BigInteger::from_i64(-8);
        let b = BigInteger::from_i64(5);
        let res = BigInteger::sub(&a, &b);
        assert_eq!(res.digits, vec![13]);
        assert_eq!(res.sign, Sign::Negative);

        let a = BigInteger::from_i64(5);
        let b = BigInteger::from_i64(-8);
        let res = BigInteger::sub(&a, &b);
        assert_eq!(res.digits, vec![13]);
        assert_eq!(res.sign, Sign::Positive);

        let a = BigInteger::from_i64(-5);
        let b = BigInteger::from_i64(8);
        let res = BigInteger::sub(&a, &b);
        assert_eq!(res.digits, vec![13]);
        assert_eq!(res.sign, Sign::Negative);

        let a = BigInteger::from_i64(8);
        let b = BigInteger::from_i64(-5);
        let res = BigInteger::sub(&a, &b);
        assert_eq!(res.digits, vec![13]);
        assert_eq!(res.sign, Sign::Positive);
    }

    #[test]
    fn test_int_add_with_different_signs() {
        let a = BigInteger::from_i64(8);
        let b = BigInteger::from_i64(5);
        let res = BigInteger::add(&a, &b);
        assert_eq!(res.digits, vec![13]);
        assert_eq!(res.sign, Sign::Positive);

        let a = BigInteger::from_i64(5);
        let b = BigInteger::from_i64(8);
        let res = BigInteger::add(&a, &b);
        assert_eq!(res.digits, vec![13]);
        assert_eq!(res.sign, Sign::Positive);

        let a = BigInteger::from_i64(-8);
        let b = BigInteger::from_i64(-5);
        let res = BigInteger::add(&a, &b);
        assert_eq!(res.digits, vec![13]);
        assert_eq!(res.sign, Sign::Negative);

        let a = BigInteger::from_i64(-5);
        let b = BigInteger::from_i64(-8);
        let res = BigInteger::add(&a, &b);
        assert_eq!(res.digits, vec![13]);
        assert_eq!(res.sign, Sign::Negative);

        let a = BigInteger::from_i64(-8);
        let b = BigInteger::from_i64(5);
        let res = BigInteger::add(&a, &b);
        assert_eq!(res.digits, vec![3]);
        assert_eq!(res.sign, Sign::Negative);

        let a = BigInteger::from_i64(5);
        let b = BigInteger::from_i64(-8);
        let res = BigInteger::add(&a, &b);
        assert_eq!(res.digits, vec![3]);
        assert_eq!(res.sign, Sign::Negative);

        let a = BigInteger::from_i64(-5);
        let b = BigInteger::from_i64(8);
        let res = BigInteger::add(&a, &b);
        assert_eq!(res.digits, vec![3]);
        assert_eq!(res.sign, Sign::Positive);

        let a = BigInteger::from_i64(8);
        let b = BigInteger::from_i64(-5);
        let res = BigInteger::add(&a, &b);
        assert_eq!(res.digits, vec![3]);
        assert_eq!(res.sign, Sign::Positive);
    }

    #[test]
    fn test_unsigned_int_multiplication() {
        let a = BigInteger::from_u64(184467440737095516);
        let b = BigInteger::from_u64(23456789101112131);

        let result = BigInteger::mul(&a, &b);
        assert_eq!(result.digits, vec![5714737576593783060, 234567891011121]);
    }

    #[test]
    fn test_div_multi_digit() {
        let a = BigInteger::from_slice(Sign::Positive, &[1, 2, 3, 4, 5]);
        let b = BigInteger::from_slice(Sign::Positive, &[1, 2]);

        let (quotient, remainder) = BigInteger::div(&a, &b).unwrap();

        assert_eq!(
            quotient.digits,
            vec![
                2305843009213693952,
                13835058055282163713,
                9223372036854775808,
                2
            ]
        );
        assert_eq!(remainder.digits, vec![16140901064495857665]);
    }

    #[test]
    fn test_printing_big_integer() {
        let a = BigInteger::from_i64(1234567890123456789);
        let b = BigInteger::from_i64(-987654321098765431);
        let c = BigInteger::from_i64(31589776473858734);

        let r = BigInteger::mul(&BigInteger::mul(&a, &b), &c);

        assert_eq!(
            format!("{}", r),
            "-38518245624879860378241768376646701319896340484039306"
        );
    }

    #[test]
    fn test_big_integer_from_str_radix() {
        let a = BigInteger::from_str_radix("123456789012", 10).unwrap();
        assert_eq!(a.digits, vec![123456789012]);
        assert_eq!(a.sign, Sign::Positive);

        let a = BigInteger::from_str_radix("+123456789012", 10).unwrap();
        assert_eq!(a.digits, vec![123456789012]);
        assert_eq!(a.sign, Sign::Positive);

        let b = BigInteger::from_str_radix("-123456789012", 10).unwrap();
        assert_eq!(b.digits, vec![123456789012]);
        assert_eq!(b.sign, Sign::Negative);

        let a = BigInteger::from_str_radix("123456789012345678901234567890", 10).unwrap();
        assert_eq!(a.digits, vec![14083847773837265618, 6692605942]);
        assert_eq!(a.sign, Sign::Positive);

        let c = BigInteger::from_str_radix("0", 10).unwrap();
        assert_eq!(c.digits, vec![0]);
        assert_eq!(c.sign, Sign::Positive);
    }

    #[test]
    fn test_to_hex_string() {
        let a = BigInteger::from_slice(Sign::Positive, &[0]);
        assert_eq!(a.to_hex_string(), "0x0");

        let a = BigInteger::from_slice(Sign::Positive, &[1, 2]);
        assert_eq!(a.to_hex_string(), "0x20000000000000001");

        let a = BigInteger::from_slice(Sign::Negative, &[1, 2]);
        assert_eq!(a.to_hex_string(), "-0x20000000000000001");

        let a = BigInteger::from_slice(Sign::Positive, &[1, 2, 3]);
        assert_eq!(a.to_hex_string(), "0x300000000000000020000000000000001");
    }

    #[test]
    fn test_comparisons_with_negatives() {
        let n5 = BigInteger::from_i64(-5);
        let n8 = BigInteger::from_i64(-8);
        let p5 = BigInteger::from_i64(5);
        let p8 = BigInteger::from_i64(8);
        let z = BigInteger::from_i64(0);

        assert!(n5.gt_inner(&n8));
        assert!(n8.lt_inner(&n5));

        assert!(n8.lt_inner(&n5));

        assert!(n5.lt_inner(&z));
        assert!(z.lt_inner(&p5));

        assert!(p5.gt_inner(&n5));
        assert!(n5.lt_inner(&p5));

        assert!(p8.ge_inner(&p8));
        assert!(p8.le_inner(&p8));
        assert!(p8.eq_inner(&BigInteger::from_i64(8)));
    }

    #[test]
    fn test_division_zero_cases() {
        let z = BigInteger::from_i64(0);
        let n7 = BigInteger::from_i64(7);
        let mn7 = BigInteger::from_i64(-7);

        let (q, r) = BigInteger::div(&z, &n7).expect("0/7 should succeed");
        assert!(q.eq_inner(&z));
        assert!(r.eq_inner(&z));

        let (q, r) = BigInteger::div(&z, &mn7).expect("0/-7 should succeed");
        assert!(q.eq_inner(&z));
        assert!(r.eq_inner(&z));

        assert!(BigInteger::div(&n7, &z).is_none());
        assert!(BigInteger::div(&z, &z).is_none());
    }

    #[test]
    fn test_division_invariant_qbr_small_set() {
        let cases = [
            (123, 7),
            (123, -7),
            (-123, 7),
            (-123, -7),
            (9223372036854775807i64, 97),
            (-9223372036854775807i64, 97),
            (1000, 1),
            (1000, -1),
        ];

        for (a, b) in cases {
            let a = BigInteger::from_i64(a);
            let b = BigInteger::from_i64(b);

            assert_ne!(b, BigInteger::from_i64(0));

            let (q, r) = BigInteger::div(&a, &b).expect("division failed unexpectedly");

            let qb = BigInteger::mul(&q, &b);
            let rhs = BigInteger::add(&qb, &r);
            assert!(
                rhs.eq_inner(&a),
                "invariant a=q*b+r failed\n a={}\n b={}\n q={}\n r={}\n q*b+r={}",
                a,
                b,
                q,
                r,
                rhs
            );

            // Check |r| < |b| (works for truncating remainder too)
            let ar = r.abs();
            let ab = b.abs();
            assert!(
                ar.lt_inner(&ab) || ar.eq_inner(&BigInteger::from_u64(0)),
                "remainder magnitude too large\n a={}\n b={}\n r={}\n |r|={}\n |b|={}",
                a,
                b,
                r,
                ar,
                ab
            );
        }
    }

    #[test]
    fn test_division_multi_limb_known_results() {
        let a = BigInteger::from_str_radix("18446744073709551616", 10).unwrap();
        let b = BigInteger::from_u64(3);
        let (q, r) = BigInteger::div(&a, &b).unwrap();

        assert!(q.eq_inner(&BigInteger::from_str_radix("6148914691236517205", 10).unwrap()));
        assert!(r.eq_inner(&BigInteger::from_u64(1)));

        let a = BigInteger::from_str_radix("340282366920938463463374607431768211456", 10).unwrap();
        let b = BigInteger::from_str_radix("18446744073709551616", 10).unwrap();
        let (q, r) = BigInteger::div(&a, &b).unwrap();

        assert!(q.eq_inner(&b));
        assert!(r.eq_inner(&BigInteger::from_u64(0)));
    }

    #[test]
    fn test_mul_distributive_small() {
        fn bi(x: i64) -> BigInteger {
            BigInteger::from_i64(x)
        }

        let a = bi(12345);
        let b = bi(678);
        let c = bi(-90);

        let bc = BigInteger::add(&b, &c);
        let left = BigInteger::mul(&a, &bc);

        let ab = BigInteger::mul(&a, &b);
        let ac = BigInteger::mul(&a, &c);
        let right = BigInteger::add(&ab, &ac);

        assert!(left.eq_inner(&right));
    }

    #[test]
    fn test_roundtrip_decimal_parse_display() {
        let inputs = [
            "0",
            "-0",
            "1",
            "-1",
            "10",
            "-10",
            "18446744073709551616",
            "-18446744073709551616",
            "123456789012345678901234567890",
            "-123456789012345678901234567890",
        ];

        for s in inputs {
            let x = BigInteger::from_str_radix(s, 10).unwrap();
            let s2 = format!("{}", x);
            let y = BigInteger::from_str_radix(&s2, 10).unwrap();
            assert!(x.eq_inner(&y), "roundtrip failed: in={}, out={}", s, s2);

            if s == "-0" {
                assert_eq!(s2, "0");
            }
        }
    }

    #[test]
    fn test_no_negative_zero_invariant() {
        let z1 = BigInteger::from_i64(0);
        assert!(z1.is_zero());
        assert!(z1.is_positive());

        let z2 = BigInteger::from_vec(Sign::Negative, vec![0]);
        assert!(z2.is_zero());
        assert!(z2.is_positive(), "zero should always be positive sign");
    }

    #[test]
    fn test_div_mult_limb() {
        let a = BigInteger::from_vec(
            Sign::Positive,
            vec![
                52278, 831776, 700918, 692733, 286021, 246328, 220077, 806682, 374954, 6464,
                697051, 19522, 425920, 585270, 274739,
            ],
        );
        let b = BigInteger::from_vec(
            Sign::Positive,
            vec![70327, 463338, 227530, 519575, 637288, 58611, 314888],
        );

        let (q, r) = BigInteger::div(&a, &b).unwrap();

        // q = BASE^4 => digits [0,0,0,0,1] (5 digits)
        let expected_q = BigInteger::from_str_radix(
            "116982792067798683497881656656520388270385078252689639723\
            9405026943276151096150241294193654711076255544992622749629418\
            1992233312503353989754382825995955612",
            10,
        )
        .unwrap();
        let expected_r = BigInteger::from_str_radix(
            "12302556959215319080439869486705260436435669956771286732638\
            961083170069626250041609128570238081515621916430973969104763314",
            10,
        )
        .unwrap();

        assert!(q.eq_inner(&expected_q), "q was {}", q);
        assert!(r.eq_inner(&expected_r), "r was {}", r);
    }
}
