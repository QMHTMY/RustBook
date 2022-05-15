// hash.rs

fn hash1(ascii_str: &str,  size: usize) -> usize {
    let mut sum = 0;
    for c in ascii_str.chars() {
        sum += c as usize;
    }

    sum % size
}

// hash1 的迭迭代形式
fn hash1_iter(ascii_str: &str, size: usize) -> usize {
    ascii_str.chars()
             .fold(0, |acc, c| acc + c as usize)
             % size
}

fn hash2(ascii_str: &str, size: usize) -> usize {
    let mut sum = 0;
    for (i, c) in ascii_str.chars().enumerate() {
        sum += (i + 1) * (c as usize);
    }

    sum % size
}

// hash2 的迭代形式
fn hash2_iter(ascii_str: &str, size: usize) -> usize {
    ascii_str.chars()
             .enumerate()
             .fold(0, |acc, (i, c)| acc + (i + 1) * (c as usize))
             % size
}

fn main() {
    let s1 = "rust";
    let s2 = "Rust";
    let size = 11;

    let p1 = hash1(s1, size);
    let p2 = hash1(s2, size);
    println!("{s1} in slot {p1}, {s2} in slot {p2}");

    let p1 = hash2(s1, size);
    let p2 = hash2(s2, size);
    println!("{s1} in slot {p1}, {s2} in slot {p2}");
}
