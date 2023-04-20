use crate::{parsers::{skip::{skip_until, skip}, char::{one_of, one_whitespace}}, types::{Error, AstNode, Parser}, json::terminate::skip_until_terminate};

pub fn parse_number_json() -> Parser<AstNode> {
	Box::new(|input| {
		let r = skip_until_terminate()(input);
		if r.1.is_err() {
			return (r.0, Err(r.1.err().unwrap()));
		}
		let s = r.1.unwrap().parse::<f64>();
		if s.is_err() {
			return (r.0, Err(Error::new(&s.err().unwrap().to_string())));
		}

		(r.0, Ok(AstNode::Number(s.unwrap())))
	})
}