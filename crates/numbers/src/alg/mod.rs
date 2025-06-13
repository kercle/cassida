use crate::integer::BigInteger;

pub fn gcd(a: BigInteger, b: BigInteger) -> BigInteger {
    let mut a = a;
    let mut b = b;

    while !b.is_zero() {
        let temp = b.clone();
        if let Some(rem) = a % b {
            b = rem;
        } else {
            return BigInteger::from_u64(1);
        }
        a = temp;
    }

    a
}
