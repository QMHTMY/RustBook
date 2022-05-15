// binary_tree.rs

use std::cmp::{max, Ordering::*};
use std::fmt::{Debug, Display};

// 此队列用于层序遍历
#[derive(Debug)]
struct Queue<T> {
    cap: usize,
    data: Vec<T>,
}

impl<T> Queue<T> {
    fn new(size: usize) -> Self {
        Self {
            cap: size,
            data: Vec::with_capacity(size),
        }
    }

    fn is_empty(&self) -> bool {
        0 == self.len()
    }

    fn len(&self) -> usize {
        self.data.len()
    }

    fn enqueue(&mut self, val: T) -> Result<(), String> {
        if Self::len(&self) == self.cap {
            return Err("No space available".to_string());
        }
        self.data.insert(0, val);

        Ok(())
    }

    fn dequeue(&mut self) -> Option<T> {
        if self.len() > 0 {
            self.data.pop()
        } else {
            None
        }
    }
}

// 二叉树子节点链接
type Link<T> = Option<Box<BinaryTree<T>>>;

// 二叉树定义
// key 保存数据、left 和 right 保存左右子节点
#[derive(Debug, Clone, PartialEq)]
struct BinaryTree<T> {
    key: T,
    left: Link<T>,
    right: Link<T>,
}

impl<T: Clone + Ord + ToString + Debug> BinaryTree<T> {
    // 构建新树
    fn new(key: T) -> Self {
        Self {
            key: key,
            left: None,
            right: None,
        }
    }

    // 计算节点数
    fn size(&self) -> usize {
        self.calc_size(0)
    }

    fn calc_size(&self, mut size: usize) -> usize {
        size += 1;

        if !self.left.is_none() {
            size = self.left.as_ref().unwrap().calc_size(size);
        }
        if !self.right.is_none() {
            size = self.right.as_ref().unwrap().calc_size(size);
        }

        size
    }

    // 计算叶节点数
    fn leaf_size(&self) -> usize {
        // 都为空，当前节点就是叶节点，返回 1
        if self.left.is_none() && self.right.is_none() {
            return 1;
        }

        // 计算左右子树的叶节点数
        let left_leaf = match &self.left {
            Some(left) => left.leaf_size(),
            None => 0,
        };
        let right_leaf = match &self.right {
            Some(right) => right.leaf_size(),
            None => 0,
        };

        // 左右子树叶节点数之和 = 总叶节点数
        left_leaf + right_leaf
    }

    // 计算非叶节点数 [千万不要想复杂了]
    fn none_leaf_size(&self) -> usize {
        self.size() - self.leaf_size()
    }

    // 计算树深度
    fn depth(&self) -> usize {
        let mut left_depth = 1;
        if let Some(left) = &self.left {
            left_depth += left.depth();
        }

        let mut right_depth = 1;
        if let Some(right) = &self.right {
            right_depth += right.depth();
        }

        // 取左右子树深度的最大值
        max(left_depth, right_depth)
    }

    // 获取左右子树
    fn get_left(&self) -> Link<T> {
        self.left.clone()
    }

    fn get_right(&self) -> Link<T> {
        self.right.clone()
    }

    // 获取及设置 key
    fn get_key(&self) -> T {
        self.key.clone()
    }

    fn set_key(&mut self, key: T) {
        self.key = key;
    }

    // 求最大最小 key
    fn min(&self) -> Option<&T> {
        match self.left {
            None => Some(&self.key),
            Some(ref node) => node.min(),
        }
    }

    fn max(&self) -> Option<&T> {
        match self.right {
            None => Some(&self.key),
            Some(ref node) => node.max(),
        }
    }

    // 查询 key 是否存在于树中
    fn contains(&self, key: &T) -> bool {
        match &self.key.cmp(key) {
            Equal => true,
            Greater => {
                match &self.left {
                    Some(left) => left.contains(key),
                    None => false,
                }
            },
            Less => {
                match &self.right {
                    Some(right) => right.contains(key),
                    None => false,
                }
            },
        }
    }

    // 新子节点作为根节点的左子节点
    fn insert_left_tree(&mut self, key: T) {
        if self.left.is_none() {
            let node = BinaryTree::new(key);
            self.left = Some(Box::new(node));
        } else {
            let mut node = BinaryTree::new(key);
            node.left = self.left.take();
            self.left = Some(Box::new(node));
        }
    }

    // 新子节点作为根节点的右子节点
    fn insert_right_tree(&mut self, key: T) {
        if self.right.is_none() {
            let node = BinaryTree::new(key);
            self.right = Some(Box::new(node));
        } else {
            let mut node = BinaryTree::new(key);
            node.right = self.right.take();
            self.right = Some(Box::new(node));
        }
    }

    // 前中后层序遍历: 内部实现 [递归方式]
    fn preorder(&self) {
        println!("key: {:?}", &self.key);
        match &self.left {
            Some(node) => node.preorder(),
            None => (),
        }
        match &self.right {
            Some(node) => node.preorder(),
            None => (),
        }
    }

    fn inorder(&self) {
        if self.left.is_some() {
            self.left.as_ref().unwrap().inorder();
        }
        println!("key: {:?}", &self.key);
        if self.right.is_some() {
            self.right.as_ref().unwrap().inorder();
        }
    }

    fn postorder(&self) {
        match &self.left {
            Some(node) => node.postorder(),
            None => (),
        }
        match &self.right {
            Some(node) => node.postorder(),
            None => (),
        }
        println!("key: {:?}", &self.key);
    }

    fn levelorder(&self) {
        let size = self.size();
        let mut q = Queue::new(size);

        // 根节点入队列
        let _r = q.enqueue(Box::new(self.clone()));
        while !q.is_empty() {
            // 出队首节点，输出值
            let front = q.dequeue().unwrap();
            println!("key: {:?}", front.get_key());

            // 找子节点并入队
            match front.get_left() {
                Some(left) => { let _r = q.enqueue(left); },
                None => {},
            }

            match front.get_right() {
                Some(right) => { let _r = q.enqueue(right); },
                None => {},
            }
        }
    }

    // 按照节点位置返回节点组成的字符串表达式: 内部实现
    fn iexp(&self) -> String {
        let mut exp = "".to_string();

        exp += "(";
        let exp_left = match &self.left {
            Some(left) => left.iexp(),
            None => "".to_string(),
        };
        exp += &exp_left;

        exp += &self.get_key().to_string();

        let exp_right = match &self.right {
            Some(right) => right.iexp(),
            None => "".to_string(),
        };
        exp += &exp_right;
        exp += ")";

        exp
    }
}

// 前中后层序遍历: 外部实现 [递归方式]，
// 考虑 bt 是不是该用 &Link<T>
fn preorder<T: Clone + Ord + ToString + Debug>(bt: Link<T>) {
    if !bt.is_none() {
        println!("key: {:?}", bt.as_ref().unwrap().get_key());
        preorder(bt.as_ref().unwrap().get_left());
        preorder(bt.as_ref().unwrap().get_right());
    }
}

fn inorder<T: Clone + Ord + ToString + Debug>(bt: Link<T>) {
    if !bt.is_none() {
        inorder(bt.as_ref().unwrap().get_left());
        println!("key: {:?}", bt.as_ref().unwrap().get_key());
        inorder(bt.as_ref().unwrap().get_right());
    }
}

fn postorder<T: Clone + Ord + ToString + Debug>(bt: Link<T>) {
    if !bt.is_none() {
        postorder(bt.as_ref().unwrap().get_left());
        postorder(bt.as_ref().unwrap().get_right());
        println!("key: {:?}", bt.as_ref().unwrap().get_key());
    }
}

fn levelorder<T: Clone + Ord + ToString + Debug>(bt: Link<T>) {
    if bt.is_none() { return; }

    let size = bt.as_ref().unwrap().size();
    let mut q = Queue::new(size);

    let _r = q.enqueue(bt.as_ref().unwrap().clone());
    while !q.is_empty() {
        let front = q.dequeue().unwrap();
        println!("key: {:?}", front.get_key());

        match front.get_left() {
            Some(left) => { let _r = q.enqueue(left); },
            None => {},
        }

        match front.get_right() {
            Some(right) => { let _r = q.enqueue(right); },
            None => {},
        }
    }
}

// 按照节点位置返回节点组成的字符串表达式: 外部实现
fn oexp<T: Clone + Ord + ToString + Debug + Display>(bt: Link<T>) -> String {
    let mut exp = "".to_string();
    if !bt.is_none() {
        exp = "(".to_string() + &oexp(bt.as_ref().unwrap().get_left());
        exp += &bt.as_ref().unwrap().get_key().to_string();
        exp += &(oexp(bt.as_ref().unwrap().get_right()) + ")");
    }

    exp
}

fn main() {
    basic();
    order();

    fn basic() {
        let mut bt = BinaryTree::new(10usize);

        let root = bt.get_key();
        println!("root key: {:?}", root);

        bt.set_key(11usize);
        let root = bt.get_key();
        println!("root key: {:?}", root);

        bt.insert_left_tree(2usize);
        bt.insert_right_tree(18usize);

        println!("left child: {:#?}", bt.get_left());
        println!("right child: {:#?}", bt.get_right());

        println!("min key: {:?}", bt.min().unwrap());
        println!("max key: {:?}", bt.max().unwrap());

        println!("tree nodes: {}", bt.size());
        println!("tree leaves: {}", bt.leaf_size());
        println!("tree internals: {}", bt.none_leaf_size());
        println!("tree depth: {}", bt.depth());
        println!("tree contains '2': {}", bt.contains(&2));
    }

    fn order() {
        let mut bt = BinaryTree::new(10usize);
        bt.insert_left_tree(2usize);
        bt.insert_right_tree(18usize);

        println!("internal pre-in-post-level order:");
        bt.preorder();
        bt.inorder();
        bt.postorder();
        bt.levelorder();

        let nk = Some(Box::new(bt.clone()));
        println!("outside pre-in-post-level order:");
        preorder(nk.clone());
        inorder(nk.clone());
        postorder(nk.clone());
        levelorder(nk.clone());

        println!("internal exp: {}", bt.iexp());
        println!("outside exp: {}", oexp(nk));
    }
}
