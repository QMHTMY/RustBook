// integer_to_complex.rs

#[derive(Debug)]
struct Complex {
    real: i32, // 实部
    imag: i32  // 虚部
}

 // 为 i32 实现到复数的转换功能，i32 转换为实部，虚部置 0
impl From<i32> for Complex {
    fn from(real: i32) -> Self {
        Complex {
            real,
            imag: 0
        }
    }
}

fn main() {
    let c1: Complex = Complex::from(2_i32);
    let c2: Complex = 2_i32.into(); // 默认实现了 Into
    println!("c1: {:?}, c2: {:?}", c1, c2);
}
