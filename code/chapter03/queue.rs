// queue.rs

// 队列
#[derive(Debug)]
struct Queue<T> {
    cap: usize,   // 容量
    data: Vec<T>, // 数据容器
}

impl<T> Queue<T> {
    fn new(size: usize) -> Self {
        Queue {
            cap: size,
            data: Vec::with_capacity(size),
        }
    }

    // 判断是否有剩余空间、有则数据加入队列
    fn enqueue(&mut self, val: T) -> Result<(), String> {
        if Self::size(&self) == self.cap {
            return Err("No space available".to_string());
        }
        self.data.insert(0, val);

        Ok(())
    }

    // 数据出队
    fn dequeue(&mut self) -> Option<T> {
        if self.size() > 0 {
            self.data.pop()
        } else {
            None
        }
    }

    fn is_empty(&self) -> bool {
        0 == self.size()
    }

    fn size(&self) -> usize {
        self.data.len()
    }
}

fn main() {
    let mut q = Queue::new(3);
    let _r1 = q.enqueue(1);
    let _r2 = q.enqueue(2);
    let _r3 = q.enqueue(3);
    if let Err(error) = q.enqueue(4) {
        println!("Enqueue error: {error}");
    }

    if let Some(data) = q.dequeue() {
        println!("data: {data}");
    } else {
        println!("empty queue");
    }

    println!("size: {}, empty: {}", q.size(), q.is_empty());
    println!("content: {:?}", q);
}
