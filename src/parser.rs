use std::collections::HashMap;

pub struct Token<'a> {
  pub raw: &'a str,
  pub value: &'a str,
  pub modifier: Vec<&'a str>,
}

pub struct Parser<'a> {
    tokens: Vec<Token<'a>>,
    cache: HashMap<&'a str, Token<'a>>
}