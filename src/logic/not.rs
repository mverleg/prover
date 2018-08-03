use logic::answer::Answer;
use logic::Logic;
use std::fmt::{Display, Error, Formatter};
use util::Difficulty;
use util::Nr;
use std::marker::PhantomData;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Not<T, L>(pub L, pub PhantomData<T>) where L: Logic<T>, T: Nr;

impl<T, L> Difficulty for Not<T, L> where L: Logic<T>, T: Nr {
    fn difficulty(&self) -> usize {
        1 + self.0.difficulty()
    }
}

impl<T, L> Logic<T> for Not<T, L> where L: Logic<T>, T: Nr {
    fn solve(&mut self) -> Answer {
        Answer::Maybe
    }
}

impl<T, L> Display for Not<T, L> where L: Logic<T>, T: Nr {
    fn fmt(&self, f: &mut Formatter) -> Result<(), Error> {
        f.write_str(&format!("￢{}", self.0))
    }
}
