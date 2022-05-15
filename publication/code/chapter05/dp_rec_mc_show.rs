// dp_rec_mc_show.rs

// 使用 cashes_used 收集使用过的各面额纸币
fn dp_rec_mc_show(cashes: &[u32],
                  amount: u32,
                  min_cashes: &mut [u32],
                  cashes_used: &mut [u32]) -> u32
{
    for denm in 1..=amount {
        let mut min_cashe_num = denm ;
        let mut used_cashe = 1; // 最小面额是 1 元
        for c in cashes.iter().filter(|&&c| c <= denm).collect::<Vec<&u32>>() {
            let index = (denm - c) as usize;
            let cashe_num = 1 + min_cashes[index];
            if cashe_num < min_cashe_num {
                min_cashe_num = cashe_num;
                used_cashe = *c;
            }
        }

        // 更新各金额对应的最小纸币数
        min_cashes[denm as usize] = min_cashe_num;
        cashes_used[denm as usize] = used_cashe;
    }

    min_cashes[amount as usize]
}

// 打印输出各面额纸币
fn print_cashes(cashes_used: &[u32], mut amount: u32) {
    while amount > 0 {
        let curr = cashes_used[amount as usize];
        println!("￥{curr}");
        amount -= curr;
    }
}

fn main() {
    let cashes = [1,5,10,20,50];
    let amount = 90u32;
    let mut min_cashes: [u32; 91] = [0; 91];
    let mut cashes_used: [u32; 91] = [0; 91];

    let min_cashe_num = dp_rec_mc_show(&cashes, amount, &mut min_cashes, &mut cashes_used);
    println!("Refund for ￥{amount} requires {min_cashe_num} cashes:");
    print_cashes(&cashes_used, amount);
}
