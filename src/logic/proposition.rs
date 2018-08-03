use logic::answer::Answer;
use logic::Logic;
use std::fmt::{Display, Error, Formatter};
use util::Difficulty;
use util::Nr;
use num::Num;

/// Proposition
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct Prop(pub String);

impl Difficulty for Prop {
    fn difficulty(&self) -> usize {
        1
    }
}

impl<T> Logic<T> for Prop where T: Nr {
    fn solve(&mut self) -> Answer {
        // todo: does this make sense?
        unimplemented!();
    }
}

impl Display for Prop {
    fn fmt(&self, f: &mut Formatter) -> Result<(), Error> {
        f.write_str(&format!("{}", self.0))
    }
}
