use std::fmt::{Display, Formatter, Error};

#[derive(Debug, PartialEq, Eq)]
pub enum Answer {
    True,
    False,
    Maybe,
}

impl Display for Answer {
    fn fmt(&self, f: &mut Formatter) -> Result<(), Error> {
        f.write_str(match self {
            Answer::True => "true",
            Answer::False => "false",
            Answer::Maybe => "maybe",
        })
    }
}
