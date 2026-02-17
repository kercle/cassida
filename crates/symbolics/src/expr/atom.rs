use std::cmp::Ordering;

use numbers::Number;

#[derive(Clone, PartialEq)]
pub enum Atom {
    Number(Number),
    Symbol(String),
}

impl Atom {
    fn rank(&self) -> u8 {
        match self {
            Atom::Number(_) => 0,
            Atom::Symbol(_) => 1,
        }
    }

    fn cmp_atom(&self, other: &Self) -> Ordering {
        match self.rank().cmp(&other.rank()) {
            Ordering::Equal => {}
            ord => return ord,
        }

        match (self, other) {
            (Atom::Number(a), Atom::Number(b)) => {
                // Assumes Number supports partial_cmp; if it's total, even better.
                // If Number is floating, NaN would make this None, so you may want a total wrapper.
                a.partial_cmp(b).unwrap_or(Ordering::Equal)
            }
            (Atom::Symbol(a), Atom::Symbol(b)) => a.cmp(b),
            _ => Ordering::Equal, // unreachable due to rank check
        }
    }
}

impl From<&str> for Atom {
    fn from(s: &str) -> Self {
        Atom::Symbol(s.to_string())
    }
}

impl From<String> for Atom {
    fn from(s: String) -> Self {
        Atom::Symbol(s)
    }
}

impl From<i64> for Atom {
    fn from(n: i64) -> Self {
        Atom::Number(Number::from_i64(n))
    }
}

impl From<i32> for Atom {
    fn from(n: i32) -> Self {
        Atom::Number(Number::from_i64(n as i64))
    }
}

impl PartialOrd for Atom {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for Atom {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.cmp_atom(other)
    }
}

impl Eq for Atom {}
