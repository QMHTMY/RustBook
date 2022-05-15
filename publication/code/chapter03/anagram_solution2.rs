// anagram_solution2.rs

fn anagram_solution2(s1: &str, s2: &str) -> bool {
    if  s1.len() != s2.len() {
        return false;
    }

    // s1 和 s2 中的字符分别加入 vec_a, vec_b
    let mut vec_a = Vec::new();
    let mut vec_b = Vec::new();
    for c in s1.chars() { vec_a.push(c); }
    for c in s2.chars() { vec_b.push(c); }

    // pos1、pos2 索引字符
    let mut pos1: usize = 0;
    let mut pos2: usize;

    // 乱序字符串标示、控制循环
    let mut is_anagram = true;

    // 标示字符是否在 s2 中
    let mut found: bool;

    while pos1 < s1.len() && is_anagram {
        pos2 = 0;
        found = false;
        while pos2 < vec_b.len() && !found {
            if vec_a[pos1] == vec_b[pos2] {
                found = true;
            } else {
                pos2 += 1;
            }
        }

        // 某字符存在于 s2 中，将其替换成 ' ' 避免再次比较
        if found {
            vec_b[pos2]= ' ';
        } else {
            is_anagram = false;
        }

        // 处理 s1 中下一个字符
        pos1 += 1;
    }

    is_anagram
}

fn main() {
    let s1 = "rust";
    let s2 = "trus";
    let result = anagram_solution2(s1, s2);
    println!("s1 and s2 is anagram: {result}");
}
