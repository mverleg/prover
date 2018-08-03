use logic::answer::Answer;
use logic::Logic;
use std::fmt::{Display, Error, Formatter};
use util::Difficulty;
use util::Nr;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct And<T, L>(L, L) where L: Logic<T>, T: Nr;

impl<T, L> Difficulty for And<T, L> where L: Logic<T>, T: Nr {
    fn difficulty(&self) -> usize {
        1 + self.0.difficulty() + self.1.difficulty()
    }
}

impl<T, L> Logic<T> for And<T, L> where L: Logic<T>, T: Nr {
    fn solve(&mut self) -> Answer {
        Answer::Maybe
    }
}

impl<T, L> Display for And<T, L> where L: Logic<T>, T: Nr {
    fn fmt(&self, f: &mut Formatter) -> Result<(), Error> {
        f.write_str(&format!("({} ∧ {})", self.0, self.1))
    }
}
