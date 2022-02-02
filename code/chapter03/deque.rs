// deque.rs

// 双端队列
#[derive(Debug)]
struct Deque<T> {
    cap: usize,   // 容量
    data: Vec<T>, // 数据容器
}

impl<T> Deque<T> {
    fn new(cap: usize) -> Self {
        Deque {
            cap: cap,
            data: Vec::with_capacity(cap),
        }
    }

    // Vec 末尾为队首
    fn add_front(&mut self, val: T) -> Result<(), String> {
        if self.size() == self.cap {
            return Err("No space avaliable".to_string());
        }
        self.data.push(val);

        Ok(())
    }

    // Vec 首部为队尾
    fn add_rear(&mut self, val: T) -> Result<(), String> {
        if self.size() == self.cap {
            return Err("No space avaliabl".to_string());
        }
        self.data.insert(0, val);

        Ok(())
    }

    // 从队首移除数据
    fn remove_front(&mut self) -> Option<T> {
        if self.size() > 0 {
            self.data.pop()
        } else {
            None
        }
    }

    // 从队尾移除数据
    fn remove_rear(&mut self) -> Option<T> {
        if self.size() > 0 {
            Some(self.data.remove(0))
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
    let mut d = Deque::new(4);
    let _r1 = d.add_front(1); let _r2 = d.add_front(2);
    let _r1 = d.add_rear(3); let _r2 = d.add_rear(4);
    if let Err(error) = d.add_front(5) {
        println!("add_front error: {error}");
    }

    if let Some(data) = d.remove_rear() {
        println!("remove data {data}");
    } else {
        println!("empty deque");
    }

    println!("size: {}, is_empyt: {}", d.size(), d.is_empty());
    println!("content: {:?}", d);
}
