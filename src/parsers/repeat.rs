use crate::types::{Parser, Error};

pub fn repeat_until_err<F: 'static + Clone>(first: Parser<F>) -> Parser<Vec<F>> {
    Box::new(move |input| {
        let mut res = vec![];
        let mut prev_res = (input, Err(Error::new("never")));
        loop {
            match first(prev_res.0) {
                x if x.1.is_err() => {
                    if res.is_empty() {
                        return (input, Err(x.1.err().unwrap()));
                    }
                    return (prev_res.0, Ok(res));
                },
                x => {res.push(x.1.clone().unwrap());prev_res = x;},
            }
        }
    })
}

pub fn repeat_optional<F: 'static + Clone>(first: Parser<F>, min: usize, max:usize) -> Parser<Vec<F>> {
    Box::new(move |input| {
        let mut res = vec![];
        let mut prev_res = (input, Err(Error::new("never")));
        loop {
            match first(prev_res.0) {
                x if x.1.is_err() || res.len() == max => {
                    if res.len() < min {
                        return (input, Err(x.1.err().unwrap()));
                    }
                    return (prev_res.0, Ok(res));
                },
                x => {res.push(x.1.clone().unwrap());prev_res = x;},
            }
        }
    })
}

#[cfg(test)]
mod test {
    use crate::{parsers::{any::any, char::{one, one_whitespace}, combine_two::{second, first}, repeat::repeat_until_err, skip::skip}, json::value::number::parse_number_json, types::{Error, AstNode}};

    #[test]
    fn first_test() {
        assert_eq!("s", repeat_until_err(one('d'))("ddddds").0);
        assert_eq!(("ddddds", Err(Error::new("reached end of file"))), repeat_until_err(first(parse_number_json(), skip(one_whitespace())))("ddddds"));
        assert_eq!(("knjfgbk", Ok(vec![AstNode::Number(123.0), AstNode::Number(234.0), AstNode::Number(45645645.546456)])), repeat_until_err(first(parse_number_json(), skip(one_whitespace())))("123 234 45645645.546456 knjfgbk"));
    }
}
