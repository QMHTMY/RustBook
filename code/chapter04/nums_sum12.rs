fn nums_sum1(nums: &[i32]) -> i32 {
    if 1 == nums.len() {
        nums[0]
    } else {
        let first = nums[0];
        first + nums_sum1(&nums[1..])
    }
}

fn nums_sum2(nums: &[i32]) -> i32 {
    if 1 == nums.len() {
        nums[0]
    } else {
        let last = nums[nums.len() - 1];
        last + nums_sum2(&nums[..nums.len() - 1])
    }
}

fn main() {
    let nums = [2,1,7,4,5];
    let sum1 = nums_sum1(&nums);
    let sum2 = nums_sum2(&nums);
    println!("sum1 is {sum1}, sum2 is {sum2}");
}
