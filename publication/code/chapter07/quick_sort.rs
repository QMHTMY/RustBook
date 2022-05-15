// quick_sort.rs

fn quick_sort1(nums: &mut [i32], low: usize, high: usize) {
    if low < high {
        let split = partition(nums, low, high);

        // 防止越界和语法错误（split <= 1 的情形）
        if split > 1 {
            quick_sort1(nums, low, split - 1);
        }
        quick_sort1(nums, split + 1, high);
    }
}

// 计算分割点
fn partition(nums: &mut [i32], low: usize, high: usize) -> usize {
    let mut lm = low;  // 左标记
    let mut rm = high; // 右标记

    loop {
        // 左标记不断右移
        while lm <= rm && nums[lm] <= nums[low] {
            lm += 1;
        }
        // 右标记不断左移
        while lm <= rm && nums[low] <= nums[rm] {
            rm -= 1;
        }

        // 左标记越过右标记时退出并交换左右标记数据
        if lm > rm {
            break;
        } else {
            nums.swap(lm, rm);
        }
    }
    nums.swap(low, rm);

    rm
}

// 分割点不单独计算的快速排序，lm 和 rm 作分割点
fn quick_sort2(nums: &mut [i32], low: usize, high: usize) {
    if low >= high { return; }

    let mut lm = low;
    let mut rm = high;
    while lm < rm {
        // 右标记不断左移
        while lm < rm && nums[low] <= nums[rm] {
            rm -= 1;
        }
        // 左标记不断右移
        while lm < rm && nums[lm] <= nums[low] {
            lm += 1;
        }
        // 交换左右标记处数据
        nums.swap(lm, rm);
    }
    // 交换分割点数据
    nums.swap(low, lm);

    if lm > 1 {
        quick_sort2(nums, low, lm - 1);
    }

    quick_sort2(nums, rm + 1, high);
}

fn main() {
    let mut nums = [54,26,93,17,77,31,44,55,20];
    let high = nums.len() - 1;
    quick_sort1(&mut nums, 0, high);
    println!("sorted nums: {:?}", nums);

    let mut nums = [54,26,93,17,77,31,44,55,20];
    let high = nums.len() - 1;
    quick_sort2(&mut nums, 0, high);
    println!("sorted nums: {:?}", nums);
}
