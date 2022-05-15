// stack.rs

#[derive(Debug)]
struct Stack<T> {
    size: usize,  // 栈大小
    data: Vec<T>, // 栈数据
}

impl<T> Stack<T> {
    // 初始化空栈
    fn new() -> Self {
        Self {
            size: 0,
            data: Vec::new()
        }
    }

    fn is_empty(&self) -> bool {
        0 == self.size
    }

    fn len(&self) -> usize {
        self.size
    }

    // 清空栈
    fn clear(&mut self) {
        self.size = 0;
        self.data.clear();
    }

    // 数据保存在 Vec 末尾
    fn push(&mut self, val: T) {
        self.data.push(val);
        self.size += 1;
    }

    // 栈顶减 1 后再弹出数据
    fn pop(&mut self) -> Option<T> {
        if 0 == self.size { return None; }
        self.size -= 1;
        self.data.pop()
    }

    // 返回栈顶数据引用和可变引用
    fn peek(&self) -> Option<&T> {
        if 0 == self.size { return None; }
        self.data.get(self.size - 1)
    }

    fn peek_mut(&mut self) -> Option<&mut T> {
        if 0 == self.size { return None; }
        self.data.get_mut(self.size - 1)
    }

    // 以下是为栈实现的迭代功能
    // into_iter: 栈改变，成为迭代器
    // iter: 栈不变，得到不可变迭代器
    // iter_mut: 栈不变，得到可变迭代器
    fn into_iter(self) -> IntoIter<T> {
        IntoIter(self)
    }

    fn iter(&self) -> Iter<T> {
        let mut iterator = Iter { stack: Vec::new() };
        for item in self.data.iter() {
            iterator.stack.push(item);
        }

        iterator
    }

    fn iter_mut(&mut self) -> IterMut<T> {
        let mut iterator = IterMut { stack: Vec::new() };
        for item in self.data.iter_mut() {
            iterator.stack.push(item);
        }

        iterator
    }
}

// 实现三种迭代功能
struct IntoIter<T>(Stack<T>);
impl<T: Clone> Iterator for IntoIter<T> {
    type Item = T;
    fn next(&mut self) -> Option<Self::Item> {
        if !self.0.is_empty() {
            self.0.size -= 1;
            self.0.data.pop()
        } else {
            None
        }
    }
}

struct Iter<'a, T: 'a> { stack: Vec<&'a T>, }
impl<'a, T> Iterator for Iter<'a, T> {
    type Item = &'a T;
    fn next(&mut self) -> Option<Self::Item> {
        self.stack.pop()
    }
}

struct IterMut<'a, T: 'a> { stack: Vec<&'a mut T> }
impl<'a, T> Iterator for IterMut<'a, T> {
    type Item = &'a mut T;
    fn next(&mut self) -> Option<Self::Item> {
        self.stack.pop()
    }
}

fn main() {
    basic();
    peek();
    iter();

    fn basic() {
        let mut s = Stack::new();
        s.push(1); s.push(2); s.push(3);

        println!("size: {}, {:?}", s.len(), s);
        println!("pop {:?}, size {}",s.pop().unwrap(), s.len());
        println!("empty: {}, {:?}", s.is_empty(), s);

        s.clear();
        println!("{:?}", s);
    }

    fn peek() {
        let mut s = Stack::new();
        s.push(1); s.push(2); s.push(3);

        println!("{:?}", s);
        let peek_mut = s.peek_mut();
        if let Some(top) = peek_mut {
            *top = 4;
        }

        println!("top {:?}", s.peek().unwrap());
        println!("{:?}", s);
    }

    fn iter() {
        let mut s = Stack::new();
        s.push(1); s.push(2); s.push(3);

        let sum1 = s.iter().sum::<i32>();
        let mut addend = 0;
        for item in s.iter_mut() {
            *item += 1;
            addend += 1;
        }

        let sum2 = s.iter().sum::<i32>();
        println!("{sum1} + {addend} = {sum2}");
        assert_eq!(9, s.into_iter().sum::<i32>());
    }
}
