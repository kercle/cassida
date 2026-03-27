#[macro_export]
macro_rules! raw_expr {
    ($($tt:tt)*) => {
        $crate::expr::RawExpr::from($crate::_parser_macros::parse!($($tt)*))
    };
}

#[macro_export]
macro_rules! norm_expr {
    ($($tt:tt)*) => {
        $crate::raw_expr!($($tt)*).normalize()
    };
}

#[macro_export]
macro_rules! chain_replace_quick_and_dirty {
    // allow trailing comma
    ($expr:expr, $({ $($pat:tt)* } => { $($rep:tt)* }),+ $(,)?) => {{
        let mut __e = $expr;
        $(
            __e = __e.replace_all_quick_and_dirty(
                norm_expr! { $($pat)* },
                expr! { $($rep)* },
            );
        )+
        __e
    }};
}

#[macro_export]
macro_rules! rules {
    // entry: takes an identifier that is the rewriter expr (e.g. `rw;`)
    ($rw:expr; $(($lhs:tt) => $rhs:expr;)+) => {{
        let mut r = $rw;
        $(
            r = r.with_rule(
                norm_expr! { $lhs },
                |ctx| ctx.fill($rhs),
            );
        )+
        r
    }};
}

#[macro_export]
macro_rules! timed_eprintln {
    ($($arg:tt)*) => {{
        use std::sync::OnceLock;
        use std::time::Instant;
        static START: OnceLock<Instant> = OnceLock::new();
        let start = START.get_or_init(Instant::now);
        let elapsed = start.elapsed().as_millis();
        eprintln!("[{elapsed}ms] {}", format_args!($($arg)*));
    }};
}

#[macro_export]
macro_rules! ensure {
    ($cond:expr, $err:expr) => {
        if !$cond {
            return Err($err);
        }
    };
    ($cond:expr) => {
        if !$cond {
            return None;
        }
    };
}
