use crate::{
    parsers::char::tag,
    parsers::map::{self, map_all},
    types::{AstNode, Parser},
};

pub fn parse_null_json() -> Parser<AstNode> {
    map_all(tag("null".to_string()), &|_| AstNode::Null)
}
