use logic::answer::Answer;
use logic::Logic;
use std::fmt::{Display, Error, Formatter};
use util::Difficulty;
use util::Nr;
use num::Num;
use std::marker::PhantomData;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Gt<T, L, R>(pub L, pub R, pub PhantomData<T>) where L: Num<T>, R: Num<T>, T: Nr;

impl<T, L, R> Difficulty for Gt<T, L, R> where L: Num<T>, R: Num<T>, T: Nr {
    fn difficulty(&self) -> usize {
        1 + self.0.difficulty() + self.1.difficulty()
    }
}

impl<T, L, R> Logic<T> for Gt<T, L, R> where L: Num<T>, R: Num<T>, T: Nr {
    fn solve(&mut self) -> Answer {
        Answer::Maybe
    }
}

impl<T, L, R> Display for Gt<T, L, R> where L: Num<T>, R: Num<T>, T: Nr {
    fn fmt(&self, f: &mut Formatter) -> Result<(), Error> {
        f.write_str(&format!("({} > {})", self.0, self.1))
    }
}
