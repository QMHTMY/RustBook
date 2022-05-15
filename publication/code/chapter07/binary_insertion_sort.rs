// binary_insertion_sort.rs

// 二分插入排序
fn binary_insertion_sort(nums: &mut [i32]) {
    let mut temp;
    let mut left;
    let mut mid;
    let mut right;

    for i in 1..nums.len() {
        // 已排序数组左右边界
        left = 0;
        right = i - 1;

        // 待排序数据
        temp = nums[i];

        // 二分法找到 temp 的位置
        while left <= right {
            mid = (left + right) >> 1;
            if temp < nums[mid] {
                // 防止出现 right = 0 - 1
                if 0 == mid { break; }
                right = mid - 1;
            } else {
                left = mid + 1;
            }
        }

        // 将数据后移，留出空位
        for j in (left..=i-1).rev() {
            nums.swap(j, j+1);
        }

        // 将 temp 插入空位
        if left != i {
            nums[left] = temp;
        }
    }
}

fn main() {
    let mut nums = [1,3,2,8,6,4,9,7,5,10];
    binary_insertion_sort(&mut nums);
    println!("sorted nums: {:?}", nums);
}
