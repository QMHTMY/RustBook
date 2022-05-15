// use_cow.rs

use std::borrow::Cow;

fn delete_spaces(src: &str) -> String {
    let mut dest = String::with_capacity(src.len());
    for c in src.chars() {
        if ' ' != c {
            dest.push(c);
        }
    }

    dest
}

fn delete_spaces2<'a>(src: &'a str) -> Cow<'a, str> {
    if src.contains(' ') {
        let mut dest = String::with_capacity(src.len());
        for c in src.chars() {
            if ' ' != c { dest.push(c); }
        }
        return Cow::Owned(dest); // 获取所有权，dest 被移出
    }

    return Cow::Borrowed(src); // 直接获取 src 的引用，
}

fn main() {
    let s = "i love you";
    let res1 = delete_spaces(s);
    let res2 = delete_spaces2(s);
    println!("{res1}, {res2}");

    let s = "iloveyou";
    let res1 = delete_spaces(s);
    let res2 = delete_spaces2(s);
    println!("{res1}, {res2}");
}
