use crate::types::{Parser, AstNode, Error};

pub(crate) fn skip<T: 'static>(p: Parser<T>) -> Parser<String> {
	Box::new(move |input: &str| {
		let mut word = String::new();
		let mut r: (&str, Result<T, Error>) = (input, Err(Error::new("unreacheble error")));
		loop {
			let new_r = p(r.0);
			if new_r.1.is_err() {
				let l =new_r.0.as_ptr() as usize - input.as_ptr() as usize;
				return (r.0, Ok(input[..if l != 0 {l - 1}else {l}].to_string()));
			}
			r = new_r;
		}
	})
}

pub(crate) fn skip_until_without_last_char<T: 'static>(p: Parser<T>, max: i32) -> Parser<String> {
	Box::new(move |input: &str| {
		let mut r = input;
		loop {
			let new_r = p(r);
			if new_r.1.is_ok() {
				return (r, Ok(input[..new_r.0.as_ptr() as usize - input.as_ptr() as usize - 1].to_string()));
			}

			if new_r.0.len() == 0{
				return (input, Err(Error::new("reached end of file")));
			}
			r = &new_r.0[1..];
		}
	})
}

pub(crate) fn skip_until<T: 'static>(p: Parser<T>) -> Parser<String> {
	Box::new(move |input: &str| {
		let mut r = input;
		loop {
			let new_r = p(r);
			if new_r.1.is_ok() {
				return (new_r.0, Ok(input[..new_r.0.as_ptr() as usize - input.as_ptr() as usize - 1].to_string()));
			}

			if new_r.0.len() == 0{
				return (input, Err(Error::new("reached end of file")));
			}
			r = &new_r.0[1..];
		}
	})
}

pub(crate) fn skip_until_with_max<T: 'static>(p: Parser<T>, max: i32) -> Parser<String> {
	Box::new(move |input: &str| {
		let mut r = input;
		let mut i =0;
		loop {
			let new_r = p(r);
			if i > max {
				return (r, Err(new_r.1.err().unwrap()));
			}
			if new_r.1.is_ok() {
				return (new_r.0, Ok(input[..new_r.0.as_ptr() as usize - input.as_ptr() as usize - 1].to_string()));
			}
						if new_r.0.len() == 0{
				return (input, Err(Error::new("reached end of file")));
			}
			r = &new_r.0[1..];
			i+=1;
		}
	})
}

#[cfg(test)]
mod test{
    use crate::{parsers::{skip::skip_until, char::one}, types::AstNode};

	#[test]
	fn skip_until_test(){
		assert_eq!(("ad", Ok("ds".to_string())), skip_until(one('f'))("dsfad"));
	}
}