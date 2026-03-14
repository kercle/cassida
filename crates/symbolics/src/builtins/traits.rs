use crate::expr::NormExpr;

#[derive(Clone, Debug)]
pub struct BuiltInDoc {
    pub category: &'static str,
    pub title: &'static str,
    pub summary: &'static str,
    pub pattern_doc: Vec<PatternDoc>,
    pub examples: Vec<(&'static str, &'static str)>,
    pub related: Vec<&'static str>,
}

#[derive(Clone, Debug)]
pub struct PatternDoc {
    pub pattern: String,
    pub summary: String,
}

impl PatternDoc {
    pub fn new<S: ToString, T: ToString>(pattern: S, summary: T) -> Self {
        Self {
            pattern: pattern.to_string(),
            summary: summary.to_string(),
        }
    }
}

pub trait BuiltIn {
    fn head_symbol(&self) -> &'static str;

    fn doc(&self) -> BuiltInDoc;

    fn apply_all(&self, expr: NormExpr) -> NormExpr {
        expr
    }
}
