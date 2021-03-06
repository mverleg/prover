use logic::Logic;
use util::Nr;
use logic::answer::Answer;

pub fn assert_provable<T, L>(mut statement: L) where L: Logic<T>, T: Nr {
    println!("trying to prove: {}", statement);
    let res = statement.solve();
    if Answer::True != res {
        println!("  failed to prove true statement:");
        println!("  {}", statement);
        println!("  proof result: {}", res);
        panic!(1);
    }
}

pub fn assert_disprovable<T, L>(mut statement: L) where L: Logic<T>, T: Nr {
    println!("trying to disprove: {}", statement);
    let res = statement.solve();
    if Answer::False != res {
        println!("  failed to disprove false statement:");
        println!("  {}", statement);
        println!("  proof result: {}", res);
        panic!(1);
    }
}

pub fn assert_unprovable<T, L>(mut statement: L) where L: Logic<T>, T: Nr {
    println!("trying to show impossible to prove: {}", statement);
    let res = statement.solve();
    if Answer::Maybe != res {
        println!("  proved improvable statement:");
        println!("  {}", statement);
        println!("  proof result: {}", res);
        panic!(1);
    }
}
