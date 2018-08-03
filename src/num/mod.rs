use util::Difficulty;
use util::Nr;
use std::fmt::Display;
use std::hash::Hash;

pub trait Num<T>: Difficulty + Display + Clone + PartialEq + Eq + Hash where T: Nr {
    // todo
}

mod todo;
