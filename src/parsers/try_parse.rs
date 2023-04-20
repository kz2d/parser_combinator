use crate::types::{Error, Parser};

pub fn try_parse<F: 'static>(first: Vec<Parser<F>>) -> Parser<F> {
    Box::new(move |input| {
        let mut last = (input, Err(Error::new("parser is empty")));
        for i in &first {
            match i(input) {
                x if x.1.is_err() => {
                    last = x;
                    continue;
                }
                x => return x,
            }
        }
        last
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
