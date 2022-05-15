// avl.rs

use std::cmp::{max, Ordering::*};
use std::fmt::Debug;
use std::mem::replace;

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

// Avl 树定义，使用的是枚举
#[derive(Clone, Debug, PartialEq)]
enum AvlTree<T> {
    Null,
    Tree(Box<AvlNode<T>>),
}

// Avl 树节点定义
#[derive(Clone, Debug, PartialEq)]
struct AvlNode<T> {
    key: T,
    left: AvlTree<T>,  // 左子树
    right: AvlTree<T>, // 右子树
    bfactor: i8,       // 平衡因子
}

use AvlTree::*;

impl<T> AvlTree<T> where T : Clone + Ord + Debug {
    // 新树为空
    fn new() -> Self {
        Null
    }

    fn is_empty(&self) -> bool {
        match self {
            Null => true,
            _ => false,
        }
    }

    // 计算树节点数: 左右子树节点 + 根节点数，递归计算
    fn size(&self) -> usize {
        match self {
            Null => 0,
            Tree(n) => 1 + n.left.size() + n.right.size(),
        }
    }

    // 计算叶节点数
    fn leaf_size(&self) -> usize {
        match self {
            Null => 0,
            Tree(node) => {
                if node.left == Null && node.right == Null {
                    return 1;
                }

                let left_leaf = match node.left {
                    Null => 0,
                    _ => node.left.leaf_size(),
                };
                let right_leaf = match node.right {
                    Null => 0,
                    _ => node.right.leaf_size(),
                };

                left_leaf + right_leaf
            },
        }
    }

    // 计算非叶节点数
    fn none_leaf_size(&self) -> usize {
        self.size() - self.leaf_size()
    }

    // 树深度是左右子树深度最大值 + 1，递归计算
    fn depth(&self) -> usize {
        match self {
            Null => 0,
            Tree(n) => max(n.left.depth(), n.right.depth()) + 1,
        }
    }

    // 获取树的最大最小节点值
    fn min(&self) -> Option<&T> {
        match self {
            Null => None,
            Tree(node) => {
                match node.left {
                    Null => Some(&node.key),
                    _ => node.left.min(),
                }
            },
        }
    }

    fn max(&self) -> Option<&T> {
        match self {
            Null => None,
            Tree(node) => {
                match node.right {
                    Null => Some(&node.key),
                    _ => node.right.max(),
                }
            },
        }
    }

    // 节点查找
    fn contains(&self, key: &T) -> bool {
        match self {
            Null => false,
            Tree(n) => {
                match n.key.cmp(key) {
                    Equal => true,
                    Greater => {
                        match &n.left {
                            Null => false,
                            _ => n.left.contains(key),
                        }
                    },
                    Less => {
                        match &n.right {
                            Null => false,
                            _ => n.right.contains(key),
                        }
                    },
                }
            },
        }
    }

    // 获取子节点
    fn node(&mut self) -> &mut AvlNode<T> {
        match self {
            Null => panic!("Error: Empty tree!"),
            Tree(node) => node,
        }
    }

    // 获取左右子树
    fn left_subtree(&mut self) -> &mut Self {
        match self {
            Null => panic!("Error: Empty tree!"),
            Tree(node) => &mut node.left,
        }
    }

    fn right_subtree(&mut self) -> &mut Self {
        match self {
            Null => panic!("Error: Empty tree!"),
            Tree(node) => &mut node.right,
        }
    }

    // 左右旋
    fn rotate_left(&mut self) {
        let mut n = replace(self, Null);
        let mut right = replace(n.right_subtree(), Null);
        let right_left = replace(right.left_subtree(), Null);
        *n.right_subtree() = right_left;
        *right.left_subtree() = n;
        *self = right;
    }

    fn rotate_right(&mut self) {
        let mut n = replace(self, Null);
        let mut left = replace(n.left_subtree(), Null);
        let left_right = replace(left.right_subtree(), Null);
        *n.left_subtree() = left_right;
        *left.right_subtree() = n;
        *self = left;
    }

    // 插入节点
    fn insert(&mut self, key: T) -> (bool, bool) {
        let ret = match self {
            // 没有节点，直接插入
            Null => {
                let node = AvlNode {
                    key: key,
                    left: Null,
                    right: Null,
                    bfactor: 0,
                };
                *self = Tree(Box::new(node));
                (true, true)
            },
            // 比较节点值，再判断该从哪边插入
            Tree(ref mut node) => match node.key.cmp(&key) {
                Equal => (false, false), // 相等，无需插入
                Less => {
                    // 比节点数据大，插入右边
                    let (inserted, deepened) = node.right.insert(key);

                    // inserted 表示是否插入
                    // deepened 表示是否加深
                    if deepened {
                        let ret = match node.bfactor {
                            -1 => (inserted, false),
                             0 => (inserted, true),
                             1 => (inserted, false),
                             _ => unreachable!(),
                        };
                        node.bfactor += 1;
                        ret
                    } else {
                        (inserted, deepened)
                    }
                },
                Greater => {
                    // 比节点数据小，插入左边
                    let (inserted, deepened) = node.left.insert(key);

                    if deepened {
                        let ret = match node.bfactor {
                            -1 => (inserted, false),
                             0 => (inserted, true),
                             1 => (inserted, false),
                             _ => unreachable!(),
                        };
                        node.bfactor -= 1;
                        ret
                    } else {
                        (inserted, deepened)
                    }
                },
            },
        };
        self.rebalance();

        ret
    }

    // 调整各节点的平衡因子
    fn rebalance(&mut self) {
        match self {
            // 没数据，不用调整
            Null => (),
            Tree(_) => match self.node().bfactor {
                // 右子树重
                -2 => {
                    let lbf = self.node().left.node().bfactor;
                    if lbf == -1 || lbf == 0 {
                        let (a, b) = if lbf == -1 {
                            (0, 0)
                        } else {
                            (-1,1)
                        };

                        // 旋转并更新平衡因子
                        self.rotate_right();
                        self.node().right.node().bfactor = a;
                        self.node().bfactor = b;
                    } else if lbf == 1 {
                        let (a, b) = match self.node()
                                               .left.node()
                                               .right.node()
                                               .bfactor {
                           -1 => (1, 0),
                            0 => (0, 0),
                            1 => (0,-1),
                            _ => unreachable!(),
                        };

                        // 先左旋再右旋，最后更新平衡因子
                        self.node().left.rotate_left();
                        self.rotate_right();
                        self.node().right.node().bfactor = a;
                        self.node().left.node().bfactor = b;
                        self.node().bfactor = 0;
                    } else {
                        unreachable!()
                    }
                },
                // 左子树重
                2 => {
                    let rbf = self.node().right.node().bfactor;
                    if rbf == 1 || rbf == 0 {
                        let (a,b) = if rbf == 1 {
                            (0, 0)
                        } else {
                            (1,-1)
                        };

                        // 旋转并更新平衡因子
                        self.rotate_left();
                        self.node().left.node().bfactor = a;
                        self.node().bfactor = b;
                    } else if rbf == -1 {
                        let (a, b) = match self.node()
                                               .right.node()
                                               .left.node()
                                               .bfactor {
                            1 => (-1, 0),
                            0 => ( 0, 0),
                           -1 => ( 0, 1),
                            _ => unreachable!(),
                        };

                        // 先右旋再左旋
                        self.node().right.rotate_right();
                        self.rotate_left();
                        self.node().left.node().bfactor = a;
                        self.node().right.node().bfactor = b;
                        self.node().bfactor = 0;
                    } else {
                         unreachable!()
                    }
                },
                // 平衡的，不用处理
                _ => (),
            },
        }
    }

    // 前中后层序遍历：内部实现
    fn preorder(&self) {
        match self {
            Null => (),
            Tree(node) => {
                println!("key: {:?}", node.key);
                node.left.preorder();
                node.right.preorder();
            },
        }
    }

    fn inorder(&self) {
        match self {
            Null => (),
            Tree(node) => {
                node.left.inorder();
                println!("key: {:?}", node.key);
                node.right.inorder();
            },
        }
    }

    fn postorder(&self) {
        match self {
            Null => (),
            Tree(node) => {
                node.left.postorder();
                node.right.postorder();
                println!("key: {:?}", node.key);
            },
        }
    }

    fn levelorder(&self) {
        let size = self.size();
        let mut q = Queue::new(size);

        let _r = q.enqueue(self);
        while !q.is_empty() {
            let front = q.dequeue().unwrap();
            match front {
                Null => (),
                Tree(node) => {
                    println!("key: {:?}", node.key);
                    let _r = q.enqueue(&node.left);
                    let _r = q.enqueue(&node.right);
                },
            }
        }
    }
}

// 前中后层序遍历: 外部实现
fn preorder<T: Clone + Ord + Debug>(avl: &AvlTree<T>) {
    match avl {
        Null => (),
        Tree(node) => {
            println!("key: {:?}", node.key);
            preorder(&node.left);
            preorder(&node.right);
        },
    }
}

fn inorder<T: Clone + Ord + Debug>(avl: &AvlTree<T>) {
    match avl {
        Null => (),
        Tree(node) => {
            inorder(&node.left);
            println!("key: {:?}", node.key);
            inorder(&node.right);
        },
    }
}

fn postorder<T: Clone + Ord + Debug>(avl: &AvlTree<T>) {
    match avl {
        Null => (),
        Tree(node) => {
            postorder(&node.left);
            postorder(&node.right);
            println!("key: {:?}", node.key);
        },
    }
}

fn levelorder<T: Clone + Ord + Debug>(avl: &AvlTree<T>) {
    let size = avl.size();
    let mut q = Queue::new(size);

    let _r = q.enqueue(avl);
    while !q.is_empty() {
        let front = q.dequeue().unwrap();
        match front {
            Null => (),
            Tree(node) => {
                println!("key: {:?}", node.key);
                let _r = q.enqueue(&node.left);
                let _r = q.enqueue(&node.right);
            },
        }
    }
}

fn main() {
    basic();
    order();

    fn basic() {
        let mut t = AvlTree::new();
        for i in 0..5 { let (_r1, _r2) = t.insert(i); }

        println!("empty:{},size:{}",t.is_empty(),t.size());
        println!("leaves:{},depth:{}",t.leaf_size(),t.depth());
        println!("internals:{}", t.none_leaf_size());
        println!("min-max key:{:?}-{:?}",t.min(),t.max());
        println!("contains 9:{}",t.contains(&9));
    }

    fn order() {
        let mut avl = AvlTree::new();
        for i in 0..5 { let (_r1, _r2) = avl.insert(i); }

        println!("internal pre-in-pos-level order");
        avl.preorder();
        avl.inorder();
        avl.postorder();
        avl.levelorder();
        println!("outside pre-in-pos-level order");
        preorder(&avl);
        inorder(&avl);
        postorder(&avl);
        levelorder(&avl);
    }
}
