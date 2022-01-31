// counting_sort.rs

fn counting_sort(nums: &mut [usize]) {
    if nums.len() <= 1 { return; }

    // 桶数量为 nums 中最大值加 1，保证数据都有桶放
    let max_bkt_num = nums.iter().max().unwrap() + 1;
    let mut counter = vec![0; max_bkt_num];
    for &v in nums.iter() {
        counter[v] += 1; // 将数据标记到桶
    }

    // 数据写回原 nums 切片
    let mut j = 0;
    for i in 0..max_bkt_num {
        while counter[i] > 0 {
            nums[j] = i;
            counter[i] -= 1;
            j += 1;
        }
    }
}

fn main() {
    let mut nums = [54,32,99,18,75,31,43,56,21,22,1,100];
    counting_sort(&mut nums);
    println!("sorted nums: {:?}", nums);
}
