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

    fn trim_leading_zeros(&mut self) {
        while let Some(&0) = self.digits.last() {
            self.digits.pop();
        }

        if self.digits.is_empty() {
            self.digits.push(0);
            self.sign = Sign::Positive;
        }
    }

    pub fn eq(&self, other: &Self) -> bool {
        self.sign == other.sign
            && !Self::cmp_digits(CompareFunction::NotEqual, &self.digits, &other.digits)
    }

    pub fn gt(&self, other: &Self) -> bool {
        if self.sign != other.sign {
            return self.sign == Sign::Positive;
        }

        Self::cmp_digits(CompareFunction::Greater, &self.digits, &other.digits)
    }

    pub fn lt(&self, other: &Self) -> bool {
        if self.sign != other.sign {
            return self.sign == Sign::Negative;
        }

        Self::cmp_digits(CompareFunction::Less, &self.digits, &other.digits)
    }

    pub fn abs(&self) -> Self {
        BigInteger::from_vec(Sign::Positive, self.digits.clone())
    }

    pub fn digit(&self, index: usize) -> Digit {
        self.digits.get(index).cloned().unwrap_or_default()
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

        return result;
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

    fn div_multi_digit_with_reminder_naive(
        lhs: &[Digit],
        rhs: &[Digit],
    ) -> Option<(Vec<Digit>, Vec<Digit>)> {
        if !Self::cmp_digits(CompareFunction::NotEqual, lhs, &[0]) {
            return None;
        }

        let mut result = Vec::with_capacity(lhs.len());
        let mut rem: Vec<Digit> = Vec::new();

        for &digit in lhs.iter().rev() {
            rem.insert(0, digit); // Add new digit to the remainder

            let mut quotient = 0;
            while Self::cmp_digits(CompareFunction::GreaterEqual, &rem, rhs) {
                todo!()
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

    pub fn mul(lhs: &Self, rhs: &Self) -> Self {
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
}

impl fmt::Display for BigInteger {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let sign_str = match self.sign {
            Sign::Positive => "",
            Sign::Negative => "-",
        };

        let mut digits_str: String = String::new();
        let mut num = self.clone();

        loop {
            if num.eq(&BigInteger::from_u64(0)) {
                break;
            }

            let (quotient, remainder) = Self::div(&num, &BigInteger::from_u64(10)).unwrap();

            num = quotient;
            digits_str.insert(0, char::from_digit(remainder.digit(0) as u32, 10).unwrap());
        }

        write!(f, "{}{}", sign_str, digits_str)
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

        dbg!(&quotient.digits);

        assert_eq!(quotient.digits, vec![1, 2]);
        assert_eq!(remainder.digits, vec![3, 4, 5]);
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
}
