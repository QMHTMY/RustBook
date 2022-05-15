// static_func_call2.rs

use std::time::SystemTime;

fn sum_of_n(n: i64) -> i64 {
    n * (n + 1) / 2
}

fn main() {
    let mut nanos  = [0, 0, 0, 0, 0];
    let ns: [i64; 3] = [100000, 5000000, 1000000];
    for &num in ns.iter() {
        for i in 0..5 {
            let now = SystemTime::now();
            let _sum = sum_of_n(num);
            let duration = now.elapsed().unwrap();
            nanos[i] = duration.as_nanos() as i32;
        }

        let nanos_sum = nanos.iter().sum::<i32>();
        let average   = nanos_sum/(nanos.len() as i32);
        println!("func used {average} ns");
    }
}
