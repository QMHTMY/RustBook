// exponential_search.rs

fn binary_search(nums: &[i32], num: i32) -> bool {
    let mut low = 0;
    let mut high = nums.len() - 1;
    let mut found = false;

    // 注意是 <= 不是 <
    while low <= high && !found {
        let mid: usize = (low + high) >> 1;
        if num == nums[mid] {
            found = true;
        } else if num < nums[mid] {
            high = mid - 1;
        } else {
            low = mid + 1;
        }
    }

    found
}

fn exponential_search(nums: &[i32], target: i32) -> bool {
    let size = nums.len();
    if size == 0 { return false; }

    // 逐步找到上界
    let mut high = 1usize;
    while high < size && nums[high] < target {
        high <<= 1;
    }

    //  上界的一半一定可以作为下界
    let low = high >> 1;

    // 区间内二分搜索加速查找
    binary_search(&nums[low..size.min(high+1)], target)
}

fn main() {
    let nums = [1,9,10,15,16,17,19,23,27,28,29,30,32,35];
    let target = 27;
    let found = exponential_search(&nums, target);
    println!("nums contains {target}: {found}");
}
