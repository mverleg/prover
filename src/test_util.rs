use logic::BLogic;
use logic::Answer;
use util::Nr;

pub fn assert_provable<T: Nr>(statement: BLogic<T>) {
    println!("trying to prove: {}", statement);
    let res = statement.resolve();
    if Answer::True != res {
        println!("  failed to prove true statement:");
        println!("  {}", statement);
        println!("  proof result: {}", res);
        panic!(1);
    }
}

pub fn assert_disprovable<T: Nr>(statement: BLogic<T>) {
    println!("trying to disprove: {}", statement);
    let res = statement.resolve();
    if Answer::False != res {
        println!("  failed to disprove false statement:");
        println!("  {}", statement);
        println!("  proof result: {}", res);
        panic!(1);
    }
}

pub fn assert_unprovable<T: Nr>(statement: BLogic<T>) {
    println!("trying to show impossible to prove: {}", statement);
    let res = statement.resolve();
    if Answer::Maybe != res {
        println!("  proved improvable statement:");
        println!("  {}", statement);
        println!("  proof result: {}", res);
        panic!(1);
    }
}
