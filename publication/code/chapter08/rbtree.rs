// rbtree.rs

use std::boxed::Box;
use std::cmp::{Ord, max, Ordering::*};
use std::fmt::Debug;
use std::iter::Iterator;
use std::ptr::null_mut;

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

// 红黑树颜色枚举
#[derive(Copy, Clone)]
enum Color {
    Red,
    Black,
}

// 红黑树节点定义
struct RBNode<K: Ord + Debug, V> {
    key: K,
    val: V,
    color: Color,
    parent: *mut RBNode<K, V>,
    left: *mut RBNode<K, V>,
    right: *mut RBNode<K, V>,
}

impl<K: Ord + Debug, V> RBNode<K, V> {
    fn new(key: K, val: V) -> Self {
        Self {
            key,
            val,
            color: Color::Red,
            parent: null_mut(),
            left: null_mut(),
            right: null_mut(),
        }
    }

    unsafe fn size(&self, mut size: usize) -> usize {
        size += 1;

        if !self.left.is_null() {
            size = (*self.left).size(size);
        }
        if !self.right.is_null() {
            size = (*self.right).size(size);
        }

        size
    }

    unsafe fn leaf_size(&self) -> usize {
        if self.left.is_null() && self.right.is_null() {
            return 1;
        }

        let mut left_leaf = 0;
        if !self.left.is_null() {
            left_leaf = (*self.left).leaf_size();
        }

        let mut right_leaf = 0;
        if !self.right.is_null() {
            right_leaf = (*self.right).leaf_size();
        }

        left_leaf + right_leaf
    }

    unsafe fn none_leaf_size(&self) -> usize {
        self.size(0) - self.leaf_size()
    }

    unsafe fn depth(&self) -> usize {
        let mut dl = 1;
        if !self.left.is_null() {
            dl += (*self.left).depth();
        }

        let mut dr = 1;
        if !self.right.is_null() {
            dr += (*self.right).depth();
        }

        max(dl, dr)
    }

    unsafe fn min(&self) -> Option<(&K, &V)> {
        if self.left.is_null() {
            Some((&self.key, &self.val))
        } else {
            (*self.left).min()
        }
    }

    unsafe fn max(&self) -> Option<(&K, &V)> {
        if self.right.is_null() {
            Some((&self.key, &self.val))
        } else {
            (*self.right).max()
        }
    }

    unsafe fn contains(&self, key: &K) -> bool {
        match self.key.cmp(key) {
            Equal => true,
            Greater => {
                if !self.left.is_null() {
                    (*self.left).contains(key)
                } else {
                    false
                }
            },
            Less => {
                if !self.right.is_null() {
                    (*self.right).contains(key)
                } else {
                    false
                }
            },
        }
    }

    unsafe fn preorder(&self) {
        println!("key: {:?}", &self.key);
        if !self.left.is_null() { (*self.left).preorder(); }
        if !self.right.is_null() { (*self.right).preorder(); }
    }

    unsafe fn inorder(&self) {
        if !self.left.is_null() { (*self.left).inorder(); }
        println!("key: {:?}", &self.key);
        if !self.right.is_null() { (*self.right).inorder(); }
    }

    unsafe fn postorder(&self) {
        if !self.left.is_null() { (*self.left).postorder(); }
        if !self.right.is_null() { (*self.right).postorder(); }
        println!("key: {:?}", &self.key);
    }

    unsafe fn levelorder(&self) {
        let size = self.size(0);
        let mut q = Queue::new(size);

        let _r = q.enqueue(self.clone());
        while !q.is_empty() {
            let front = q.dequeue().unwrap();
            println!("key: {:?}", front.key);

            if !front.left.is_null() {
                let _r = q.enqueue(&(*front.left));
            }
            if !front.right.is_null() {
                let _r = q.enqueue(&(*front.right));
            }
        }
    }
}

// 红黑树定义
struct RBTree<K: Ord + Debug, V> {
    root: *mut RBNode<K, V>,
}

// 为红黑树实现默认值
impl<K: Ord + Debug, V> Default for RBTree<K, V> {
    fn default() -> Self {
        Self::new()
    }
}

// 实现红黑树
impl<K: Ord + Debug, V> RBTree<K, V> {
    fn new() -> Self {
        Self { root: null_mut() }
    }

    fn is_empty(&self) -> bool {
        self.root.is_null()
    }

    // 计算树节点数
    fn size(&self) -> usize {
        unsafe {
            (*self.root).size(0)
        }
    }

    // 计算叶节点数
    fn leaf_size(&self) -> usize {
        unsafe {
            (*self.root).leaf_size()
        }
    }

    // 计算非叶节点数
    fn none_leaf_size(&self) -> usize {
        unsafe {
            (*self.root).none_leaf_size()
        }
    }

    // 树深度
    fn depth(&self) -> usize {
        unsafe {
            (*self.root).depth()
        }
    }

    // 最大最小值
    fn min(&self) -> Option<(&K, &V)> {
        unsafe {
            (*self.root).min()
        }
    }

    fn max(&self) -> Option<(&K, &V)> {
        unsafe {
            (*self.root).max()
        }
    }

    // 数据查询
    fn contains(&self, key: &K) -> bool {
        unsafe {
            (*self.root).contains(key)
        }
    }

    // 获取值引用及可变引用
    fn get(&self, key: &K) -> Option<&V> {
        unsafe {
            let mut node = self.root;
            while !node.is_null() {
                node = match (*node).key.cmp(key) {
                    Less => (*node).right,
                    Equal => return Some(&(*node).val),
                    Greater => (*node).left,
                }
            }
        }

        None
    }

    fn get_mut(&self, key: &K) -> Option<&mut V> {
        unsafe {
            let mut node = self.root;
            while !node.is_null() {
                node = match (*node).key.cmp(key) {
                    Less => (*node).right,
                    Equal => return Some(&mut (*node).val),
                    Greater => (*node).left,
                }
            }
        }

        None
    }

    // 数据插入
    fn insert(&mut self, key: K, val: V) {
        unsafe {
            let mut parent = null_mut();
            let mut node = self.root;

            // 找到待插入节点及其父节点位置
            while !node.is_null() {
                parent = node;
                node = match (*node).key.cmp(&key) {
                    Less => (*node).right,
                    Equal => {
                        (*node).val = val;
                        return;
                    }
                    Greater => (*node).left,
                }
            }

            // 数据插入
            node = Box::into_raw(Box::new(RBNode::new(key, val)));

            // 更新节点关系
            if !parent.is_null() {
                if (*node).key < (*parent).key {
                    (*parent).left = node;
                } else {
                    (*parent).right = node;
                }
            } else {
                self.root = node;
            }

            // 更新父节点关系
            (*node).parent = parent;

            // 旋转，更新节点颜色
            insert_fixup(self, node);
        }
    }

    fn delete(&mut self, key: &K) {
        unsafe {
            let mut parent = null_mut();
            let mut node = self.root;

            // 找到待删除节点
            while !node.is_null() {
                node = match (*node).key.cmp(key) {
                    Less => {
                        parent = node;
                        (*node).right
                    }
                    Equal => break,
                    Greater => {
                        parent = node;
                        (*node).left
                    }
                };
            }

            if node.is_null() {
                return;
            }

            let cl = (*node).left;
            let cr = (*node).right;
            let mut deleted_color;

            // 删除又分为多种情况
            if cl.is_null() {
                replace_node(self, parent, node, cr);

                /* 左右子节点均为空，n 随便着色
                 * (n could be either color here)
                 *
                 *     (n)             NULL
                 *    /   \    -->
                 *  NULL  NULL
                 */
                if cr.is_null() {
                    deleted_color = (*node).color;
                } else {
                    /*
                     * 左子节点空，右子节点不空
                     *     N             Cr
                     *    / \    -->    /  \
                     *  NULL cr       NULL NULL
                     */
                    (*cr).parent = parent;
                    (*cr).color = Color::Black;
                    deleted_color = Color::Red;
                }
            } else if cr.is_null() {
                /*
                 * 左子节点不空，右子节点空
                 *     N             Cl
                 *    / \    -->    /  \
                 *  cl  NULL      NULL NULL
                 */

                replace_node(self, parent, node, cl);
                (*cl).parent = parent;
                (*cl).color = Color::Black;
                deleted_color = Color::Red;
            } else {
                let mut victim = (*node).right;
                while !(*victim).left.is_null() {
                    victim = (*victim).left;
                }
                if victim == (*node).right {
                    /* Case 4 - victim is the right child of node
                     *
                     *     N         N         n
                     *    / \       / \       / \
                     *  (cl) cr   (cl) Cr    Cl  Cr
                     *
                     *     N         n
                     *    / \       / \
                     *  (cl) Cr    Cl  Cr
                     *         \         \
                     *         crr       crr
                     */

                    replace_node(self, parent, node, victim);
                    (*victim).parent = parent;
                    deleted_color = (*victim).color;
                    (*victim).color = (*node).color;
                    (*victim).left = cl;
                    (*cl).parent = victim;
                    if (*victim).right.is_null() {
                        parent = victim;
                    } else {
                        deleted_color = Color::Red;
                        (*(*victim).right).color = Color::Black;
                    }
                } else {
                    /*
                     * Case 5 - victim is not the right child of node
                     */

                    /* vp and vr denote parent and right child of victim, respectively. */
                    let vp = (*victim).parent;
                    let vr = (*victim).right;
                    (*vp).left = vr;
                    if vr.is_null() {
                        deleted_color = (*victim).color;
                    } else {
                        deleted_color = Color::Red;
                        (*vr).parent = vp;
                        (*vr).color = Color::Black;
                    }
                    replace_node(self, parent, node, victim);
                    (*victim).parent = parent;
                    (*victim).color = (*node).color;
                    (*victim).left = cl;
                    (*victim).right = cr;
                    (*cl).parent = victim;
                    (*cr).parent = victim;
                    parent = vp;
                }
            }

            // 释放资源
            Box::from_raw(node);
            if matches!(deleted_color, Color::Black) {
                delete_fixup(self, parent);
            }
        }
    }

    // 前中后层序遍历
    fn preorder(&self) {
        unsafe {
            (*self.root).preorder()
        }
    }

    fn inorder(&self) {
        unsafe {
            (*self.root).inorder()
        }
    }

    fn postorder(&self) {
        unsafe {
            (*self.root).postorder()
        }
    }

    fn levelorder(&self) {
        unsafe {
            (*self.root).levelorder()
        }
    }

    // 迭代函数，包括 mut 版本
    fn iter<'a>(&self) -> Iter<'a, K, V> {
        let mut iterator = Iter { stack: Vec::new() };
        let mut node = self.root;
        unsafe {
            while !node.is_null() {
                iterator.stack.push(&*node);
                node = (*node).left;
            }
        }
        iterator
    }

    fn iter_mut<'a>(&mut self) -> IterMut<'a, K, V> {
        let mut iterator = IterMut { stack: Vec::new() };
        let mut node = self.root;
        unsafe {
            while !node.is_null() {
                iterator.stack.push(&mut *node);
                node = (*node).left;
            }
        }
        iterator
    }
}

// 实现迭代功能
struct Iter<'a, K: Ord + Debug, V> {
    stack: Vec<&'a RBNode<K, V>>,
}
impl<'a, K: Ord + Debug, V> Iterator for Iter<'a, K, V> {
    type Item = &'a RBNode<K, V>;
    fn next(&mut self) -> Option<Self::Item> {
        match self.stack.pop() {
            Some(node) => {
                let mut next = node.right;
                unsafe {
                    while !next.is_null() {
                        self.stack.push(&*next);
                        next = (*next).left;
                    }
                }
                Some(node)
            }
            None => None,
        }
    }
}

struct IterMut<'a, K: Ord + Debug, V> {
    stack: Vec<&'a mut RBNode<K, V>>,
}
impl<'a, K: Ord + Debug, V> Iterator for IterMut<'a, K, V> {
    type Item = &'a mut RBNode<K, V>;
    fn next(&mut self) -> Option<Self::Item> {
        match self.stack.pop() {
            Some(node) => {
                let mut next = node.right;
                unsafe {
                    while !next.is_null() {
                        self.stack.push(&mut *next);
                        next = (*next).left;
                    }
                }
                Some(node)
            }
            None => None,
        }
    }
}

// 插入数据时旋转、着色
#[inline]
unsafe fn insert_fixup<K: Ord + Debug, V>(tree: &mut RBTree<K, V>, mut node: *mut RBNode<K, V>) {
    let mut parent: *mut RBNode<K, V> = (*node).parent;
    let mut gparent: *mut RBNode<K, V>;
    let mut tmp: *mut RBNode<K, V>;

    loop {
        // 节点为红色
        if parent.is_null() {
            (*node).color = Color::Black;
            break;
        }

        if matches!((*parent).color, Color::Black) {
            break;
        }

        gparent = (*parent).parent;
        tmp = (*gparent).right;
        if parent != tmp {
            if !tmp.is_null() && matches!((*tmp).color, Color::Red) {
                /* parent = (*gparent).left
                 * 颜色改变
                 *      G               g
                 *     / \             / \
                 *    p   u    -->    P   U
                 *   /               /
                 *  n               n
                 */

                (*parent).color = Color::Black;
                (*tmp).color = Color::Black;
                (*gparent).color = Color::Red;
                node = gparent;
                parent = (*node).parent;
                continue;
            }
            tmp = (*parent).right;
            if node == tmp {
                /* node = (*parent).right
                 * 左子树旋转
                 *    G               G
                 *   / \             / \
                 *  p   U    -->    n   U
                 *   \             /
                 *    n           p
                 */

                left_rotate(tree, parent);
                parent = node;
            }
            /*
             * 右子树旋转
             *      G               P
             *     / \             / \
             *    p   U    -->    n   g
             *   /                     \
             *  n                       U
             */

            (*parent).color = Color::Black;
            (*gparent).color = Color::Red;
            right_rotate(tree, gparent);
        } else {
            /* parent = (*gparent).right */
            tmp = (*gparent).left;
            if !tmp.is_null() && matches!((*tmp).color, Color::Red) {
                /*
                 * 颜色改变
                 *    G               g
                 *   / \             / \
                 *  u   p    -->    U   P
                 *       \               \
                 *        n               n
                 */

                (*parent).color = Color::Black;
                (*tmp).color = Color::Black;
                (*gparent).color = Color::Red;
                node = gparent;
                parent = (*node).parent;
                continue;
            }
            tmp = (*parent).left;
            if node == tmp {
                /*
                 * 右旋
                 *
                 *       G             G
                 *      / \           / \
                 *     U   p   -->   U   n
                 *        /               \
                 *       n                 p
                 */

                right_rotate(tree, parent);
                parent = node;
            }
            /*
             * 左旋
             *       G             P
             *      / \           / \
             *     U   p   -->   g   n
             *          \       /
             *           n     U
             */

            (*parent).color = Color::Black;
            (*gparent).color = Color::Red;
            left_rotate(tree, gparent);
        }
        break;
    }
}

// 删除数据时旋转、着色
#[inline]
unsafe fn delete_fixup<K: Ord + Debug, V>(tree: &mut RBTree<K, V>, mut parent: *mut RBNode<K, V>) {
    let mut node: *mut RBNode<K, V> = null_mut();
    let mut sibling: *mut RBNode<K, V>;
    /* sl and sr denote left and right child of sibling, respectively. */
    let mut sl: *mut RBNode<K, V>;
    let mut sr: *mut RBNode<K, V>;

    loop {
        // 黑色节点或空节点、非根节点
        sibling = (*parent).right;
        if node != sibling {
            /* node = (*parent).left */
            if matches!((*sibling).color, Color::Red) {
                /*
                 * 左旋转
                 *
                 *    P               S
                 *   / \             / \
                 *  N   s    -->    p   Sr
                 *     / \         / \
                 *    Sl  Sr      N  Sl
                 */

                left_rotate(tree, parent);
                (*parent).color = Color::Red;
                (*sibling).color = Color::Black;
                sibling = (*parent).right;
            }
            sl = (*sibling).left;
            sr = (*sibling).right;

            if !sl.is_null() && matches!((*sl).color, Color::Red) {
                /*
                 * 兄弟节点右旋之后父节点左旋
                 * (p and sr could be either color here)
                 *   (p)             (p)              (sl)
                 *   / \             / \              / \
                 *  N   S    -->    N   sl    -->    P   S
                 *     / \                \         /     \
                 *    sl (sr)              S       N      (sr)
                 *                          \
                 *                          (sr)
                 */

                (*sl).color = (*parent).color;
                (*parent).color = Color::Black;
                right_rotate(tree, sibling);
                left_rotate(tree, parent);
            } else if !sr.is_null() && matches!((*sr).color, Color::Red) {
                /*
                 * 父节点左旋
                 * (p could be either color here)
                 *   (p)               S
                 *   / \              / \
                 *  N   S    -->    (p) (sr)
                 *     / \          / \
                 *    Sl  sr       N   Sl
                 */

                (*sr).color = (*parent).color;
                left_rotate(tree, parent);
            } else {
                /*
                 * 改变颜色
                 * (p could be either color here)
                 *   (p)             (p)
                 *   / \             / \
                 *  N   S    -->    N   s
                 *     / \             / \
                 *    Sl  Sr          Sl  Sr
                 */

                (*sibling).color = Color::Red;
                if matches!((*parent).color, Color::Black) {
                    node = parent;
                    parent = (*node).parent;
                    continue;
                }
                (*parent).color = Color::Black;
            }
        } else {
            /* node = (*parent).right */
            sibling = (*parent).left;
            if matches!((*sibling).color, Color::Red) {
                // 父节点右旋
                right_rotate(tree, parent);
                (*parent).color = Color::Red;
                (*sibling).color = Color::Black;
                sibling = (*parent).right;
            }
            sl = (*sibling).left;
            sr = (*sibling).right;

            if !sr.is_null() && matches!((*sr).color, Color::Red) {
                // 兄弟节点左旋之后父节点右旋
                (*sr).color = (*parent).color;
                (*parent).color = Color::Black;
                left_rotate(tree, sibling);
                right_rotate(tree, parent);
            } else if !sl.is_null() && matches!((*sl).color, Color::Red) {
                // 父节点右旋

                (*sl).color = (*parent).color;
                right_rotate(tree, parent);
            } else {
                // 改变颜色
                (*sibling).color = Color::Red;
                if matches!((*parent).color, Color::Black) {
                    node = parent;
                    parent = (*node).parent;
                    continue;
                }
                (*parent).color = Color::Black;
            }
        }
        break;
    }
}

// 节点左旋
#[inline]
unsafe fn left_rotate<K: Ord + Debug, V>(tree: &mut RBTree<K, V>, x: *mut RBNode<K, V>) {
    /*
     * x 处左旋
     * (x could also be the left child of p)
     *
     *  p           p
     *   \           \
     *    x    -->    y
     *   / \         / \
     *      y       x
     *     / \     / \
     *    c           c
     */

    let p = (*x).parent;
    let y = (*x).right;
    let c = (*y).left;

    (*y).left = x;
    (*x).parent = y;
    (*x).right = c;
    if !c.is_null() {
        (*c).parent = x;
    }
    if p.is_null() {
        tree.root = y;
    } else if (*p).left == x {
        (*p).left = y;
    } else {
        (*p).right = y;
    }
    (*y).parent = p;
}

// 节点右旋
#[inline]
unsafe fn right_rotate<K: Ord + Debug, V>(tree: &mut RBTree<K, V>, x: *mut RBNode<K, V>) {
    /*
     * x 处右旋
     * (x could also be the left child of p)
     *
     *  p           p
     *   \           \
     *    x    -->    y
     *   / \         / \
     *  y               x
     * / \             / \
     *    c           c
     */

    let p = (*x).parent;
    let y = (*x).left;
    let c = (*y).right;

    (*y).right = x;
    (*x).parent = y;
    (*x).left = c;
    if !c.is_null() {
        (*c).parent = x;
    }
    if p.is_null() {
        tree.root = y;
    } else if (*p).left == x {
        (*p).left = y;
    } else {
        (*p).right = y;
    }
    (*y).parent = p;
}

// 替换节点值并更新节点关系
#[inline]
unsafe fn replace_node<K: Ord + Debug, V>(
    tree: &mut RBTree<K, V>,
    parent: *mut RBNode<K, V>,
    node: *mut RBNode<K, V>,
    new: *mut RBNode<K, V>)
{
    if parent.is_null() {
        tree.root = new;
    } else if (*parent).left == node {
        (*parent).left = new;
    } else {
        (*parent).right = new;
    }
}

fn main() {
    fn basic() {
        let mut rbt = RBTree::<usize, char>::new();
        println!("RBTree is empty: {}", rbt.is_empty());

        for (k, v) in String::from("I love you!").chars().enumerate() {
            rbt.insert(k, v);
        }

        println!("RBTree size: {}", rbt.size());
        println!("RBTree depth: {}", rbt.depth());
        println!("RBTree leaves: {}", rbt.leaf_size());
        println!("RBTree none leaf: {}", rbt.none_leaf_size());

        println!("RBTree min k-v: {:?}", rbt.min());
        println!("RBTree max k-v: {:?}", rbt.max());
        println!("RBTree contains 10: {}", rbt.contains(&10));
    }

    fn order() {
        let mut rbt = RBTree::<usize, u32>::new();
        let nums = [0,1,2,3,4,5,6,7,8,9];
        for (k, &v) in nums.iter().enumerate() {
            rbt.insert(k, v);
        }

        println!("internal preoder:");
        rbt.preorder();
        println!("internal inoder:");
        rbt.inorder();
        println!("internal postoder:");
        rbt.postorder();
        println!("internal levelorder:");
        rbt.levelorder();
    }

    fn get() {
        let mut rbt = RBTree::<usize, char>::new();
        for (k, v) in String::from("hello, world!").chars().enumerate() {
            rbt.insert(k, v);
        }
        for (k, v) in String::from("I love you!").chars().enumerate() {
            rbt.insert(k, v);
        }

        assert_eq!(*rbt.get(&1).unwrap(), ' ');
        assert_eq!(*rbt.get(&4).unwrap(), 'v');

        assert_eq!(*rbt.get_mut(&7).unwrap_or(&mut '*'), 'y');
        assert_eq!(*rbt.get_mut(&10).unwrap_or(&mut '*'), '!');
    }

    fn iter() {
        let nums = [0,1,2,3,4,5,6,7,8,9];

        let mut rbt = RBTree::<usize, u32>::new();
        for (k, &v) in nums.iter().enumerate() {
            rbt.insert(k, v);
        }

        let s: Vec<u32> = rbt.iter().map(|x| x.val).collect();
        assert_eq!(s, nums);

        for node in rbt.iter_mut() { node.val += 1; }
        let s: Vec<u32> = rbt.iter().map(|x| x.val).collect();
        assert_eq!(s, vec![1,2,3,4,5,6,7,8,9,10]);
    }

    fn insert() {
        let mut rbt = RBTree::<usize, char>::new();
        for (k, v) in String::from("I miss you!").chars().enumerate() {
            rbt.insert(k, v);
        }
        let s: String = rbt.iter().map(|x| x.val).collect();
        assert_eq!(s, "I miss you!");
    }

    fn delete() {
        let mut rbt = RBTree::<usize, char>::new();
        for (k, v) in String::from("I love you!").chars().enumerate() {
            rbt.insert(k, v);
        }
        rbt.delete(&1);
        rbt.delete(&3);
        rbt.delete(&5);
        rbt.delete(&7);
        let s: String = rbt.iter().map(|x| x.val).collect();
        assert_eq!(s, "Ilv ou!");
    }

    basic();
    order();
    get();
    iter();
    insert();
    delete();
}
