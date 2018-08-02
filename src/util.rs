use std::fmt::Display;

pub trait Nr: Display + PartialEq + Eq + PartialOrd + Ord + From<u8> {}

impl<T> Nr for T where T: Display + PartialEq + Eq + PartialOrd + Ord + From<u8> {}

// todo: float eq
