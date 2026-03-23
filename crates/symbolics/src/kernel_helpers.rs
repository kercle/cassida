use crate::{builtins, kernel::Kernel};

impl Kernel {
    pub(super) fn register_initial_builtins(&mut self) {
        // --- Calculus -------------------------------------------------------

        self.register_builtin_default::<builtins::Integrate>(true);
        self.register_builtin_default::<builtins::Derivative>(true);

        // --- Elementary Arithmetic ------------------------------------------

        self.register_builtin_default::<builtins::Add>(false);
        self.register_builtin_default::<builtins::Sub>(false);

        self.register_builtin_default::<builtins::Mul>(false);
        self.register_builtin_default::<builtins::Div>(false);
        self.register_builtin_default::<builtins::Neg>(false);

        self.register_builtin_default::<builtins::Pow>(false);
        self.register_builtin_default::<builtins::Factorial>(false);

        // --- Elementary Functions -------------------------------------------

        self.register_builtin_default::<builtins::Sqrt>(false);

        self.register_builtin_default::<builtins::Cos>(false);
        self.register_builtin_default::<builtins::Sin>(false);
        self.register_builtin_default::<builtins::Tan>(false);

        self.register_builtin_default::<builtins::Exp>(false);
        self.register_builtin_default::<builtins::Log>(false);

        // --- Evaluation control ---------------------------------------------

        self.register_builtin_default::<builtins::Hold>(false);
        self.register_builtin_default::<builtins::HoldPattern>(false);

        // --- Patterns -------------------------------------------------------

        self.register_builtin_default::<builtins::Optional>(false);
        self.register_builtin_default::<builtins::FreeOf>(false);

        // --- Relational -----------------------------------------------------

        self.register_builtin_default::<builtins::Equal>(false);
        self.register_builtin_default::<builtins::Greater>(false);
        self.register_builtin_default::<builtins::GreaterEqual>(false);
        self.register_builtin_default::<builtins::Less>(false);
        self.register_builtin_default::<builtins::LessEqual>(false);

        // --- Scoping --------------------------------------------------------

        self.register_builtin_default::<builtins::Compound>(false);

        // --- Simplification -------------------------------------------------

        self.register_builtin_default::<builtins::Simplify>(true);
        self.register_builtin(
            Box::new(builtins::Expand::new(self.binomial_generator())),
            true,
        );

        // --- System ---------------------------------------------------------

        self.register_builtin_default::<builtins::Help>(false);
    }
}
