// binary_search.rs

fn binary_search1(nums: &[i32], num: i32) -> bool {
    let mut low = 0;
    let mut high = nums.len() - 1;
    let mut found = false;

    // 注意是 <= 不是 <
    while low <= high && !found {
        let mid: usize = (low + high) >> 1;

        // 若 low + high 可能溢出，可转换为减法
        // let mid: usize = low  + ((high - low) >> 1);

        if num == nums[mid] {
            found = true;
        } else if num < nums[mid] {
            high = mid - 1; // num < 中间值，省去后半部数据
        } else {
            low = mid + 1;  // num >= 中间值，省去前半部数据
        }
    }

    found
}

fn binary_search2(nums: &[i32], num: i32) -> bool {
    // 基本情况1: 项不存在
    if 0 == nums.len() { return false; }

    let mid: usize = nums.len() >> 1;
    if num  == nums[mid] {
        // 基本情况2: 项存在
        return true;
    } else if num < nums[mid] {
        return binary_search2(&nums[..mid], num);
    } else {
        return binary_search2(&nums[mid+1..], num);
    }
}

fn main() {
    let nums = [1,3,8,10,15,32,44,48,50,55,60,62,64];

    let mut target = 3;
    let found = binary_search1(&nums, target);
    println!("nums contains {target}: {found}");

    target = 63;
    let found = binary_search1(&nums, target);
    println!("nums contains {target}: {found}");

    target = 3;
    let found = binary_search2(&nums, target);
    println!("nums contains {target}: {found}");

    target = 63;
    let found = binary_search2(&nums, target);
    println!("nums contains {target}: {found}");
}
