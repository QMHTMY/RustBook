// adapter_consumer.rs

fn main() {
    let nums = vec![1,2,3,4,5,6];
    let nums_iter = nums.iter();
    let total = nums_iter.sum::<i32>(); // 消费者

    let new_nums: Vec<i32> = (0..100).filter(|&n| 0 == n % 2)
                                     .collect(); // 适配器
    println!("{:?}", new_nums);

    // 求小于1000的能被3或5整除的所有整数之和
    let sum = (1..1000).filter(|n| n % 3 == 0 || n % 5 == 0)
                       .sum::<u32>(); // 结合适配器和消费者
    println!("{sum}");
}
