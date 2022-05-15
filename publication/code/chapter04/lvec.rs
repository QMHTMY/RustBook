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
    fn new(elem: T) -> Self {
        Self {
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
        Self {
            size: 0,
            head: None
        }
    }

    fn is_empty(&self) -> bool {
        0 == self.size
    }

    fn len(&self) -> usize {
        self.size
    }

    fn clear(&mut self) {
        self.size = 0;
        self.head = None;
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

    // 栈末尾加入新的 LVec
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
        if self.size < 1 {
            return None;
        } else {
            self.remove(self.size - 1)
        }
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
            for _i in 0..index - 1 {
                curr = curr.next.as_mut().unwrap();
            }
            node = curr.next.take().unwrap();
            curr.next = node.next.take();
        }
        self.size -= 1;

        Some(node.elem)
    }

    // 以下是为栈实现的迭代功能
    // into_iter: 栈改变，成为迭代器
    // iter: 栈不变，只得到不可变迭代器
    // iter_mut: 栈不变，得到可变迭代器
    fn into_iter(self) -> IntoIter<T> {
        IntoIter(self)
    }

    fn iter(&self) -> Iter<T> {
        Iter { next: self.head.as_deref() }
    }

    fn iter_mut(&mut self) -> IterMut<T> {
        IterMut { next: self.head.as_deref_mut() }
    }

    // 打印 LVec
    fn print_lvec(&self) {
        if 0 == self.size {
            println!("Empty lvec");
        }

        for item in self.iter() {
            println!("{:?}", item);
        }
    }
}

// 实现三种迭代功能
struct IntoIter<T: Copy + Debug >(LVec<T>);
impl<T: Copy + Debug> Iterator for IntoIter<T> {
    type Item = T;
    fn next(&mut self) -> Option<Self::Item> {
        self.0.pop()
    }
}

struct Iter<'a, T: 'a> { next: Option<&'a Node<T>> }
impl<'a, T> Iterator for Iter<'a, T> {
    type Item = &'a T;
    fn next(&mut self) -> Option<Self::Item> {
        self.next.map(|node| {
            self.next = node.next.as_deref();
            &node.elem
        })
    }
}

struct IterMut<'a, T: 'a> { next: Option<&'a mut Node<T>> }
impl<'a, T> Iterator for IterMut<'a, T> {
    type Item = &'a mut T;
    fn next(&mut self) -> Option<Self::Item> {
        self.next.take().map(|node| {
            self.next = node.next.as_deref_mut();
            &mut node.elem
        })
    }
}

fn main() {
    basic();
    iter();

    fn basic() {
        let mut lvec1: LVec<i32> = LVec::new();
        lvec1.push(10); lvec1.push(11);
        lvec1.push(12); lvec1.push(13);
        lvec1.insert(0,9);

        lvec1.print_lvec();

        let mut lvec2: LVec<i32> = LVec::new();
        lvec2.insert(0, 8);
        lvec2.append(&mut lvec1);

        println!("len: {}", lvec2.len());
        println!("pop {:?}", lvec2.pop().unwrap());
        println!("remove {:?}", lvec2.remove(0).unwrap());

        lvec2.print_lvec();
        lvec2.clear();
        lvec2.print_lvec();
    }

    fn iter() {
        let mut lvec: LVec<i32> = LVec::new();
        lvec.push(10); lvec.push(11);
        lvec.push(12); lvec.push(13);

        let sum1 = lvec.iter().sum::<i32>();
        let mut addend = 0;
        for item in lvec.iter_mut() {
            *item += 1;
            addend += 1;
        }
        let sum2 = lvec.iter().sum::<i32>();
        println!("{sum1} + {addend} = {sum2}");

        assert_eq!(50, lvec.into_iter().sum::<i32>());
    }
}
