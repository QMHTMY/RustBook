// sequential_search_pos.rs

fn sequential_search_pos(nums: &[i32], num: i32) -> Option<usize> {
    let mut pos: usize = 0;
    let mut found = false;

    while pos < nums.len() && !found {
        if num == nums[pos] {
            found = true;
        } else {
            pos += 1;
        }
    }

    if found {
        Some(pos)
    } else {
        None
    }
}

fn main() {
    let num = 8;
    let nums = [9,3,7,4,1,6,2,8,5];
    match sequential_search_pos(&nums, num) {
        Some(pos) => println!("{num}'s index: {pos}"),
        None => println!("nums does not contain {num}"),
    }
}
