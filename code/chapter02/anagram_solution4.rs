fn anagram_solution4(s1: &str, s2: &str) -> bool {
    if s1.len() != s2.len() { return false; }

    // 大小为 26 的集合，用于将字符映射为 ASCII 值
    let mut c1 = [0; 26];
    let mut c2 = [0; 26];
    for c in s1.chars() {
        let pos = (c as usize) - 97; // 97 为 a 的 ASCII 值
        c1[pos] += 1;
    }
    for c in s2.chars() {
        let pos = (c as usize) - 97;
        c2[pos] += 1;
    }

    // 逐个比较 ascii 值
    let mut pos = 0;
    let mut ok = true;
    while pos < 26 && ok {
        if c1[pos] == c2[pos] {
            pos += 1;
        } else {
            ok = false;
        }
    }

    ok
}

fn main() {
    let s1 = "rust";
    let s2 = "trus";
    let result = anagram_solution4(&s1, &s2);
    println!("s1 and s2 is anagram: {result}");
}
