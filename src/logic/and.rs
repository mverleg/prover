use logic::answer::Answer;
use logic::Logic;
use std::fmt::{Display, Error, Formatter};
use util::Difficulty;
use util::Nr;
use std::marker::PhantomData;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct And<T, L, R>(pub L, pub R, pub PhantomData<T>) where L: Logic<T>, R: Logic<T>, T: Nr;

impl<T, L, R> Difficulty for And<T, L, R> where L: Logic<T>, R: Logic<T>, T: Nr {
    fn difficulty(&self) -> usize {
        1 + self.0.difficulty() + self.1.difficulty()
    }
}

impl<T, L, R> Logic<T> for And<T, L, R> where L: Logic<T>, R: Logic<T>, T: Nr {
    fn solve(&mut self) -> Answer {
        Answer::Maybe
    }
}

impl<T, L, R> Display for And<T, L, R> where L: Logic<T>, R: Logic<T>, T: Nr {
    fn fmt(&self, f: &mut Formatter) -> Result<(), Error> {
        f.write_str(&format!("({} ∧ {})", self.0, self.1))
    }
}
