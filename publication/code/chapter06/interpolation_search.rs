// interpolation_search.rs

fn interpolation_search(nums: &[i32], target: i32) -> bool {
    if nums.is_empty() {
        return false;
    }

    // 查询范围
    let mut low  = 0usize;
    let mut high = nums.len() - 1;
    loop {
        let low_val  = nums[low];
        let high_val = nums[high];
        if high <= low || target < low_val || target > high_val {
            break;
        }

        // 计算插值位置
        let offset = (target - low_val)*(high - low) as i32 / (high_val - low_val);
        let interpolant = low + offset as usize;

        // 更新上下界 high、low
        if nums[interpolant] > target {
            high = interpolant - 1;
        } else if nums[interpolant] < target {
            low = interpolant + 1;
        } else {
            break;
        }
    }

    // 判断最终确定的上界处是否是 target
    target == nums[high]
}

fn main() {
    let nums = [1,9,10,15,16,17,19,23,27,28,29,30,32,35];
    let target = 27;
    let found = interpolation_search(&nums, target);
    println!("nums contains {target}: {found}");

    let nums = [0,1,2,10,16,19,31,35,36,38,40,42,43,55];
    let found = interpolation_search(&nums, target);
    println!("nums contains {target}: {found}");
}
