use std::ops;

use crate::expr::Expr;

pub trait ExprBuilder {
    fn build(&self) -> Expr<()>;
}

#[derive(Debug, Clone, Copy)]
pub struct SymbolGenerator {
    name: &'static str,
}

impl SymbolGenerator {
    pub fn new(name: &'static str) -> SymbolGenerator {
        SymbolGenerator {
            name: name.as_ref(),
        }
    }
}

impl ExprBuilder for SymbolGenerator {
    fn build(&self) -> Expr {
        Expr::new_symbol(&self.name)
    }
}

pub fn f() -> Expr<()> {
    Expr::new_compound(Expr::new_symbol("f"), Vec::new())
}

pub fn g(a: Expr) -> Expr<()> {
    Expr::new_compound(Expr::new_symbol("f"), vec![a])
}

pub fn h(a: Expr, b: Expr) -> Expr<()> {
    Expr::new_compound(Expr::new_symbol("f"), vec![a, b])
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

pub fn blank(head: Option<Expr>) -> Expr<()> {
    Expr::new_compound(
        Expr::new_symbol("Head"),
        if let Some(h) = head {
            vec![h]
        } else {
            Vec::new()
        },
    )
}

pub fn blank_sequence(head: Option<Expr>) -> Expr<()> {
    Expr::new_compound(
        Expr::new_symbol("HeadSeq"),
        if let Some(h) = head {
            vec![h]
        } else {
            Vec::new()
        },
    )
}

pub fn pattern(bind_name: &str, arg: Expr) -> Expr<()> {
    Expr::new_compound(
        Expr::new_symbol("Pattern"),
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
