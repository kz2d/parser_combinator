use crate::types::Parser;

pub fn second<F: 'static, S: 'static>(first: Parser<F>, second: Parser<S>) -> Parser<S> {
    Box::new(move |input| match first(input) {
        x if x.1.is_err() => (x.0, Err(x.1.err().unwrap())),
        x => second(x.0),
    })
}

pub fn combine<F: 'static, S: 'static>(first: Parser<F>, second: Parser<S>) -> Parser<(F, S)> {
    Box::new(move |input| match first(input) {
        x if x.1.is_err() => (x.0, Err(x.1.err().unwrap())),
        x => {
            let r = second(x.0);
			if r.1.is_err() {
				return (r.0, Err(r.1.err().unwrap()));
			}else {
				return (r.0, Ok((x.1.unwrap(), r.1.unwrap())));
			}
        }
    })
}

pub fn first<F: 'static, S: 'static>(first: Parser<F>, second: Parser<S>) -> Parser<F> {
    Box::new(move |input| match first(input) {
        x if x.1.is_err() => x,
        x => {
            let r = second(x.0);
			if r.1.is_err() {
				return (r.0, Err(r.1.err().unwrap()));
			}else {
				return (r.0, x.1);
			}
        }
    })
}

#[cfg(test)]
mod test {
    use crate::{parsers::{any::any, char::one, combine_two::{second, combine}}, json::value::{string::{parse_key_json, parse_string_json}, number::parse_number_json}, types::AstNode};

    #[test]
    fn first() {
        assert_eq!(("d", Ok(((),()))), combine(one('a'), one('s'))("asd"));
        assert_eq!(("    njk", Ok(((AstNode::String("asdasd".to_string())),AstNode::Number(123.0)))), combine(parse_string_json(), parse_number_json())("\"asdasd\"123    njk"));
    }
}
