// big_o_analysis.rs
// + - * / 运算时钟周期不同

fn main() {
    let a = 1; let b = 2;
    let c = 3; let n = 1000000;

    for i in 0..n {
        for j in 0..n {
            let x = i * i;
            let y = j * j;
            let z = i * j;
            let t = i + i;
        }
    }

    for k in 0..n {
        let w = a * b - c + 45;
        let v = c * b;
    }

    let d = 2022;
}
