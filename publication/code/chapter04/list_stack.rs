// list_stack.rs

// 链表节点
#[derive(Debug, Clone)]
struct Node<T> {
    data: T,
    next: Link<T>,
}

// Node 自包含引用
type Link<T> = Option<Box<Node<T>>>;

impl<T> Node<T> {
    fn new(data: T) -> Self {
        Self {
            data: data,
            next: None // 初始化时无下一链接
        }
    }
}

// 链表栈
#[derive(Debug, Clone)]
struct LStack<T> {
    size: usize,
    top: Link<T>, // 栈顶控制整个栈
}

impl<T: Clone> LStack<T> {
    fn new() -> Self {
        Self {
            size: 0,
            top: None
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
        self.top = None;
    }

    // take 取出 top 中节点，留下空位，可以回填
    fn push(&mut self, val: T) {
        let mut node = Node::new(val);
        node.next = self.top.take();
        self.top = Some(Box::new(node));
        self.size += 1;
    }

    fn pop(&mut self) -> Option<T> {
        self.top.take().map(|node| {
            let node = *node;
            self.top = node.next;
            self.size -= 1;
            node.data
        })
    }

    // 返回链表栈数据引用和可变引用
    fn peek(&self) -> Option<&T> {
        self.top.as_ref().map(|node| &node.data)
    }

    fn peek_mut(&mut self) -> Option<&mut T> {
        self.top.as_deref_mut().map(|node| &mut node.data)
    }

    // 以下是为链表栈实现的迭代功能
    // into_iter: 链表栈改变，成为迭代器
    // iter: 链表栈不变，只得到不可变迭代器
    // iter_mut: 链表栈不变，得到可变迭代器
    fn into_iter(self) -> IntoIter<T> {
        IntoIter(self)
    }

    fn iter(&self) -> Iter<T> {
        Iter { next: self.top.as_deref() }
    }

    fn iter_mut(&mut self) -> IterMut<T> {
        IterMut { next: self.top.as_deref_mut() }
    }
}

// 实现三种迭代功能
struct IntoIter<T: Clone>(LStack<T>);
impl<T: Clone> Iterator for IntoIter<T> {
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
            &node.data
        })
    }
}

struct IterMut<'a, T: 'a> { next: Option<&'a mut Node<T>> }
impl<'a, T> Iterator for IterMut<'a, T> {
    type Item = &'a mut T;
    fn next(&mut self) -> Option<Self::Item> {
        self.next.take().map(|node| {
            self.next = node.next.as_deref_mut();
            &mut node.data
        })
    }
}

fn main() {
    basic();
    iter();

    fn basic() {
        let mut s = LStack::new();
        s.push(1); s.push(2); s.push(3);

        println!("empty: {:?}", s.is_empty());
        println!("top: {:?}, size: {}", s.peek(), s.len());
        println!("pop: {:?}, size: {}", s.pop(), s.len());

        let peek_mut = s.peek_mut();
        if let Some(data) = peek_mut {
            *data = 4
        }
        println!("top {:?}, size {}", s.peek(), s.len());

        println!("{:?}", s);
        s.clear();
        println!("{:?}", s);
    }

    fn iter() {
        let mut s = LStack::new();
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
