// rust_example.rs

// 从标准库中导入 max 函数
use std::cmp::max;

// 公开模块
pub mod math {
    // 公开函数
    pub fn add(x: i32, y: i32) -> i32 {
        x + y
    }

    // 私有函数
    fn is_zero(num: i32) -> bool {
        0 == num
    }
}

# 结构体
#[derive(Debug)]
struct Circle {
    radius: f32, // 半径
}

// 为 f32 实现到 Circle 的转换功能
impl From<f32> for Circle {
    fn from(radius: f32) -> Self {
        Self { radius }
    }
}

// 注释：自定义函数
fn calc_func(num1:i32, num2:i32) -> i32 {
    let x = 5;
    let y = {
        let x = 3;
        x + 1 // 表达式
    }; // 语句

    max(x, y)
}

// 使用模块函数
use math::add;

// 主函数
fn main() {
    let num1 = 1; let num2 = 2;

    // 函数调用
    println!("num1 + num2 = {}", add(num1, num2));
    println!("res = {}", calc_func(num1, num2));

    let f: f32 = 9.85;
    let c: Circle = Circle::from(f);
    println!("{:?}", c);
}
