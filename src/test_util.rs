use std::fmt::Display;
use logic::BLogic;
use logic::Answer;

pub fn assert_provable<T: Display + PartialEq + Eq + PartialOrd + Ord + From<u8>>(statement: BLogic<T>) {
    let res = statement.resolve();
    if Answer::True != res {
        println!("failed to prove true statement:");
        println!("  {}", statement);
        println!("  found {}", res);
        panic!(1);
    }
}

