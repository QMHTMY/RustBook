fn anagram_solution2(s1: &str, s2: &str) -> bool {
    if  s1.len() != s2.len() { return false; }

    // s1 和 s2 中的字符分别加入 alist, blist
    let mut alist = Vec::new();
    let mut blist = Vec::new();
    for c in s1.chars() { alist.push(c); }
    for c in s2.chars() { blist.push(c); }

    let mut pos1: usize = 0; // pos1、pos2 索引字符
    let mut ok = true; // 乱序字符串标示、控制循环
    while pos1 < s1.len() && ok {
        let mut pos2: usize = 0;
        let mut found = false; // 标示字符是否在 s2 中
        while pos2 < blist.len() && !found {
            if alist[pos1] == blist[pos2] {
                found = true;
            } else {
                pos2 += 1;
            }
        }

        // 某字符存在于 s2 中，将其替换成 ' ' 避免再次比较
        if found {
            blist[pos2]= ' ';
        } else {
            ok = false;
        }

        // 处理 s1 中下一个字符
        pos1 += 1;
    }

    ok
}

fn main() {
    let s1 = "rust";
    let s2 = "trus";
    let result = anagram_solution2(&s1, &s2);
    println!("s1 and s2 is anagram: {result}");
}
