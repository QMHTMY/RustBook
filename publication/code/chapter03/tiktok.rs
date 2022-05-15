// tiktok.rs

fn tiktok(tik: i32) -> i32 {
    let mut tok = 0;
    for k in 1..=tik {
        let ggg = k;
        tok = tok + ggg;
    }

    tok
}

fn main() {
    let tik = 10;
    let res = tiktok(tik);
    println!("{res}");
}
