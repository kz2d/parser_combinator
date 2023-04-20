use crate::{types::Parser, parsers::{skip::{skip_until, skip_until_with_max, skip_until_without_last_char}, char::one_of}};

pub fn skip_until_terminate() -> Parser<String> {
	skip_until_without_last_char(is_terminate(), 100)
}

pub fn is_terminate() -> Parser<()> {
	one_of(vec![',', '}', ']', '\n', '\t', ' '])
}