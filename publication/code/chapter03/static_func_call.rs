// static_func_call.rs

use std::time::SystemTime;

fn sum_of_n(n: i64) -> i64 {
    let mut sum: i64 = 0;
    for i in 1..=n {
        sum += i
    }

    sum
}

fn main() {
    for _i in 0..5 {
        let now = SystemTime::now();
        let _sum = sum_of_n(500000);
        let duration = now.elapsed().unwrap().as_millis();
        println!("func used {duration} ms");
    }
}
