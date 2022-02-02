#[derive(Debug)]
struct Stack<T> {
    top: usize,
    data: Vec<T>,
}

impl<T> Stack<T> {
    fn new() -> Self {
        Stack {
            top: 0,
            data: Vec::new()
        }
    }

    fn push(&mut self, val: T) {
        self.data.push(val);
        self.top += 1;
    }

    fn pop(&mut self) -> Option<T> {
        if self.top == 0 { return None; }
        self.top -= 1;
        self.data.pop()
    }

    fn is_empty(&self) -> bool {
        0 == self.top
    }
}

fn divide_by_two(mut dec_num: u32) -> String {
    // 用栈来保存余数 rem
    let mut rem_stack = Stack::new();

    // 余数 rem 入栈
    while dec_num > 0 {
        let rem = dec_num % 2;
        rem_stack.push(rem);
        dec_num /= 2;
    }

    // 栈中元素出栈组成字符串
    let mut bin_str = "".to_string();
    while !rem_stack.is_empty() {
        let rem = rem_stack.pop().unwrap().to_string();
        bin_str += &rem;
    }

    bin_str
}

fn main() {
    let bin_str: String = divide_by_two(10);
    println!("10 is b{bin_str}");
}
