use logic::BLogic;
use logic::Answer;
use util::Nr;

pub fn assert_provable<T: Nr>(statement: BLogic<T>) {
    let res = statement.resolve();
    if Answer::True != res {
        println!("failed to prove true statement:");
        println!("  {}", statement);
        println!("  found {}", res);
        panic!(1);
    }
}

