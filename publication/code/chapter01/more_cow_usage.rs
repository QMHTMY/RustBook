// more_cow_usage.rs

use std::borrow::Cow;

fn abs_all(input: &mut Cow<[i32]>) {
    for i in 0..input.len() {
        let v = input[i];
        if v < 0 {
            input.to_mut()[i] = -v;
        }
    }
}

fn main() {
    // 只读，不写，没有发生复制操作
    let a = [0, 1, 2];
    let mut input = Cow::from(&a[..]);
    abs_all(&mut input);
    assert_eq!(input, Cow::Borrowed(a.as_ref()));

    // 写时复制， 在读到-1的时候发生复制
    let b = [0, -1, -2];
    let mut input = Cow::from(&b[..]);
    abs_all(&mut input);
    assert_eq!(input, Cow::Owned(vec![0,1,2]) as Cow<[i32]>);

    // 没有写时复制，因为已经拥有所有权
    let mut input = Cow::from(vec![0, -1, -2]);
    abs_all(&mut input);
    assert_eq!(input, Cow::Owned(vec![0,1,2]) as Cow<[i32]>);

    let v = input.into_owned();
    assert_eq!(v, [0, 1, 2]);
}
