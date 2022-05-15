// conshash.rs

use std::fmt::Debug;
use std::string::ToString;
use std::hash::{Hash, Hasher};
use std::collections::BTreeMap;
use std::collections::hash_map::DefaultHasher;

const DEFAULT_REPLICAS: usize = 100;

// 环上节点
#[derive(Clone, Debug)]
struct Node {
    host: &'static str,
    ip: &'static str,
    port: u16,
}

// 为 Node 添加 to_string 功能
impl ToString for Node {
    fn to_string(&self) -> String {
        self.ip.to_string() + &self.port.to_string()
    }
}

// 哈希计算函数
fn hash<T: Hash>(val: &T) -> u64 {
    let mut hasher = DefaultHasher::new();
    val.hash(&mut hasher);
    hasher.finish()
}

// 环
struct Ring<T: Clone + ToString + Debug> {
    replicas: usize,         // 分区数
    ring: BTreeMap<u64, T>,  // 保存数据的环
}

impl<T> Ring<T> where T: Clone + ToString + Debug {
    fn new() -> Self {
        Self::with_capacity(DEFAULT_REPLICAS)
    }

    fn with_capacity(replicas: usize) -> Self {
        Ring {
            replicas: replicas,
            ring: BTreeMap::new(),
        }
    }

    // 批量插入结点
    fn add_multi(&mut self, nodes: &[T]) {
        if !nodes.is_empty() {
            for node in nodes.iter() {
                self.add(node);
            }
        }
    }

    // 插入一个结点
    fn add(&mut self, node: &T) {
        for i in 0..self.replicas {
            let key = hash(&(node.to_string()+&i.to_string()));
            self.ring.insert(key, node.clone());
        }
    }

    // 批量删除结点
    fn remove_multi(&mut self, nodes: &[T]) {
        if !nodes.is_empty() {
            for node in nodes.iter() {
                self.remove(node);
            }
        }
    }

    // 删除一个结点
    fn remove(&mut self, node: &T) {
        assert!(!self.ring.is_empty());
        for i in 0..self.replicas {
            let key = hash(&(node.to_string() + &i.to_string()));
            self.ring.remove(&key);
        }
    }

    // 获取结点
    fn get(&self, key: u64) -> Option<&T> {
        if self.ring.is_empty() {
            return None;
        }
        let mut keys = self.ring.keys();
        keys.find(|&k| k >= &key)
            .and_then(|k| self.ring.get(k))
            .or(keys.nth(0).and_then(|x| self.ring.get(x)))
    }
}

fn main() {
    let replica = 3;
    let mut ring = Ring::with_capacity(replica);

    let node = Node{
        host: "localhost",
        ip: "127.0.0.1",
        port: 23,
    };
    ring.add(&node);

    for i in 0..replica {
        let key = hash(&(node.to_string() + &i.to_string()));
        let res = ring.get(key);
        assert_eq!(node.host, res.unwrap().host);
    }

    println!("{:?}", &node);
    ring.remove(&node);
}
