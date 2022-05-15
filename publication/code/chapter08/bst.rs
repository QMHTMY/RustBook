// bst.rs

use std::cmp::{max, Ordering::*};
use std::fmt::Debug;

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

    fn is_empty(&self) -> bool {
        0 == self.len()
    }

    fn len(&self) -> usize {
        self.data.len()
    }
}

// 二叉查找树子节点链接
type Link<T,U> = Option<Box<BST<T,U>>>;

// 二叉查找树定义
#[derive(Debug,Clone)]
struct BST<T,U> {
    key: Option<T>,
    val: Option<U>,
    left: Link<T,U>,
    right: Link<T,U>,
}

impl<T,U> BST<T,U>
    where T: Copy + Ord + Debug,
          U: Copy + Debug
{
    fn new() -> Self {
        Self {
            key: None,
            val: None,
            left: None,
            right: None,
        }
    }

    fn is_empty(&self) -> bool {
        self.key.is_none()
    }

    fn size(&self) -> usize {
        self.calc_size(0)
    }

    // 递归计算节点个数
    fn calc_size(&self, mut size: usize) -> usize {
        if self.key.is_none() { return size; }

        // 当前节点数加入总节点数 i
        size += 1;

        // 计算左右子节点数
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

        // 左右子树的叶节点数之和就是总的叶节点数
        left_leaf + right_leaf
    }

    // 计算非叶节点数
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

        max(left_depth, right_depth)
    }

    // 节点插入
    fn insert(&mut self, key: T, val: U) {
        // 没数据直接插入
        if self.key.is_none() {
            self.key = Some(key);
            self.val = Some(val);
        } else {
            match &self.key {
                Some(k) => {
                    // 存在 key，更新 val
                    if key == *k {
                        self.val = Some(val);
                        return;
                    }

                    // 未找到相同 key，需要插入新节点
                    // 先找到需要插入的子树
                    let child = if key < *k {
                        &mut self.left
                    } else {
                        &mut self.right
                    };

                    // 根据节点递归下去，直到插入
                    match child {
                        Some(ref mut node) => {
                            node.insert(key, val);
                        },
                        None => {
                            let mut node = BST::new();
                            node.insert(key, val);
                            *child = Some(Box::new(node));
                        },
                    }
                },
                None => (),
            }
        }
    }

    // 节点查询
    fn contains(&self, key: &T) -> bool {
        match &self.key {
            None => false,
            Some(k) => {
                // 比较 key 值，并判断是否继续递归查找
                match k.cmp(key) {
                    Equal => true, // 找到数据
                    Greater => { // 在左子树搜索
                        match &self.left {
                            Some(node) => node.contains(key),
                            None => false,
                        }
                    },
                    Less => {
                        match &self.right { // 在右子树搜索
                            Some(node) => node.contains(key),
                            None => false,
                        }
                    },
                }
            },
        }
    }

    fn min(&self) -> (Option<&T>, Option<&U>) {
        // 最小值一定在最左侧
        match &self.left {
            Some(node) => node.min(),
            None => match &self.key {
                Some(key) => (Some(&key), self.val.as_ref()),
                None => (None, None),
            },
        }
    }

    fn max(&self) -> (Option<&T>, Option<&U>) {
        // 最大值一定在最右侧
        match &self.right {
            Some(node) => node.max(),
            None => match &self.key {
                Some(key) => (Some(&key), self.val.as_ref()),
                None => (None, None),
            },
        }
    }

    // 获取左右子节点
    fn get_left(&self) -> Link<T,U> {
        self.left.clone()
    }

    fn get_right(&self) -> Link<T,U> {
        self.right.clone()
    }

    // 获取值引用，和查找流程相似
    fn get(&self, key: &T) -> Option<&U> {
        match &self.key {
            None => None,
            Some(k) => {
                match k.cmp(key) {
                    Equal => self.val.as_ref(),
                    Greater => {
                        match &self.left {
                            None => None,
                            Some(node) => node.get(key),
                        }
                    },
                    Less => {
                        match &self.right {
                            None => None,
                            Some(node) => node.get(key),
                        }
                    },
                }
            },
        }
    }

    // 前中后层序遍历: 内部实现
    fn preorder(&self) {
        println!("key: {:?}, val: {:?}", self.key.unwrap(), self.val.unwrap());
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
        match &self.left {
            Some(node) => node.inorder(),
            None => (),
        }
        println!("key: {:?}, val: {:?}", self.key.unwrap(), self.val.unwrap());
        match &self.right {
            Some(node) => node.inorder(),
            None => (),
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
        println!("key: {:?}, val: {:?}", self.key.unwrap(), self.val.unwrap());
    }

    fn levelorder(&self) {
        let size = self.size();
        let mut q = Queue::new(size);

        let _r = q.enqueue(Box::new(self.clone()));
        while !q.is_empty() {
            let front = q.dequeue().unwrap();
            println!("key: {:?}, val: {:?}", front.key.unwrap(), front.val.unwrap());

            match front.get_left() {
                Some(left) => { let _r = q.enqueue(left); },
                None => (),
            }
            match front.get_right() {
                Some(right) => { let _r = q.enqueue(right); },
                None => (),
            }
        }
    }

}

// 前中后层序遍历: 外部实现
fn preorder<T, U>(bst: Link<T,U>)
where T: Copy + Ord + Debug,
      U: Copy + Debug
{
    if !bst.is_none() {
        println!("key: {:?}, val: {:?}", bst.as_ref().unwrap().key.unwrap(), bst.as_ref().unwrap().val.unwrap());
        preorder(bst.as_ref().unwrap().get_left());
        preorder(bst.as_ref().unwrap().get_right());
    }
}

fn inorder<T, U>(bst: Link<T,U>)
where T: Copy + Ord + Debug,
      U: Copy + Debug
{
    if !bst.is_none() {
        inorder(bst.as_ref().unwrap().get_left());
        println!("key: {:?}, val: {:?}", bst.as_ref().unwrap().key.unwrap(), bst.as_ref().unwrap().val.unwrap());
        inorder(bst.as_ref().unwrap().get_right());
    }
}

fn postorder<T, U>(bst: Link<T,U>)
where T: Copy + Ord + Debug,
      U: Copy + Debug
{
    if !bst.is_none() {
        postorder(bst.as_ref().unwrap().get_left());
        postorder(bst.as_ref().unwrap().get_right());
        println!("key: {:?}, val: {:?}", bst.as_ref().unwrap().key.unwrap(), bst.as_ref().unwrap().val.unwrap());
    }
}

fn levelorder<T, U>(bst: Link<T,U>)
where T: Copy + Ord + Debug,
      U: Copy + Debug
{
    if bst.is_none() { return; }

    let size = bst.as_ref().unwrap().size();
    let mut q = Queue::new(size);

    let _r = q.enqueue(bst.as_ref().unwrap().clone());
    while !q.is_empty() {
        let front = q.dequeue().unwrap();
        println!("key: {:?}, val: {:?}", front.key.unwrap(), front.val.unwrap());

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

fn main() {
    basic();
    order();

    fn basic() {
        let mut bst = BST::<i32, char>::new();
        bst.insert(8, 'e'); bst.insert(6,'c');
        bst.insert(7, 'd'); bst.insert(5,'b');
        bst.insert(10,'g'); bst.insert(9,'f');
        bst.insert(11,'h'); bst.insert(4,'a');

        println!("bst is empty: {}", bst.is_empty());
        println!("bst size: {}", bst.size());
        println!("bst leaves: {}", bst.leaf_size());
        println!("bst internals: {}", bst.none_leaf_size());
        println!("bst depth: {}", bst.depth());

        let min_kv = bst.min();
        let max_kv = bst.max();
        println!("min key: {:?}, min val: {:?}", min_kv.0, min_kv.1);
        println!("max key: {:?}, max val: {:?}", max_kv.0, max_kv.1);

        println!("bst contains 5: {}", bst.contains(&5));
        println!("key: 5, val: {:?}", bst.get(&5).unwrap());
    }

    fn order() {
        let mut bst = BST::<i32, char>::new();
        bst.insert(8, 'e'); bst.insert(6,'c');
        bst.insert(7, 'd'); bst.insert(5,'b');
        bst.insert(10,'g'); bst.insert(9,'f');
        bst.insert(11,'h'); bst.insert(4,'a');

        println!("internal inorder, preorder, postorder: ");
        bst.inorder();
        bst.preorder();
        bst.postorder();
        bst.levelorder();
        println!("outside inorder, preorder, postorder: ");
        let nk = Some(Box::new(bst.clone()));
        inorder(nk.clone());
        preorder(nk.clone());
        postorder(nk.clone());
        levelorder(nk.clone());
    }
}
