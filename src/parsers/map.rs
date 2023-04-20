use crate::types::{Parser, Error};

pub fn map<F: 'static, G: Clone>(first: Parser<Vec<F>>, map: &'static dyn Fn(F,) -> G) -> Parser<Vec<G>> 
{
    Box::new(move |input| {
		match first(input) {
			y if y.1.is_err() => (y.0, Err(y.1.err().unwrap())),
			x => {
				let mut v = vec![];
				for i in x.1.unwrap() {
					v.push(map(i));
				}
				(x.0, Ok(v))
			}
		}
    })
}

pub fn map_all<F: 'static, G: Clone>(first: Parser<F>, map: &'static dyn Fn(F,) -> G) -> Parser<G> 
{
    Box::new(move |input| {
		match first(input) {
			y if y.1.is_err() => (y.0, Err(y.1.err().unwrap())),
			x => {
				(x.0, Ok(map(x.1.unwrap())))
			}
		}
    })
}

#[cfg(test)]
mod test {
    use crate::parsers::{any::any, char::one, combine_two::second};

    #[test]
    fn first() {
        assert_eq!("s", second(any(), one('d'))("ads").0);
        assert_eq!("ads", second(one('d'), one('a'))("ads").0);
    }
}
