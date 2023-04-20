use crate::types::{Parser, Error};

pub(crate) fn any() -> Parser<()> {
	Box::new(|input: &str|
		match input.len() {
			0 => (input, Err(Error::new("empty input any.rs"))),
			_ => (&input[1..], Ok(())),
		}
	)
}

#[cfg(test)]
mod test{
	use super::any;

	#[test]
	fn first(){
		assert_eq!("ds", any()("ads").0);
		assert_eq!("", any()("").0);
		assert!(any()("").1.is_err());
	}
}