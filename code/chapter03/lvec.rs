// lvec.rs
use std::fmt::Debug;

// 节点
#[derive(Debug)]
struct Node<T> {
    elem: T,
    next: Link<T>,
}

type Link<T> = Option<Box<Node<T>>>;

impl<T> Node<T> {
    fn new(elem: T) -> Node<T> {
        Node {
            elem: elem,
            next: None
        }
    }
}

// 链表 Vec
#[derive(Debug)]
struct LVec<T> {
    size: usize,
    head: Link<T>,
}

impl<T: Copy + Debug> LVec<T> {
    fn new() -> Self {
        LVec {
            size: 0,
            head: None
        }
    }

    fn clear(&mut self) {
        self.size = 0;
        self.head = None;
    }

    fn len(&self) -> usize {
        self.size
    }

    fn is_empty(&self) -> bool {
        0 == self.size
    }

    fn push(&mut self, elem: T) {
        let node = Node::new(elem);
        if self.is_empty() {
            self.head = Some(Box::new(node));
        } else {
            let mut curr = self.head.as_mut().unwrap();

            // 找到链表最后一个节点
            for _i in 0..self.size-1 {
                curr = curr.next.as_mut().unwrap();
            }

            // 在最后一个节点后插入新数据
            curr.next = Some(Box::new(node));
        }

        self.size += 1;
    }

    // 栈末尾加入数据
    fn append(&mut self, other: &mut Self) {
        while let Some(node) = other.head.as_mut().take() {
            self.push(node.elem);
            other.head = node.next.take();
        }
        other.clear();
    }

    fn insert(&mut self, mut index: usize, elem: T) {
        if index >= self.size { index = self.size; }

        // 分三种情况插入新节点
        let mut node = Node::new(elem);
        if self.is_empty() { // LVec 为空
            self.head = Some(Box::new(node));
        } else if index == 0 { // 插入链表首部
            node.next = self.head.take();
            self.head = Some(Box::new(node));
        } else { // 插入链表中间
            let mut curr = self.head.as_mut().unwrap();
            for _i in 0..index - 1 { // 找到插入位置
                curr = curr.next.as_mut().unwrap();
            }
            node.next = curr.next.take();
            curr.next = Some(Box::new(node));
        }
        self.size += 1;
    }

    fn pop(&mut self) -> Option<T> {
        self.remove(self.size - 1)
    }

    fn remove(&mut self, index: usize) -> Option<T> {
        if index >= self.size { return None; }

        // 分两种情况删除节点，首节点删除最好处理
        let mut node;
        if 0 == index {
            node = self.head.take().unwrap();
            self.head = node.next.take();
        } else { // 非首节点需要找到待删除节点，并处理前后链接
            let mut curr = self.head.as_mut().unwrap();
            for _i in 0..index-1 {
                curr = curr.next.as_mut().unwrap();
            }
            node = curr.next.take().unwrap();
            curr.next = node.next.take();
        }
        self.size -= 1;

        Some(node.elem)
    }

    // 打印 LVec，当然也可以实习 ToString 特性
    fn print_lvec(&self) {
        let mut curr = self.head.as_ref();
        while let Some(node) = curr {
            println!("lvec val: {:#?}", node.elem);
            curr = node.next.as_ref();
        }
    }
}

fn main() {
    let mut lvec: LVec<i32> = LVec::new();
    lvec.push(10); lvec.push(11);
    lvec.push(12); lvec.push(13);
    lvec.insert(0,9);

    let mut lvec2: LVec<i32> = LVec::new();
    lvec2.insert(0, 8);
    lvec2.append(&mut lvec);
    println!("lvec2 len: {}", lvec2.len());
    lvec2.print_lvec();

    let res1 = lvec2.pop();
    let res2 = lvec2.remove(0);
    println!("pop {:?}", res1.unwrap());
    println!("remove {:?}", res2.unwrap());
    lvec2.print_lvec();
}
