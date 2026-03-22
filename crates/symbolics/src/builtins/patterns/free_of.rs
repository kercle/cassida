use crate::{
    builtins::{
        BuiltInCategory,
        traits::{BuiltIn, BuiltInDoc, PatternDoc},
    },
    expr::{ExprKind, NormExpr, RawExpr, walk::ExprTopDownWalker},
    pattern::{program::Compiler, runtime::Runtime},
    raw_expr,
};

#[derive(Default)]
pub struct FreeOf;

impl FreeOf {
    pub const HEAD: &'static str = "FreeOf";
}

impl BuiltIn for FreeOf {
    #[inline(always)]
    fn head() -> &'static str {
        Self::HEAD
    }

    fn head_dyn(&self) -> &'static str {
        Self::head()
    }

    fn doc(&self) -> BuiltInDoc {
        BuiltInDoc {
            category: BuiltInCategory::Patterns,
            title: Self::head(),
            summary: "Tests if no sub-expressions of a given subject matches the provided pattern. This is Cassidoids analogue to Mathematicas `FreeQ`.",
            pattern_doc: vec![PatternDoc::new(
                raw_expr!(FreeOf[t_, p_]),
                "Walks over all subexpressions of $t$ and returns `False` if any match if found. Otherwise it returns `True`.",
            )],
            examples: vec![
                ("FreeOf[u^2 (1 - 1 / Exp[v]), x]", "True"),
                ("Free[x^2+1,x]", "False"),
            ],
            related: vec![],
        }
    }

    fn apply_all(&self, expr: NormExpr) -> NormExpr {
        if !expr.is_application_of(Self::HEAD, 2) {
            return expr;
        }

        let ExprKind::Node { args, .. } = expr.into_kind() else {
            unreachable!()
        };

        let [subject, pattern]: [NormExpr; 2] = args.try_into().unwrap();

        let program = Compiler::default().compile(&pattern);

        let contains_pattern = ExprTopDownWalker::new(&subject)
            .find(|s| Runtime::new(&program, s).is_match())
            .is_some();

        if contains_pattern {
            RawExpr::new_symbol("False").normalize()
        } else {
            RawExpr::new_symbol("True").normalize()
        }
    }
}
