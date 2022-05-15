// par_checker1.rs

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

fn par_checker1(par: &str) -> bool {
    // 字符加入 Vec
    let mut char_list = Vec::new();
    for c in par.chars() {
        char_list.push(c);
    }

    let mut index = 0;
    let mut balance = true; // 括号是否匹配(平衡)标示
    let mut stack = Stack::new();
    while index < char_list.len() && balance {
        let c = char_list[index];

        if '(' == c { // 如果为开符号, 入栈
            stack.push(c);
        } else { // 如果为闭符号, 判断栈是否为空
            if stack.is_empty() { // 为空, 所以不匹配
                balance = false;
            } else {
                let _r = stack.pop();
            }
        }

        index += 1;
    }

    // 平衡且栈为空时，括号表达式才是匹配的
    balance && stack.is_empty()
}

fn main() {
    let sa = "()(())";
    let sb = "()((()";
    let res1 = par_checker1(sa);
    let res2 = par_checker1(sb);
    println!("{sa} balanced: {res1}, {sb} balanced: {res2}");
}
