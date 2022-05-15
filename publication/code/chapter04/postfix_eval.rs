// postfix_eval.rs

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
}

// 执行数学运算
fn do_calc(op: &str, op1: i32, op2: i32) -> i32 {
    if "+" == op  {
        op1 + op2
    } else if "-" == op {
        op1 - op2
    } else if "*" == op {
        op1 * op2
    } else if "/" == op {
        if 0 == op2 {
            panic!("ZeroDivisionError: Invalid operation!");
        }
        op1 / op2
    } else {
        panic!("OperatorError: Invalid operator: {:?}", op);
    }
}

fn postfix_eval(postfix: &str) -> Option<i32> {
    // 少于五个字符，不是有效的后缀表达式，因为表达式
    // 至少两个操作数加一个操作符，还需要两个空格隔开
    if postfix.len() < 5 { return None; }

    // 操作数栈
    let mut ops = Stack::new();
    for token in postfix.split_whitespace() {
        if "0" <= token  && token <= "9" {
            ops.push(token.parse::<i32>().unwrap());
        } else {
            // 对于减法和除法，顺序有要求
            // 所以先出栈的是第二个操作数
            let op2 = ops.pop().unwrap();
            let op1 = ops.pop().unwrap();
            let res = do_calc(token, op1, op2);
            ops.push(res);
        }
    }

    Some(ops.pop().unwrap())
}

fn main() {
    let postfix = "1 2 + 1 2 + *";
    let res = postfix_eval(postfix);
    match res {
        Some(val) => println!("res = {val}"),
        None => println!("{postfix} isn't a corret postfix"),
    }
}
