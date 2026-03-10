use numbers::Number;
use parser::ast::ParserAst;
use proc_macro::TokenStream;
use quote::quote;

#[proc_macro]
pub fn tcas_parse(input: TokenStream) -> TokenStream {
    let input_string = token_stream_to_string(input.into());

    match parser::parse(&input_string) {
        Ok(ast) => ast_to_token_stream(ast).into(),
        Err(e) => {
            let msg = e.to_string();
            quote! { compile_error!(#msg) }.into()
        }
    }
}

fn token_stream_to_string(input: proc_macro2::TokenStream) -> String {
    let mut result = String::new();
    let mut prev_end: Option<proc_macro2::LineColumn> = None;

    for tt in input {
        let span = tt.span();
        let start = span.start();

        let adjacent = prev_end
            .map(|end| end.line == start.line && end.column == start.column)
            .unwrap_or(false);

        if !adjacent && !result.is_empty() {
            result.push(' ');
        }

        result.push_str(&tt.to_string());
        prev_end = Some(span.end());
    }

    result
}

fn ast_to_token_stream(ast: ParserAst) -> proc_macro2::TokenStream {
    let ast_path = quote! { parser::ast::ParserAst };
    let number_path = quote! { parser::_numbers::Number };

    use ParserAst::*;
    match ast {
        Constant {
            value: Number::Integer(num),
        } => {
            let str_num = num.to_string();
            quote! {
                #ast_path::new_constant(#number_path::new_integer_from_str(#str_num).unwrap())
            }
        }
        Constant {
            value: Number::Rational(_),
        } => unimplemented!(),
        Symbol { name } => quote! { #ast_path::new_symbol(#name) },
        LesserThan { lhs, rhs } => {
            let lhs = ast_to_token_stream(*lhs);
            let rhs = ast_to_token_stream(*rhs);
            quote! { #ast_path::new_lt(#lhs, #rhs) }
        }
        LesserEq { lhs, rhs } => {
            let lhs = ast_to_token_stream(*lhs);
            let rhs = ast_to_token_stream(*rhs);
            quote! { #ast_path::new_le(#lhs, #rhs) }
        }
        Equals { lhs, rhs } => {
            let lhs = ast_to_token_stream(*lhs);
            let rhs = ast_to_token_stream(*rhs);
            quote! { #ast_path::new_eq(#lhs, #rhs) }
        }
        GreaterEq { lhs, rhs } => {
            let lhs = ast_to_token_stream(*lhs);
            let rhs = ast_to_token_stream(*rhs);
            quote! { #ast_path::new_ge(#lhs, #rhs) }
        }
        GreaterThan { lhs, rhs } => {
            let lhs = ast_to_token_stream(*lhs);
            let rhs = ast_to_token_stream(*rhs);
            quote! { #ast_path::new_gt(#lhs, #rhs) }
        }
        Add { lhs, rhs } => {
            let lhs = ast_to_token_stream(*lhs);
            let rhs = ast_to_token_stream(*rhs);
            quote! { #ast_path::new_add(#lhs, #rhs) }
        }
        Sub { lhs, rhs } => {
            let lhs = ast_to_token_stream(*lhs);
            let rhs = ast_to_token_stream(*rhs);
            quote! { #ast_path::new_sub(#lhs, #rhs) }
        }
        Negation { arg } => {
            let arg = ast_to_token_stream(*arg);
            quote! { #ast_path::new_negation(#arg) }
        }
        Mul { lhs, rhs } => {
            let lhs = ast_to_token_stream(*lhs);
            let rhs = ast_to_token_stream(*rhs);
            quote! { #ast_path::new_mul(#lhs, #rhs) }
        }
        Div { lhs, rhs } => {
            let lhs = ast_to_token_stream(*lhs);
            let rhs = ast_to_token_stream(*rhs);
            quote! { #ast_path::new_div(#lhs, #rhs) }
        }
        Pow { lhs, rhs } => {
            let lhs = ast_to_token_stream(*lhs);
            let rhs = ast_to_token_stream(*rhs);
            quote! { #ast_path::new_pow(#lhs, #rhs) }
        }
        FunctionCall { name, args } => {
            let args: Vec<proc_macro2::TokenStream> =
                args.into_iter().map(ast_to_token_stream).collect();

            quote! { #ast_path::new_function_call(#name, vec![#(#args),*]) }
        }
        Blank {
            bind_name,
            head_constraint,
            optional,
        } => {
            let bind_name_ts = optional_string_to_token_stream(bind_name);
            let head_constraint_ts = optional_string_to_token_stream(head_constraint);
            quote! { #ast_path::new_blank(#bind_name_ts, #head_constraint_ts, #optional) }
        }
        BlankSeq {
            bind_name,
            head_constraint,
            optional,
        } => {
            let bind_name_ts = optional_string_to_token_stream(bind_name);
            let head_constraint_ts = optional_string_to_token_stream(head_constraint);
            quote! { #ast_path::new_blank_seq(#bind_name_ts, #head_constraint_ts, #optional) }
        }
        BlankNullSeq {
            bind_name,
            head_constraint,
            optional,
        } => {
            let bind_name_ts = optional_string_to_token_stream(bind_name);
            let head_constraint_ts = optional_string_to_token_stream(head_constraint);
            quote! { #ast_path::new_blank_null_seq(#bind_name_ts, #head_constraint_ts, #optional) }
        }
        Block { .. } => todo!(),
    }
}

fn optional_string_to_token_stream(os: Option<String>) -> proc_macro2::TokenStream {
    if let Some(s) = os {
        quote! { Some(#s.to_string()) }
    } else {
        quote! { None }
    }
}
