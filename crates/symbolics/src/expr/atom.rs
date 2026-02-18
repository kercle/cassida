use std::cmp::Ordering;

use numbers::Number;

#[derive(Clone, PartialEq)]
pub enum Atom {
    Number(Number),
    Symbol(String),
    StringLiteral(String),
}

impl Atom {
    fn rank(&self) -> u8 {
        match self {
            Atom::Number(_) => 0,
            Atom::Symbol(_) => 1,
            Atom::StringLiteral(_) => 2,
        }
    }

    fn cmp_atom(&self, other: &Self) -> Ordering {
        match (self, other) {
            (Atom::Number(a), Atom::Number(b)) => a.cmp(b),
            (Atom::Symbol(a), Atom::Symbol(b)) => a.cmp(b),
            _ => {
                let r = self.rank().cmp(&other.rank());
                if r.is_eq() {
                    unreachable!("If atoms are of same type, cmp must be handled explicitly")
                } else {
                    r
                }
            }
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
