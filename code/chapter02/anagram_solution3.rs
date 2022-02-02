fn anagram_solution3(s1: &str, s2: &str) -> bool {
    if  s1.len() != s2.len() { return false; }

    // s1 和 s2 中的字符分别加入 alist, blist 并排序
    let mut alist = Vec::new();
    let mut blist = Vec::new();
    for c in s1.chars() { alist.push(c); }
    for c in s2.chars() { blist.push(c); }
    alist.sort(); blist.sort();

    // 逐个比较排序的集合，任何字符不匹配就退出循环
    let mut pos: usize = 0;
    let mut ok = true;
    while pos < alist.len() && ok {
        if alist[pos] == blist[pos] {
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
    let result = anagram_solution3(&s1, &s2);
    println!("s1 and s2 is anagram: {result}");
}
