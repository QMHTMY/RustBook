// infix_to_postfix.rs

use std::collections::HashMap;

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

    fn peek(&self) -> Option<&T> {
        if 0 == self.top { return None; }
        self.data.get(self.top - 1)
    }

    fn is_empty(&self) -> bool {
        0 == self.top
    }
}

// 同时检测多种括号
fn par_match(open: char, close: char) -> bool {
    let opens = "([{";
    let closers = ")]}";
    opens.find(open) == closers.find(close)
}

// 检测括号是否匹配
fn par_checker3(infix: &str) -> bool {
    let mut char_list = Vec::new();
    for c in infix.chars() {
        char_list.push(c);
    }

    let mut index = 0;
    let mut balance = true;
    let mut stack = Stack::new();
    while index < char_list.len() && balance {
        let c = char_list[index];
        if '(' == c || '[' == c || '{' == c {
            stack.push(c);
        }

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

        index += 1;
    }

    balance && stack.is_empty()
}

fn infix_to_postfix(infix: &str) -> Option<String> {
    // 括号匹配检验
    if !par_checker3(infix) { return None; }

    // 设置各个符号的优先级
    let mut prec = HashMap::new();
    prec.insert("(", 1); prec.insert(")", 1);
    prec.insert("+", 2); prec.insert("-", 2);
    prec.insert("*", 3); prec.insert("/", 3);

    // ops 保存操作符号、postfix 保存后缀表达式
    let mut ops = Stack::new();
    let mut postfix = Vec::new();
    for token in infix.split_whitespace() {
        if ("A" <= token && token <= "Z") || ("0" <= token && token <= "9") {
            // 0 - 9  和 A-Z 范围字符入栈
            postfix.push(token);
        } else if "(" == token  {
            // 遇到开括号，将操作符入栈
            ops.push(token);
        } else if ")" == token  {
            // 遇到闭括号，将操作数入栈
            let mut top = ops.pop().unwrap();
            while top != "(" {
                postfix.push(top);
                top = ops.pop().unwrap();
            }
        } else {
            // 比较符号的优先级来决定操作符号是否加入 postfix
            while !ops.is_empty() &&
                prec[ops.peek().unwrap()] >= prec[token] {
                postfix.push(ops.pop().unwrap());
            }
            ops.push(token);
        }
    }

    // 剩下的操作数入栈
    while !ops.is_empty() {
        postfix.push(ops.pop().unwrap())
    }

    // 出栈并组成字符串
    let mut postfix_str = "".to_string();
    for c in postfix {
        postfix_str += &c.to_string();
        postfix_str += " ";
    }

    Some(postfix_str)
}

fn main() {
    let infix = "( A + B ) * ( C + D )";
    let postfix = infix_to_postfix(infix);
    match postfix {
        Some(val) => {
            println!("{infix} -> {val}");
        },
        None => {
            println!("{infix} isn't a corret infix string");
        },
    }
}
