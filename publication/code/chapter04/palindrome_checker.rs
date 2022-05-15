// palindrome_checker.rs

#[derive(Debug)]
struct Deque<T> {
    cap: usize,
    data: Vec<T>,
}

impl<T> Deque<T> {
    fn new(cap: usize) -> Self {
        Self {
            cap: cap,
            data: Vec::with_capacity(cap),
        }
    }

    fn add_rear(&mut self, val: T) -> Result<(), String> {
        if self.size() == self.cap {
            return Err("No space avaliabl".to_string());
        }
        self.data.insert(0, val);

        Ok(())
    }

    fn remove_front(&mut self) -> Option<T> {
        if self.size() > 0 {
            self.data.pop()
        } else {
            None
        }
    }

    fn remove_rear(&mut self) -> Option<T> {
        if self.size() > 0 {
            Some(self.data.remove(0))
        } else {
            None
        }
    }

    fn size(&self) -> usize {
        self.data.len()
    }
}

// 回文检查
fn palindrome_checker(pal: &str) -> bool {
    // 数据入队列
    let mut d = Deque::new(pal.len());
    for c in pal.chars() {
        let _r = d.add_rear(c);
    }

    let mut is_pal = true;
    while d.size() > 1 && is_pal {
        // 出队首尾字符
        let head = d.remove_front();
        let tail = d.remove_rear();

        // 比较首尾字符, 若不同则非回文
        if head != tail {
            is_pal = false;
        }
    }

    is_pal
}

fn main() {
    let pal = "rustsur";
    let is_pal = palindrome_checker(pal);
    println!("{pal} is palindrome string: {is_pal}");

    let pal = "panda";
    let is_pal = palindrome_checker(pal);
    println!("{pal} is palindrome string: {is_pal}");
}
