use logic::answer::Answer;
use logic::Logic;
use num::Num;
use std::fmt::{Display, Error, Formatter};
use std::marker::PhantomData;
use util::Difficulty;
use util::Nr;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Lt<T, L, R>(pub L, pub R, pub PhantomData<T>) where L: Num<T>, R: Num<T>, T: Nr;

impl<T, L, R> Difficulty for Lt<T, L, R> where L: Num<T>, R: Num<T>, T: Nr {
    fn difficulty(&self) -> usize {
        1 + self.0.difficulty() + self.1.difficulty()
    }
}

impl<T, L, R> Logic<T> for Lt<T, L, R> where L: Num<T>, R: Num<T>, T: Nr {
    fn solve(&mut self) -> Answer {
        Answer::Maybe
    }
}

impl<T, L, R> Display for Lt<T, L, R> where L: Num<T>, R: Num<T>, T: Nr {
    fn fmt(&self, f: &mut Formatter) -> Result<(), Error> {
        f.write_str(&format!("({} < {})", self.0, self.1))
    }
}
