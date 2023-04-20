use std::collections::HashMap;

use crate::{
    json::value::{
        self,
        null::parse_null_json,
        number::parse_number_json,
        string::{parse_key_json, parse_string_json},
    },
    parsers::{
        char::{one, one_whitespace},
        combine_two::{combine, first, second},
        map::{map, map_all},
        repeat::{repeat_optional, repeat_until_err},
        skip::skip,
        try_parse::try_parse,
    },
    types::{AstNode, Error, Parser},
};

pub fn parse_json_json() -> Parser<AstNode> {
    second(
        first(one('{'), skip(one_whitespace())),
        first(
            map_all(
                repeat_until_err(combine(
                    first(
                        second(skip(one_whitespace()), parse_key_json()),
                        first(skip(one_whitespace()), one(':')),
                    ),
                    first(
                        second(
                            skip(one_whitespace()),
                            try_parse(vec![
                                parse_number_json(),
                                parse_string_json(),
                                parse_null_json(),
                                Box::new(|input| parse_json_json()(input)),
                            ]),
                        ),
                        one(','),
                    ),
                )),
                &|input| {
                    let mut h = HashMap::new();
                    input
                        .iter()
                        .map(|(k, v)| h.insert(k.clone().to_string(), v.clone()))
                        .count();
                    AstNode::Object(h)
                },
            ),
            first(skip(one_whitespace()), one('}')),
        ),
    )
}

pub fn parse_json(input: &str) -> Result<AstNode, Error> {
    let mut cursor = input;
    let mut r1;
    let mut v1;
    (cursor, _) = one('{')(skip(one_whitespace())(cursor).0);
    let mut v = HashMap::new();
    loop {
        (cursor, r1) = second(skip(one_whitespace()), parse_key_json())(cursor);
        (cursor, v1) = second(
            first(
                first(skip(one_whitespace()), one(':')),
                skip(one_whitespace()),
            ),
            try_parse(vec![
                parse_number_json(),
                parse_string_json(),
                parse_null_json(),
            ]),
        )(cursor);
        (cursor, _) = skip(one(','))(skip(one_whitespace())(cursor).0);
        v.insert(r1.unwrap(), v1.unwrap());

        if one('}')(cursor).1.is_ok() {
            break;
        }
    }

    let (cursor, _) = one('}')(cursor);
    if cursor.len() != 0 {
        return Err(Error::new("too many symbols"));
    }

    return Ok(AstNode::Object(v));
}

#[cfg(test)]
mod test {
    use std::collections::HashMap;

    use crate::{json_parser::parse_json_json, types::AstNode};

    use super::parse_json;

    #[test]
    fn simple() {
        assert_eq!(
            Ok(AstNode::Object(HashMap::from([(
                "asd".to_string(),
                AstNode::Number(123.23)
            )]))),
            parse_json(
                r#"{
			"asd": 123.23
		}"#
            )
        );

        assert_eq!(
            Ok(AstNode::Object(HashMap::from([
                ("asd".to_string(), AstNode::Number(123.23)),
                ("fa".to_string(), AstNode::Number(123.0))
            ]))),
            parse_json(
                r#"{
			"asd": 123.23,
			"fa"  :    123
		}"#
            )
        );

        assert_eq!(
            Ok(AstNode::Object(HashMap::from([
                ("asd".to_string(), AstNode::String("123.23".to_string())),
                ("fdsf".to_string(), AstNode::Null),
                ("fa".to_string(), AstNode::Number(123.0))
            ]))),
            parse_json(
                r#"{
			"asd": "123.23",
			"fa"  :    123,
            "fdsf": null
		}"#
            )
        );
    }

    #[test]
    fn simple_two() {
        assert_eq!(
            (
                "",
                Ok(AstNode::Object(HashMap::from([
                    ("asd".to_string(), AstNode::String("123.23".to_string())),
                    ("fdsf".to_string(), AstNode::Null),
                    ("fa".to_string(), AstNode::Number(123.0))
                ])))
            ),
            parse_json_json()(
                r#"{
			"asd": "123.23",
			"fa":   123,
            "fdsf": null,
		}"#
            )
        );

        assert_eq!(
            (
                "",
                Ok(AstNode::Object(HashMap::from([
                    ("asd".to_string(), AstNode::String("123.23".to_string())),
                    (
                        "fdsf".to_string(),
                        AstNode::Object(HashMap::from([
                            ("g".to_string(), AstNode::Null),
                            (
                                "l".to_string(),
                                AstNode::Object(HashMap::from([(
                                    "hi".to_string(),
                                    AstNode::Number(123.321)
                                )]))
                            )
                        ]))
                    ),
                    ("fa".to_string(), AstNode::Number(123.0))
                ])))
            ),
            parse_json_json()(
                r#"{
			"asd": "123.23",
			"fa":   123,
            "fdsf":  {
                "g": null,
                "l": {
                    "hi": 123.321,
                },
            },
		}"#
            )
        );
    }
}
