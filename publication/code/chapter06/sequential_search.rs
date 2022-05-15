// sequential_search.rs

fn sequential_search(nums: &[i32], num: i32) -> bool {
    let mut pos = 0;
    let mut found = false;

    // found 表示是否找到
    // pos 在索引范围内且未找到就继续循环
    while pos < nums.len() && !found {
        if num == nums[pos] {
            found = true;
        } else {
            pos += 1;
        }
    }

    found
}

fn main() {
    let num = 8;
    let nums = [9,3,7,4,1,6,2,8,5];
    let found = sequential_search(&nums, num);
    println!("nums contains {num}: {found}");
}
