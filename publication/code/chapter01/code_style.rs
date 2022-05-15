// 枚举
enum Result<T, E> {
    Ok(T),
    Err(E),
}

// 特性，trait
pub trait From<T> {
    fn from<T> -> Self;
}

// 结构体
struct Rectangle {
    height: i32,
    width: i32,
}
impl Rectangle {
    // 构造函数
    fn new(height: i32, width: i32) -> Self {
        Self {
            height,
            width,
        }
    }

    // 函数
    fn calc_area(&self) -> i32 {
        self.height * self.width
    }
}

// 静态变量和常量
static NAME: &str = "kew";
const AGE: i32 = 25;

// 宏定义
macro_rules! add {
    ($a:expr, $b:expr) => {
        {
            $a + $b
        }
    }
}

// 变量及宏使用
let sum_of_nums = add!(1, 2);
