use std::fmt::{Debug, Display};

// 子节点链接
type Link<T> = Option<Box<BinaryTree<T>>>;

// 二叉树定义
// key 保存数据
// left 和 right 保存左右子节点链接
#[derive(Debug, Clone)]
struct BinaryTree<T> {
    key: T,
    left: Link<T>,
    right: Link<T>,
}

impl<T: Clone + Debug> BinaryTree<T> {
    fn new(key: T) -> Self {
        BinaryTree {
            key: key,
            left: None,
            right: None
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

    // 获取左右子节点及根节点，注意使用了 clone
    fn get_left(&self) -> Link<T> {
        self.left.clone()
    }

    fn get_right(&self) -> Link<T> {
        self.right.clone()
    }

    fn get_key(&self) -> T {
        self.key.clone()
    }

    fn set_key(&mut self, key: T) {
        self.key = key;
    }

    // 前中后序遍历: 内部实现
    fn preorder(&self) {
        println!("kes is {:?}", &self.key);
        if !self.left.is_none() { self.left.as_ref().unwrap().preorder(); }
        if !self.right.is_none() { self.right.as_ref().unwrap().preorder(); }
        // as_ref 获取节点引用，因为打印不能更改节点
    }

    fn inorder(&self) {
        if !self.left.is_none() { self.left.as_ref().unwrap().inorder(); }
        println!("kes is {:?}", &self.key);
        if !self.right.is_none() { self.right.as_ref().unwrap().inorder(); }
    }

    fn postorder(&self) {
        if !self.left.is_none() { self.left.as_ref().unwrap().postorder(); }
        if !self.right.is_none() { self.right.as_ref().unwrap().postorder(); }
        println!("kes is {:?}", &self.key);
    }
}

// 前中后序遍历: 外部实现
fn preorder<T: Clone + Debug>(bt: Link<T>) {
    if !bt.is_none() {
        println!("key is {:?}", bt.as_ref().unwrap().get_key());
        preorder(bt.as_ref().unwrap().get_left());
        preorder(bt.as_ref().unwrap().get_right());
    }
}

fn inorder<T: Clone + Debug>(bt: Link<T>) {
    if !bt.is_none() {
        inorder(bt.as_ref().unwrap().get_left());
        println!("key is {:?}", bt.as_ref().unwrap().get_key());
        inorder(bt.as_ref().unwrap().get_right());
    }
}

fn postorder<T: Clone + Debug>(bt: Link<T>) {
    if !bt.is_none() {
        postorder(bt.as_ref().unwrap().get_left());
        postorder(bt.as_ref().unwrap().get_right());
        println!("key is {:?}", bt.as_ref().unwrap().get_key());
    }
}

// 按照节点位置返回节点组成的字符串
fn get_exp<T: Clone + Debug + Display>(bt: Link<T>) -> String {
    let mut exp = "".to_string();
    if !bt.is_none() {
        exp = "(".to_string() + &get_exp(bt.as_ref().unwrap().get_left());
        exp += &bt.as_ref().unwrap().get_key().to_string();
        exp += &(get_exp(bt.as_ref().unwrap().get_right()) + ")");
    }

    exp
}

fn main() {
    let mut bt = BinaryTree::new('a');

    let root = bt.get_key();
    println!("root val is {:?}", root);

    bt.insert_left_tree('b');
    bt.insert_right_tree('c');

    let left = bt.get_left();
    println!("left child is {:#?}", left);
    let right = bt.get_left();
    println!("right child is {:#?}", right);

    bt.preorder();
    bt.inorder();
    bt.postorder();

    let nk = Some(Box::new(bt));

    preorder(nk.clone());
    inorder(nk.clone());
    postorder(nk.clone());

    let tree_str = get_exp(nk);
    println!("String expr is {tree_str}");
}
