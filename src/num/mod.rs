use util::Difficulty;
use util::Nr;
use std::fmt::Display;

pub trait Num<T>: Difficulty + Display + Clone where T: Nr {
    // todo
}

mod todo;