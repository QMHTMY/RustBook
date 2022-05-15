// hamming_distance.rs

fn hamming_distance1(source: u64, target: u64) -> u32 {
    let mut distance = 0;
    let mut xor = source ^ target;

    // 异或取值
    while xor != 0 {
        distance += xor & 1;
        xor >>= 1;
    }

    distance as u32
}

fn hamming_distance2(source: u64, target: u64) -> u32 {
    (source ^ target).count_ones()
}

fn hamming_distance_str(source: &str, target: &str) -> u32 {
    let mut count = 0;
    let mut source = source.chars();
    let mut target = target.chars();

    // 两字符串逐字符比较可能出现如下四种情况
    // 1. 都有下一个字符，不等时距离加 1
    // 2. 都有下一个字符，相等时继续比较下一个字符
    // 3. 都没有下一个字符，则完成比较
    // 4. 一边有，一边没有，则长度都不同
    loop {
        match (source.next(), target.next()) {
            (Some(cs), Some(ct)) if cs != ct => count += 1,
            (Some(_), None) | (None, Some(_)) => panic!("Error: mismatched length!"),
            (None, None) => break,
            _ => continue,
        }
    }

    count as u32
}

fn main() {
    let source = 1;
    let target = 2;
    let distance = hamming_distance1(source, target);
    println!("the hamming distance is {distance}");

    let source = 3;
    let target = 4;
    let distance = hamming_distance1(source, target);
    println!("the hamming distance is {distance}");

    let source = "abce";
    let target = "edcf";
    let distance = hamming_distance_str(source, target);
    println!("the hamming distance is {distance}");
}
