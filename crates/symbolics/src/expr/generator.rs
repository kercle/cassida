// TODO: Deprecate this file in favor of proc-macro raw_expr! { .. }

use std::ops;

use crate::{
    expr::Expr,
    pattern::{BLANK_ONE_HEAD, BLANK_SEQ_HEAD, PATTERN_HEAD},
};

pub trait ExprBuilder {
    fn build(&self) -> Expr<()>;
}

#[derive(Debug, Clone, Copy)]
pub struct SymbolGenerator {
    name: &'static str,
}

impl SymbolGenerator {
    pub fn new(name: &'static str) -> SymbolGenerator {
        SymbolGenerator { name }
    }
}

impl ExprBuilder for SymbolGenerator {
    fn build(&self) -> Expr {
        Expr::new_symbol(self.name)
    }
}

impl From<SymbolGenerator> for Expr {
    fn from(s: SymbolGenerator) -> Self {
        s.build()
    }
}

pub fn f() -> Expr<()> {
    Expr::new_compound(Expr::new_symbol("f"), Vec::new())
}

pub fn g<T: Into<Expr>>(a: T) -> Expr<()> {
    Expr::new_compound(Expr::new_symbol("f"), vec![a.into()])
}

pub fn h<S: Into<Expr>, T: Into<Expr>>(a: S, b: T) -> Expr<()> {
    Expr::new_compound(Expr::new_symbol("f"), vec![a.into(), b.into()])
}

pub fn cos(a: Expr) -> Expr<()> {
    Expr::new_compound(Expr::new_symbol("Cos"), vec![a])
}

pub fn sin(a: Expr) -> Expr<()> {
    Expr::new_compound(Expr::new_symbol("Sin"), vec![a])
}

pub fn exp(a: Expr) -> Expr<()> {
    Expr::new_compound(Expr::new_symbol("Exp"), vec![a])
}

pub fn log(a: Expr) -> Expr<()> {
    Expr::new_compound(Expr::new_symbol("Log"), vec![a])
}

pub fn pow<A, S: Into<Expr<A>>, T: Into<Expr<A>>>(b: S, e: T) -> Expr<A>
where
    A: Default + Clone + PartialEq,
{
    Expr::new_compound(Expr::new_symbol("Pow"), vec![b.into(), e.into()])
}

pub fn blank<A: Default + Clone + PartialEq>(head: Option<Expr<A>>) -> Expr<A> {
    Expr::new_compound(
        Expr::new_symbol(BLANK_ONE_HEAD),
        if let Some(h) = head {
            vec![h]
        } else {
            Vec::new()
        },
    )
}

pub fn blank_sequence(head: Option<Expr>) -> Expr<()> {
    Expr::new_compound(
        Expr::new_symbol(BLANK_SEQ_HEAD),
        if let Some(h) = head {
            vec![h]
        } else {
            Vec::new()
        },
    )
}

pub fn pattern(bind_name: &str, arg: Expr) -> Expr<()> {
    Expr::new_compound(
        Expr::new_symbol(PATTERN_HEAD),
        vec![Expr::new_symbol(bind_name), arg],
    )
}

///////////////
// Operators //
///////////////

impl ops::Add for SymbolGenerator {
    type Output = Expr;

    fn add(self, other: Self) -> Self::Output {
        self.build() + other.build()
    }
}

impl ops::Add<SymbolGenerator> for Expr {
    type Output = Expr;

    fn add(self, other: SymbolGenerator) -> Self::Output {
        self + other.build()
    }
}

impl ops::Add<Expr> for SymbolGenerator {
    type Output = Expr;

    fn add(self, other: Expr) -> Self::Output {
        self.build() + other
    }
}

impl ops::Add<SymbolGenerator> for i32 {
    type Output = Expr;

    fn add(self, other: SymbolGenerator) -> Self::Output {
        self + other.build()
    }
}

impl ops::Add<i32> for SymbolGenerator {
    type Output = Expr;

    fn add(self, other: i32) -> Self::Output {
        self.build() + other
    }
}

impl ops::Mul for SymbolGenerator {
    type Output = Expr;

    fn mul(self, other: Self) -> Self::Output {
        self.build() * other.build()
    }
}

impl ops::Mul<SymbolGenerator> for Expr {
    type Output = Expr;

    fn mul(self, other: SymbolGenerator) -> Self::Output {
        self * other.build()
    }
}

impl ops::Mul<Expr> for SymbolGenerator {
    type Output = Expr;

    fn mul(self, other: Expr) -> Self::Output {
        self.build() * other
    }
}

impl ops::Mul<SymbolGenerator> for i32 {
    type Output = Expr;

    fn mul(self, other: SymbolGenerator) -> Self::Output {
        self * other.build()
    }
}

impl ops::Mul<i32> for SymbolGenerator {
    type Output = Expr;

    fn mul(self, other: i32) -> Self::Output {
        self.build() * other
    }
}
