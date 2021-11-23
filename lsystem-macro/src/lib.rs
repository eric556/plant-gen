#![feature(proc_macro_diagnostic)]
use proc_macro::TokenStream;
use syn::{Ident, Lit, Token, braced, parse::{Parse, ParseStream}, parse_macro_input, punctuated::Punctuated};
use quote::{quote};

/// Parses out a single rule from the rule defenitions 
/// 
///     F => "F+G"
///
#[derive(Clone)]
struct Rule {
    constant: Ident,
    rule_body: Lit
}

impl Parse for Rule {
    fn parse(input: syn::parse::ParseStream) -> syn::Result<Self> {
        let constant: Ident = input.parse()?;
        input.parse::<Token![=>]>()?;
        let body: Lit = input.parse()?;

        Ok(Rule {
            constant: constant,
            rule_body: body
        })
    }
}

struct LSystem {
    name: Ident,
    axiom: Lit,
    rules: Vec<Rule>,
    iteration_count: Lit
}

impl Parse for LSystem {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        let name: Ident = input.parse()?;
        input.parse::<Token![,]>()?;
        let axiom: Lit = input.parse()?;
        input.parse::<Token![,]>()?;
        let content;
        let _ = braced!(content in input);
        let rules: Vec<Rule> = (Punctuated::<Rule, Token![,]>::parse_terminated(&content)?).into_iter().collect();
        input.parse::<Token![,]>()?;
        let iteration_count: Lit = input.parse()?;

        Ok(LSystem {
            name: name,
            axiom: axiom,
            rules: rules,
            iteration_count: iteration_count,
        })
    }
}

fn parse_string(string_lit: &Lit, span: &proc_macro2::Span) -> String {
    return match string_lit {
        Lit::Str(string_litstr) => {
            string_litstr.value()
        },
        _ => {
            span.unwrap().error("Expected string").emit();
            String::from("")
        }
    };
}

fn parse_int(int_lit: &Lit, span: &proc_macro2::Span) -> u32 {
    return match int_lit {
        Lit::Int(int_litint) => {
            int_litint.base10_parse().unwrap()
        },
        _ => {
            span.unwrap().error("Expected u32").emit();
            0u32
        }
    };
}

#[proc_macro]
pub fn l_system(input: TokenStream) -> TokenStream {
    let LSystem {
        name,
        axiom,
        rules,
        iteration_count
    } = parse_macro_input!(input as LSystem);

    let axiom_str: String = parse_string(&axiom, &axiom.span());
    let iteration_count_int = parse_int(&iteration_count, &iteration_count.span());
    let axiom_vec: Vec<char> = axiom_str.chars().collect(); // This is the vec thats going to be expanded using the rules

    // pre process the vec of rules
    let mut processed_rules = vec![];
    for rule in rules {
        let ident_str = rule.constant.to_string();
        if ident_str.len() > 1 {
            rule.constant.span().unwrap().error("Constant greater than 1");
            panic!();
        }
        let rule_body: Vec<char> = parse_string(&rule.rule_body, &rule.rule_body.span()).chars().collect();
        processed_rules.push((ident_str.chars().next().unwrap(), rule_body));
    }

    let mut expanded_axiom: Vec<char> = axiom_vec;
    for _ in 0..iteration_count_int {
        let mut temp = vec![];
        for c in expanded_axiom {
            let mut found = false;
            for rule in &processed_rules {
                if rule.0 == c {
                    temp.extend(rule.1.iter());
                    found = true;
                }
            }
            if !found {
                temp.push(c);
            }
        }
        expanded_axiom = temp;
    }

    let expanded_axiom_array_tokens: Vec<proc_macro2::TokenStream> = expanded_axiom.into_iter().map(|letter| {
        quote!{#letter,}
    }).collect();
    
    let expanded = quote!{
        macro_rules! #name {
            () => {
                [
                    #(#expanded_axiom_array_tokens)*
                ]
            };
        }
    };

    return expanded.into();
}