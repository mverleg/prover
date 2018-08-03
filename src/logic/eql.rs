use logic::answer::Answer;
use logic::Logic;
use std::fmt::{Display, Error, Formatter};
use util::Difficulty;
use util::Nr;
use num::Num;
use std::marker::PhantomData;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Eql<T, N>(pub N, pub N, pub PhantomData<T>) where N: Num<T>, T: Nr;

impl<T, N> Difficulty for Eql<T, N> where N: Num<T>, T: Nr {
    fn difficulty(&self) -> usize {
        1 + self.0.difficulty() + self.1.difficulty()
    }
}

impl<T, N> Logic<T> for Eql<T, N> where N: Num<T>, T: Nr {
    fn solve(&mut self) -> Answer {
        Answer::Maybe
    }
}

impl<T, N> Display for Eql<T, N> where N: Num<T>, T: Nr {
    fn fmt(&self, f: &mut Formatter) -> Result<(), Error> {
        f.write_str(&format!("({} = {})", self.0, self.1))
    }
}
