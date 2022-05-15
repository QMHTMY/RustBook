// queue.rs

// 队列定义
#[derive(Debug)]
struct Queue<T> {
    cap: usize,   // 容量
    data: Vec<T>, // 数据容器
}

impl<T> Queue<T> {
    fn new(size: usize) -> Self {
        Self {
            cap: size,
            data: Vec::with_capacity(size),
        }
    }

    fn is_empty(&self) -> bool {
        0 == Self::len(&self)
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

    // 判断是否有剩余空间、有则数据加入队列
    fn enqueue(&mut self, val: T) -> Result<(), String> {
        if self.len() == self.cap {
            return Err("No space available".to_string());
        }
        self.data.insert(0, val);

        Ok(())
    }

    // 数据出队
    fn dequeue(&mut self) -> Option<T> {
        if self.len() > 0 {
            self.data.pop()
        } else {
            None
        }
    }

    // 以下是为队列实现的迭代功能
    // into_iter: 队列改变，成为迭代器
    // iter: 队列不变，只得到不可变迭代器
    // iter_mut: 队列不变，得到可变迭代器
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
struct IntoIter<T>(Queue<T>);
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
        let mut q = Queue::new(4);
        let _r1 = q.enqueue(1); let _r2 = q.enqueue(2);
        let _r3 = q.enqueue(3); let _r4 = q.enqueue(4);
        if let Err(error) = q.enqueue(5) {
            println!("Enqueue error: {error}");
        }

        if let Some(data) = q.dequeue() {
            println!("dequeue data: {data}");
        } else {
            println!("empty queue");
        }

        println!("empty: {}, len: {}", q.is_empty(), q.len());
        println!("full: {}, {:?}", q.is_full(), q);

        q.clear();
        println!("{:?}", q);
    }

    fn iter() {
        let mut q = Queue::new(4);
        let _r1 = q.enqueue(1); let _r2 = q.enqueue(2);
        let _r3 = q.enqueue(3); let _r4 = q.enqueue(4);

        let sum1 = q.iter().sum::<i32>();
        let mut addend = 0;
        for item in q.iter_mut() {
            *item += 1;
            addend += 1;
        }
        let sum2 = q.iter().sum::<i32>();
        println!("{sum1} + {addend} = {sum2}");

        assert_eq!(14, q.into_iter().sum::<i32>());
    }
}
