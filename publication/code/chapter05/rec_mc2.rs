// rec_mc2.rs

fn rec_mc2(cashes: &[u32], amount: u32, min_cashes: &mut [u32]) -> u32 {
    // 全用 1 元纸币的最小找零纸币数量
    let mut min_cashe_num = amount;

    if cashes.contains(&amount) {
        // 收集和当前待找零值相同的币种
        min_cashes[amount as usize] = 1;
        return 1;
    } else if min_cashes[amount as usize] > 0 {
        // 找零值 amount 有最小找零纸币数，直接返回
        return min_cashes[amount as usize];
    } else {
        for c in cashes.iter().filter(|&&c| c <= amount).collect::<Vec<&u32>>() {
            let cashe_num = 1 + rec_mc2(cashes, amount - c, min_cashes);
            // 更新最小找零纸币数
            if cashe_num < min_cashe_num {
                min_cashe_num = cashe_num;
                min_cashes[amount as usize] = min_cashe_num;
            }
        }
    }

    min_cashe_num
}

fn main() {
    let amount = 90u32;
    let cashes: [u32; 5] = [1,5,10,20,50];
    let mut min_cashes: [u32; 91] = [0; 91]; // 0 元找零 0 张
    let cashe_num = rec_mc2(&cashes, amount, &mut min_cashes);
    println!("need refund {cashe_num} cashes");
}
