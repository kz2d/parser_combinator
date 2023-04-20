use crate::types::{Error, Parser};

pub(crate) fn one(ch: char) -> Parser<()> {
    Box::new(move |input: &str| match input.starts_with(ch) {
        false => (input, Err(Error::new(&("empty input one:".to_owned() + &ch.to_string())))),
        true => (&input[1..], Ok(())),
    })
}

pub(crate) fn tag(ch: String) -> Parser<()> {
    Box::new(move |input: &str| match input.starts_with(&ch) {
        false => (input, Err(Error::new("empty input tag"))),
        true => (&input[ch.len()..], Ok(())),
    })
}

pub(crate) fn one_of(chars: Vec<char>) -> Parser<()> {
    Box::new(move |input: &str| {
        if input.is_empty() {
            return (input, Err(Error::new("empty input one_of")));
        }
        for ch in &chars {
            if input.starts_with(*ch) {
                return (&input[1..], Ok(()));
            }
        }
        return (
            input,
            Err(Error::new(format!(
                "expected {:?} found {}",
                chars,
                input.chars().next().unwrap()
            ).as_str())),
        );
    })
}

pub(crate) fn one_whitespace() -> Parser<()> {
    Box::new(move |input: &str| {
        if input.len() == 0 {
            (input, Err(Error::new(" one_whitespace")))
        } else if input.chars().next().unwrap().is_whitespace() {
            (&input[1..], Ok(()))
        } else {
            (&input[1..], Err(Error::new("empty input one_whitespace")))
        }
    })
}

#[cfg(test)]
mod test {
    use super::one;

    #[test]
    fn first() {
        assert_eq!("ds", one('a')("ads").0);
        assert_eq!("ads", one('b')("ads").0);
    }
}
