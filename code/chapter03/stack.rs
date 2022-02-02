#[derive(Debug)]
struct Stack<T> {
    top: usize,   // 栈顶
    data: Vec<T>, // 栈数据
}

impl<T> Stack<T> {
    fn new() -> Self { // 初始化空栈
        Stack {
            top: 0,
            data: Vec::new()
        }
    }

    fn push(&mut self, val: T) {
        self.data.push(val); // 数据保存在 Vec 末尾
        self.top += 1;
    }

    fn pop(&mut self) -> Option<T> {
        if self.top == 0 { return None; }
        self.top -= 1; // 栈顶减 1 后再弹出数据
        self.data.pop()
    }

    fn peek(&self) -> Option<&T> { // 数据不能移动，只能返回引用
        if self.top == 0 { return None; }
        self.data.get(self.top - 1)
    }

    fn is_empty(&self) -> bool {
        0 == self.top
    }

    fn size(&self) -> usize {
        self.top // 栈顶恰好就是栈中元素个数
    }
}

fn main() {
    let mut s = Stack::new();
    s.push(1); s.push(2); s.push(4);
    println!("top {:?}, size {}",s.peek().unwrap(), s.size());
    println!("pop {:?}, size {}",s.pop().unwrap(), s.size());
    println!("is_empty:{}, stack:{:?}", s.is_empty(), s);
}
