// type_transfer.rs

#![allow(overflowing_literals)]      // 忽略类型转换的溢出警告
fn main() {
    let decimal = 61.3214_f32;
    // let integer: u8 = decimal;    // 报错，不能将 f32 转 u8
    let integer = decimal as u8;     // 正确，用 as 显示转换
    let character = integer as char;
    println!("1000 as a u16: {}", 1000 as u16);
    println!("1000 as a u8: {}", 1000 as u8);
}
