// deque.rs

// 双端队列
#[derive(Debug)]
struct Deque<T> {
    cap: usize,   // 容量
    data: Vec<T>, // 数据容器
}

impl<T> Deque<T> {
    fn new(cap: usize) -> Self {
        Self {
            cap: cap,
            data: Vec::with_capacity(cap),
        }
    }

    fn is_empty(&self) -> bool {
        0 == self.len()
    }

    fn is_full(&self) -> bool {
        self.len() == self.cap
    }

    fn len(&self) -> usize {
        self.data.len()
    }

    fn clear(&mut self) {
        self.data = Vec::with_capacity(self.cap);
    }

    // Vec 末尾为队首
    fn add_front(&mut self, val: T) -> Result<(), String> {
        if self.len() == self.cap {
            return Err("No space avaliable".to_string());
        }
        self.data.push(val);

        Ok(())
    }

    // Vec 首部为队尾
    fn add_rear(&mut self, val: T) -> Result<(), String> {
        if self.len() == self.cap {
            return Err("No space avaliabl".to_string());
        }
        self.data.insert(0, val);

        Ok(())
    }

    // 从队首移除数据
    fn remove_front(&mut self) -> Option<T> {
        if self.len() > 0 {
            self.data.pop()
        } else {
            None
        }
    }

    // 从队尾移除数据
    fn remove_rear(&mut self) -> Option<T> {
        if self.len() > 0 {
            Some(self.data.remove(0))
        } else {
            None
        }
    }

    // 以下是为双端队列实现的迭代功能
    // into_iter: 双端队列改变，成为迭代器
    // iter: 双端队列不变，只得到不可变迭代器
    // iter_mut: 双端队列不变，得到可变迭代器
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
struct IntoIter<T>(Deque<T>);
impl<T: Clone> Iterator for IntoIter<T> {
    type Item = T;
    fn next(&mut self) -> Option<Self::Item> {
        if !self.0.is_empty() {
            Some(self.0.data.remove(0))
        } else {
            None
        }
    }
}

struct Iter<'a, T: 'a> { stack: Vec<&'a T>, }
impl<'a, T> Iterator for Iter<'a, T> {
    type Item = &'a T;
    fn next(&mut self) -> Option<Self::Item> {
        if 0 != self.stack.len() {
            Some(self.stack.remove(0))
        } else {
            None
        }
    }
}

struct IterMut<'a, T: 'a> { stack: Vec<&'a mut T> }
impl<'a, T> Iterator for IterMut<'a, T> {
    type Item = &'a mut T;
    fn next(&mut self) -> Option<Self::Item> {
        if 0 != self.stack.len() {
            Some(self.stack.remove(0))
        } else {
            None
        }
    }
}

fn main() {
    basic();
    iter();

    fn basic() {
        let mut d = Deque::new(4);
        let _r1 = d.add_front(1);
        let _r2 = d.add_front(2);
        let _r3 = d.add_rear(3);
        let _r4 = d.add_rear(4);
        if let Err(error) = d.add_front(5) {
            println!("add_front error: {error}");
        }
        println!("{:?}", d);

        match d.remove_rear() {
            Some(data) => println!("remove rear data {data}"),
            None => println!("empty deque"),
        }
        match d.remove_front() {
            Some(data) => println!("remove front data {data}"),
            None => println!("empty deque"),
        }
        println!("empty: {}, len: {}", d.is_empty(), d.len());
        println!("full: {}, {:?}", d.is_full(), d);

        d.clear();
        println!("{:?}", d);
    }

    fn iter() {
        let mut d = Deque::new(4);
        let _r1 = d.add_front(1);
        let _r2 = d.add_front(2);
        let _r3 = d.add_rear(3);
        let _r4 = d.add_rear(4);

        let sum1 = d.iter().sum::<i32>();
        let mut addend = 0;
        for item in d.iter_mut() {
            *item += 1;
            addend += 1;
        }
        let sum2 = d.iter().sum::<i32>();
        println!("{sum1} + {addend} = {sum2}");

        assert_eq!(14, d.into_iter().sum::<i32>());
    }
}
