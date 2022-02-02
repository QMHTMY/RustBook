// hot_potato.rs

#[derive(Debug)]
struct Queue<T> {
    cap: usize,
    data: Vec<T>,
}

impl<T> Queue<T> {
    fn new(size: usize) -> Self {
        Queue {
            cap: size,
            data: Vec::with_capacity(size),
        }
    }

    fn enqueue(&mut self, val: T) -> Result<(), String> {
        if Self::size(&self) == self.cap {
            return Err("No space available".to_string());
        }
        self.data.insert(0, val);

        Ok(())
    }

    fn dequeue(&mut self) -> Option<T> {
        if self.size() > 0 {
            self.data.pop()
        } else {
            None
        }
    }

    fn size(&self) -> usize {
        self.data.len()
    }
}

fn hot_potato(names: Vec<&str>, num: usize) -> &str {
    // 初始化队列、名字入队
    let mut q = Queue::new(names.len());
    for name in names {
        let _nm = q.enqueue(name);
    }

    while q.size() > 1 {
        // 出入栈名字，相当于传递山芋
        for _i in 0..num {
            let name = q.dequeue().unwrap();
            let _rm = q.enqueue(name);
        }

        // 出入栈达到 num 次，删除一人
        let _rm = q.dequeue();
    }

    q.dequeue().unwrap()
}

fn main() {
    let names = vec!["Shieber", "Tom", "Kew", "Lisa", "Marry", "Bob"];
    let rem = hot_potato(names, 8);
    println!("The left person is {rem}");
}
