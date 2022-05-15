// fibnacci_dp_rec.rs

fn fibnacci_dp(n: u32) -> u32 {
    // 只用两个位置来保存值，节约内存
    let mut dp = [1, 1];

    for i in 2..=n {
        let idx1 = (i % 2) as usize;
        let idx2 = ((i - 1) % 2) as usize;
        let idx3 = ((i - 2) % 2) as usize;
        dp[idx1] = dp[idx2] + dp[idx3];
    }

    dp[((n-1) % 2) as usize]
}

fn fibnacci_rec(n: u32) -> u32 {
    if n == 1 || n == 2 {
        return 1;
    } else {
        fibnacci_rec(n - 1) + fibnacci_rec(n - 2)
    }
}

fn main() {
    println!("fib(10): {}", fibnacci_dp(10));
    println!("fib(10): {}", fibnacci_rec(10));
}
