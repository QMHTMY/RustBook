// dp_rec_mc.rs

fn dp_rec_mc(cashes: &[u32], amount: u32, min_cashes: &mut [u32]) -> u32 {
    // 动态收集从 1 到 amount 的最小找零纸币数
    for denm in 1..=amount {
        // 此 min_cashe_num 等于全用 1 元纸币找零的纸币数
        let mut min_cashe_num = denm;
        for c in cashes.iter().filter(|&&c| c <= denm).collect::<Vec<&u32>>() {
            let index = (denm - c) as usize;

            // 加 1 是因为当前最小找零数等于上一最小找零数加 1 张 c 面额纸币
            let cashe_num = min_cashes[index] + 1;
            if cashe_num < min_cashe_num {
                min_cashe_num = cashe_num;
            }
        }

        min_cashes[denm as usize] = min_cashe_num;
    }

    // 因为收集了所有的最小找零纸币数，所以直接返回
    min_cashes[amount as usize]
}

fn main() {
    let amount = 90u32;
    let cashes = [1,5,10,20,50];
    let mut min_cashes: [u32; 91] =  [0; 91];
    let cashe_num = dp_rec_mc(&cashes,amount,&mut min_cashes);
    println!("Refund for ￥{amount} need {cashe_num} cashes");
}
