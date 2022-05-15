// num2str_rec.rs

const BASESTR: [&str; 16] = ["0","1","2","3","4","5","6","7",
                             "8","9","A","B","C","D","E","F"];

fn num2str_rec(num: i32, base: i32) -> String {
    if num < base {
        BASESTR[num as usize].to_string()
    } else {
        // 余数应加在末尾
        num2str_rec(num/base, base) + BASESTR[(num % base) as usize]
    }
}

fn main() {
    let num = 100;
    let sb = num2str_rec(num,2);  // sb = str_binary
    let so = num2str_rec(num,8);  // so = str_octal
    let sh = num2str_rec(num,16); // sh = str_hexdecimal
    println!("{num} = b{sb}, o{so}, x{sh}");

    let num = 1000;
    let so = num2str_rec(num,8);
    let sh = num2str_rec(num,16);
    println!("{num} = o{so}, x{sh}");
}
