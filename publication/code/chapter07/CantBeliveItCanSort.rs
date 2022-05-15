// CantBelieveItCanSort.rs

// 升序排序
fn cbic_sort1(nums: &mut [i32]) {
    for i in 0..nums.len() {
        for j in 0..nums.len() {
            if nums[i] < nums[j] {
                nums.swap(i, j);
            }
        }
    }
}

// 升序排序优化版
fn cbic_sort2(nums: &mut [i32]) {
    if nums.len() < 2 { return; }

    for i in 1..nums.len() {
        for j in 0..i {
            if nums[i] < nums[j] {
                nums.swap(i, j);
            }
        }
    }
}

// 降序排序
fn cbic_sort3(nums: &mut [i32]) {
    for i in 0..nums.len() {
        for j in 0..nums.len() {
            if nums[i] > nums[j] {
                nums.swap(i, j);
            }
        }
    }
}

// 降序排序优化版
fn cbic_sort4(nums: &mut [i32]) {
    if nums.len() < 2 { return; }

    for i in 1..nums.len() {
        for j in 0..i {
            if nums[i] > nums[j] {
                nums.swap(i, j);
            }
        }
    }
}

fn main() {
    let mut nums = [54, 32, 99, 18, 75, 31, 43, 56, 21, 22];
    cbic_sort1(&mut nums);
    //cbic_sort2(&mut nums);
    println!("sorted nums {:?}", nums);

    let mut nums = [54, 32, 99, 18, 75, 31, 43, 56, 21, 22];
    cbic_sort3(&mut nums);
    //cbic_sort4(&mut nums);
    println!("sorted nums {:?}", nums);
}
