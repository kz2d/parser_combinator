use crate::{
    parsers::{
        char::one,
        combine_two::{first, second},
        skip::skip_until,
    },
    types::{Parser, AstNode},
};

pub(crate) fn parse_key_json() -> Parser<String> {
    second(one('"'), skip_until(one('"')))
}

pub fn parse_string_json() -> Parser<AstNode> {
	Box::new(|input| {
		let v = parse_key_json()(input);
		(v.0, if v.1.is_ok() {Ok(AstNode::String(v.1.unwrap()))} else {Err(v.1.unwrap_err())})
	})
}

#[cfg(test)]
mod test {
    use crate::types::AstNode;

    use super::parse_key_json;

    #[test]
    fn first() {
        assert_eq!(
            ("", Ok("adsad".to_string())),
            parse_key_json()("\"adsad\"")
        )
    }
}
