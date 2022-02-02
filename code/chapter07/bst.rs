use std::cmp::Ordering;
use std::fmt::Debug;

// 二叉查找树子节点链接
type Link<T,U> = Option<Box<BST<T,U>>>;

// 二叉查找树定义
struct BST<T,U> {
    key: Option<T>,
    val: Option<U>,
    left: Link<T,U>,
    right: Link<T,U>,
}

impl<T,U> BST<T,U>
where T: Clone + Ord + Debug,
      U: Clone + Debug
{
    fn new() -> Self {
        BST {
            key: None,
            val: None,
            left: None,
            right: None,
        }
    }

    fn is_empty(&self) -> bool {
        self.key.is_none()
    }

    fn len(&self) -> usize {
        self.calc_len(0)
    }

    // 递归计算节点个数
    fn calc_len(&self, mut i: usize) -> usize {
        if self.key.is_none() { return i; }

        // 当前节点数加入总节点数 i
        i += 1;

        // 计算左右子节点数
        if !self.left.is_none() {
            i = self.left.as_ref().unwrap().calc_len(i);
        }
        if !self.right.is_none() {
            i = self.right.as_ref().unwrap().calc_len(i);
        }

        i
    }

    // 遍历，使用了 unwrap，使用 match 也可
    fn preorder(&self) {
        println!("key:{:?}, value:{:?}", &self.key, &self.val);
        if !self.left.is_none() { self.left.as_ref().unwrap().preorder(); }
        if !self.right.is_none() { self.right.as_ref().unwrap().preorder(); }
    }

    // 遍历，使用了 match，使用 unwarp 也可
    fn inorder(&self) {
        match &self.left {
            Some(node) => node.inorder(),
            None => (),
        }
        println!("key:{:?}, value:{:?}", &self.key, &self.val);
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
        println!("key:{:?}, value:{:?}", &self.key, &self.val);
    }

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

    fn search(&self, key: &T) -> bool {
        match &self.key {
            Some(k) => {
                // 比较 key 值，并判断是否继续递归查找
                match k.cmp(key) {
                    Ordering::Equal => { true }, // 找到数据
                    Ordering::Greater => { // 在左子树搜索
                        match &self.left {
                            Some(node) => node.search(key),
                            None => false,
                        }
                    },
                    Ordering::Less => {
                        match &self.right { // 在右子树搜索
                            Some(node) => node.search(key),
                            None => false,
                        }
                    },
                }
            },
            None => false,
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

    // 获取值，和查找流程相似
    fn get(&self, key: &T) -> Option<&U> {
        match &self.key {
            None => None,
            Some(k) => {
                match k.cmp(key) {
                    Ordering::Equal => self.val.as_ref(),
                    Ordering::Greater => {
                        match &self.left {
                            None => None,
                            Some(node) => node.get(key),
                        }
                    },
                    Ordering::Less => {
                        match &self.right {
                            None => None,
                            Some(node) => node.get(key),
                        }
                    },
                }
            },
        }
    }
}

fn main() {
    let mut bst = BST::<i32,char>::new();
    bst.insert(8, 'e'); bst.insert(6,'c');
    bst.insert(7, 'd'); bst.insert(5,'b');
    bst.insert(10,'g'); bst.insert(9,'f');
    bst.insert(11,'h'); bst.insert(4,'a');

    println!("empty: {:?}", bst.is_empty());
    println!("len: {:?}", bst.len());
    println!("min: {:?}", bst.min());
    println!("max: {:?}", bst.max());
    println!("key: 5, val: {:?}", bst.get(&5));
    println!("5 in bst: {:?}", bst.search(&5));

    println!("inorder: ");
    bst.inorder();
    println!("preorder: ");
    bst.preorder();
    println!("postorder: ");
    bst.postorder();
}
