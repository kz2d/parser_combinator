use std::collections::HashMap;

#[derive(Debug, PartialEq, Clone)]
pub enum AstNode {
	String(String),
	Number(f64),
	Object(HashMap<String, AstNode>),
	Null
}

#[derive(Debug,PartialEq, Clone)]
pub struct Error(String);

impl Error {
	pub fn new(a: &str) -> Error {
		Error(a.to_string())
	}
}

pub type Parser<Out> = Box<dyn for<'a> Fn(&'a str) -> (&str, Result<Out, Error>)>;
