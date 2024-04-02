extern crate proc_macro;

use proc_macro::TokenStream;
use quote::{quote, ToTokens};
use syn::{
    braced,
    parse::{Parse, ParseStream},
    parse_macro_input, token, Expr, Token,
};

#[derive(Debug, PartialEq)]
struct DeclExpr {
    key: Expr,
    value: Expr,
}

#[derive(Debug, PartialEq)]
struct RuleExpr {
    selector: Expr,
    nodes: Vec<AstNodeExpr>,
}

#[derive(Debug, PartialEq)]
enum AstNodeExpr {
    Decl(DeclExpr),
    Rule(RuleExpr),
}

impl ToTokens for AstNodeExpr {
    fn to_tokens(&self, tokens: &mut proc_macro2::TokenStream) {
        match self {
            AstNodeExpr::Decl(decl) => {
                let key = &decl.key;
                let value = &decl.value;
                tokens.extend(quote! {
                    crate::css::AstNode::decl(lightningcss::values::string::CowArcStr::from(#key), lightningcss::values::string::CowArcStr::from(#value))
                });
            }
            AstNodeExpr::Rule(rule) => {
                let selector = &rule.selector;
                let nodes = &rule.nodes;
                tokens.extend(quote! {
                    crate::css::AstNode::rule(#selector.into(), vec![#(#nodes),*])
                });
            }
        }
    }
}

#[derive(Debug)]
struct MyMacroInput {
    css: Vec<AstNodeExpr>,
}

fn parse_recursive(input: ParseStream) -> Result<Vec<AstNodeExpr>, syn::Error> {
    let mut nodes = vec![];
    while !input.is_empty() {
        let key: Expr = input.parse()?;
        if input.peek(token::Brace) {
            let content;
            let _ = braced!(content in input);
            nodes.push(AstNodeExpr::Rule(RuleExpr {
                selector: key,
                nodes: parse_recursive(&content)?,
            }));
            continue;
        } else {
            input.parse::<Token![:]>()?;
            let value: Expr = input.parse()?;
            if !input.is_empty() {
                input.parse::<Token![;]>()?;
            }
            nodes.push(AstNodeExpr::Decl(DeclExpr { key, value }));
        }
    }
    Ok(nodes)
}

impl Parse for MyMacroInput {
    fn parse(input: ParseStream) -> Result<Self, syn::Error> {
        Ok(MyMacroInput {
            css: parse_recursive(input)?,
        })
    }
}

#[proc_macro]
pub fn css(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as MyMacroInput);
    let generated_code = input.css.iter().map(ToTokens::to_token_stream);

    TokenStream::from(quote! {
        vec![#(#generated_code),*]
    })
}
