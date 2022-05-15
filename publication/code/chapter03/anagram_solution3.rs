// anagram_solution3.rs

fn anagram_solution3(s1: &str, s2: &str) -> bool {
    if  s1.len() != s2.len() {
        return false;
    }

    // s1 和 s2 中的字符分别加入 vec_a, vec_b 并排序
    let mut vec_a = Vec::new();
    let mut vec_b = Vec::new();
    for c in s1.chars() { vec_a.push(c); }
    for c in s2.chars() { vec_b.push(c); }
    vec_a.sort();
    vec_b.sort();

    // 逐个比较排序的集合，任何字符不匹配就退出循环
    let mut pos: usize = 0;
    let mut is_anagram = true;
    while pos < vec_a.len() && is_anagram {
        if vec_a[pos] == vec_b[pos] {
            pos += 1;
        } else {
            is_anagram = false;
        }
    }

    is_anagram
}

fn main() {
    let s1 = "rust";
    let s2 = "trus";
    let result = anagram_solution3(s1, s2);
    println!("s1 and s2 is anagram: {result}");
}
