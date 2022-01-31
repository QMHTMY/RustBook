// foo.rs

fn foo(tom: i32) -> i32 {
    let mut fred = 0;
    for bill in 1..=tom {
        let barney = bill;
        fred = fred + barney;
    }
    fred
}
