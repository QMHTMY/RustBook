// sum_of_n.rs

// 迭代计算
fn sum_of_n(n: i32) -> i32 {
    let mut sum: i32 = 0;
    for i in 1..=n {
        sum += i;
    }

    sum
}

// 迭代器计算
fn sum_of_n1(n: i32) -> i32 {
    (1..=n).sum::<i32>()
}

// 数学公式计算
fn sum_of_n2(n: i32) -> i32 {
    n * (n + 1) / 2
}

fn main() {
    let sum = sum_of_n1(10);
    println!("{sum}");
}
