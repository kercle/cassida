// expr-macro/src/lib.rs
use proc_macro::TokenStream;
use quote::quote;
use syn::{
    Ident, LitInt, LitStr, Result, Token,
    parse::{Parse, ParseStream},
    parse_macro_input, token,
};

/// Example:
/// raw_expr!{ 1 + f[1 + Cos[x]] }
#[proc_macro]
pub fn expr(input: TokenStream) -> TokenStream {
    let parsed = parse_macro_input!(input as ExprRoot);
    TokenStream::from(parsed.expand())
}

#[proc_macro]
pub fn norm_expr(input: TokenStream) -> TokenStream {
    let parsed = parse_macro_input!(input as ExprRoot);

    let expr_tokens = parsed.expand();
    TokenStream::from(quote! {
        NormalizedExpr::new(#expr_tokens)
    })
}

#[proc_macro]
pub fn pattern(input: TokenStream) -> TokenStream {
    let parsed = parse_macro_input!(input as ExprRoot);

    let expr_tokens = parsed.expand();
    TokenStream::from(quote! {
        Pattern::from_expr(#expr_tokens)
    })
}

struct ExprRoot {
    expr: Ast,
}

impl Parse for ExprRoot {
    fn parse(input: ParseStream) -> Result<Self> {
        let expr = parse_expression(input)?;
        if !input.is_empty() {
            return Err(input.error("unexpected tokens after expression"));
        }
        Ok(Self { expr })
    }
}

#[derive(Clone)]
enum Ast {
    Atom(AtomAst),
    Call { head: String, args: Vec<Ast> },
}

#[derive(Clone)]
enum AtomAst {
    Number(NumberLit),
    Symbol(String),
    StringLiteral(String),
}

#[derive(Clone)]
enum NumberLit {
    Int(LitInt),
}

fn parse_expression(input: ParseStream) -> Result<Ast> {
    parse_sum(input)
}

// <sum> ::= <product> { ("+"|"-") <product> }*
fn parse_sum(input: ParseStream) -> Result<Ast> {
    let mut node = parse_product(input)?;

    while input.peek(Token![+]) || input.peek(Token![-]) {
        if input.peek(Token![+]) {
            input.parse::<Token![+]>()?;
            let rhs = parse_product(input)?;
            node = Ast::Call {
                head: "Add".to_string(),
                args: vec![node, rhs],
            };
        } else {
            input.parse::<Token![-]>()?;
            let rhs = parse_product(input)?;
            node = Ast::Call {
                head: "Sub".to_string(),
                args: vec![node, rhs],
            };
        }
    }

    Ok(node)
}

// <product> ::= <signed_power> { ("*"|"/") <signed_power> }*
fn parse_product(input: ParseStream) -> Result<Ast> {
    let mut node = parse_signed_power(input)?;

    while input.peek(Token![*]) || input.peek(Token![/]) {
        if input.peek(Token![*]) {
            input.parse::<Token![*]>()?;
            let rhs = parse_signed_power(input)?;
            node = Ast::Call {
                head: "Mul".to_string(),
                args: vec![node, rhs],
            };
        } else {
            input.parse::<Token![/]>()?;
            let rhs = parse_signed_power(input)?;
            node = Ast::Call {
                head: "Div".to_string(),
                args: vec![node, rhs],
            };
        }
    }

    Ok(node)
}

// <signed_power> ::= { "+" | "-" }* <power>
fn parse_signed_power(input: ParseStream) -> Result<Ast> {
    let mut neg = false;
    while input.peek(Token![+]) || input.peek(Token![-]) {
        if input.peek(Token![+]) {
            input.parse::<Token![+]>()?;
        } else {
            input.parse::<Token![-]>()?;
            neg = !neg;
        }
    }

    let p = parse_power(input)?;

    if !neg {
        Ok(p)
    } else {
        Ok(Ast::Call {
            head: "Neg".to_string(),
            args: vec![p],
        })
    }
}

// <power> ::= <atom> { "^" <power> }   (right associative)
fn parse_power(input: ParseStream) -> Result<Ast> {
    let base = parse_atom(input)?;
    if input.peek(Token![^]) {
        input.parse::<Token![^]>()?;
        let exp = parse_power(input)?;
        Ok(Ast::Call {
            head: "Pow".to_string(),
            args: vec![base, exp],
        })
    } else {
        Ok(base)
    }
}

// <atom> ::= <number> | <named_value_or_function_call> | <string> | "(" <sum> ")"
fn parse_atom(input: ParseStream) -> Result<Ast> {
    // number
    if input.peek(LitInt) {
        let lit = input.parse::<LitInt>()?;
        return Ok(Ast::Atom(AtomAst::Number(NumberLit::Int(lit))));
    }

    // string literal
    if input.peek(LitStr) {
        let s = input.parse::<LitStr>()?;
        return Ok(Ast::Atom(AtomAst::StringLiteral(s.value())));
    }

    // <named_value_or_function_call>
    if input.peek(Ident) {
        return parse_named_value_or_call(input);
    }

    // "(" <sum> ")"
    if input.peek(syn::token::Paren) {
        let content;
        syn::parenthesized!(content in input);

        let expr = parse_sum(&content)?;

        if !content.is_empty() {
            return Err(content.error("unexpected tokens inside parentheses"));
        }

        return Ok(expr);
    }
    Err(input.error("expected number, string literal, identifier, or parenthesized expression"))
}

// <named_value_or_function_call> ::= <identifier>
//    | <identifier> "[" "]"
//    | <identifier> "[" <expression> { "," <expression> }* "]"
fn parse_named_value_or_call(input: ParseStream) -> Result<Ast> {
    let ident = input.parse::<Ident>()?;
    let name = ident.to_string();

    if !input.peek(token::Bracket) {
        return Ok(Ast::Atom(AtomAst::Symbol(name)));
    }

    let content;
    syn::bracketed!(content in input);

    if content.is_empty() {
        return Ok(Ast::Call {
            head: name,
            args: vec![],
        });
    }

    let mut args = Vec::new();
    args.push(parse_expression(&content)?);
    while content.peek(Token![,]) {
        content.parse::<Token![,]>()?;
        args.push(parse_expression(&content)?);
    }

    if !content.is_empty() {
        return Err(content.error("unexpected tokens inside [...]"));
    }

    Ok(Ast::Call { head: name, args })
}

// --------------------------- Codegen ---------------------------

impl ExprRoot {
    fn expand(&self) -> proc_macro2::TokenStream {
        self.expr.to_tokens_expr()
    }
}

impl Ast {
    fn to_tokens_expr(&self) -> proc_macro2::TokenStream {
        match self {
            Ast::Atom(a) => a.to_tokens_expr_atom(),
            Ast::Call { head, args } => {
                let head_expr = symbol_expr(head);
                let args_exprs = args.iter().map(|a| a.to_tokens_expr());
                quote! {
                    Expr::Compound {
                        head: ::std::boxed::Box::new(#head_expr),
                        args: ::std::vec![#(#args_exprs),*],
                        annotation: (),
                    }
                }
            }
        }
    }
}

impl AtomAst {
    fn to_tokens_expr_atom(&self) -> proc_macro2::TokenStream {
        match self {
            AtomAst::Number(n) => {
                let number_expr = n.to_tokens_number();
                quote! {
                    Expr::Atom {
                        entry: Atom::Number(#number_expr),
                        annotation: (),
                    }
                }
            }
            AtomAst::Symbol(s) => {
                quote! {
                    Expr::Atom {
                        entry: Atom::Symbol(::std::string::String::from(#s)),
                        annotation: (),
                    }
                }
            }
            AtomAst::StringLiteral(s) => {
                quote! {
                    Expr::Atom {
                        entry: Atom::StringLiteral(::std::string::String::from(#s)),
                        annotation: (),
                    }
                }
            }
        }
    }
}

impl NumberLit {
    fn to_tokens_number(&self) -> proc_macro2::TokenStream {
        // Assumption: your `Number` type is `::numbers::Number`.
        // If your real path differs, change it here.
        match self {
            NumberLit::Int(lit) => quote! {
                <::numbers::Number as ::core::convert::From<i128>>::from(#lit as i128)
            },
        }
    }
}

fn symbol_expr<S: AsRef<str>>(s: S) -> proc_macro2::TokenStream {
    let s = s.as_ref();
    quote! {
        Expr::Atom {
            entry: Atom::Symbol(::std::string::String::from(#s)),
            annotation: (),
        }
    }
}
