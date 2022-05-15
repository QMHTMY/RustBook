// par_checker3.rs

#[derive(Debug)]
struct Stack<T> {
    top: usize,
    data: Vec<T>,
}

impl<T> Stack<T> {
    fn new() -> Self {
        Self {
            top: 0,
            data: Vec::new()
        }
    }

    fn push(&mut self, val: T) {
        self.data.push(val);
        self.top += 1;
    }

    fn pop(&mut self) -> Option<T> {
        if 0 == self.top { return None; }
        self.top -= 1;
        self.data.pop()
    }

    fn is_empty(&self) -> bool {
        0 == self.top
    }
}

// 同时检测多种开闭符号是否匹配
fn par_match(open: char, close: char) -> bool {
    let opens = "([{";
    let closers = ")]}";
    opens.find(open) == closers.find(close)
}

fn par_checker3(par: &str) -> bool {
    let mut char_list = Vec::new();
    for c in par.chars() {
        char_list.push(c);
    }

    let mut index = 0;
    let mut balance = true;
    let mut stack = Stack::new();
    while index < char_list.len() && balance {
        let c = char_list[index];

        // 开符号入栈
        if '(' == c || '[' == c || '{' == c {
            stack.push(c);
        }

        // 闭符号则判断是否平衡
        if ')' == c || ']' == c || '}' == c {
            if stack.is_empty() {
                balance = false;
            } else {
                let top = stack.pop().unwrap();
                if !par_match(top, c) {
                    balance = false;
                }
            }
        }

        // 非括号直接跳过
        index += 1;
    }

    balance && stack.is_empty()
}

fn main() {
    let sa = "(2+3){func}[abc]";
    let sb = "(2+3)*(3-1";
    let res1 = par_checker3(sa);
    let res2 = par_checker3(sb);
    println!("{sa} balanced: {res1}, {sb} balanced: {res2}");
}
