// lru.rs

use std::hash::Hash;
use std::collections::HashMap;

const CACHE_SIZE: usize = 100;

// LRU 上的元素项
struct Entry<K, V> {
    key: K,
    val: Option<V>,
    next: Option<usize>,
    prev: Option<usize>,
}

// LRU 缓存
struct LRUCache<K, V> {
    cap: usize,
    head: Option<usize>,
    tail: Option<usize>,
    map: HashMap<K, usize>,
    entries: Vec<Entry<K, V>>,
}

impl<K: Clone + Hash + Eq, V> LRUCache<K, V> {
    fn new() -> Self {
        Self::with_capacity(CACHE_SIZE)
    }

    fn with_capacity(cap: usize) -> Self {
        LRUCache {
            cap: cap,
            head: None,
            tail: None,
            map: HashMap::with_capacity(cap),
            entries: Vec::with_capacity(cap),
        }
    }

    fn is_empty(&self) -> bool {
        self.map.is_empty()
    }

    fn is_full(&self) -> bool {
        self.map.len() == self.cap
    }

    fn len(&self) -> usize {
        self.map.len()
    }

    fn insert(&mut self, key: K, val: V) -> Option<V> {
        if self.map.contains_key(&key) { // 存在 key 就更新
            self.access(&key);
            let entry = &mut self.entries[self.head.unwrap()];
            let old_val = entry.val.take();
            entry.val = Some(val);
            old_val
        } else { // 不存在就插入
            self.ensure_room();

            // 更新原始头指针
            let index = self.entries.len();
            self.head.map(|e| {
                self.entries[e].prev = Some(index);
            });

            // 新的头结点
            self.entries.push(Entry {
                key: key.clone(),
                val: Some(val),
                prev: None,
                next: self.head,
            });
            self.head = Some(index);
            self.tail = self.tail.or(self.head);
            self.map.insert(key, index);

            None
        }
    }

    fn remove(&mut self, key: &K) -> Option<V> {
        self.map.remove(&key).map(|index| {
                self.remove_from_list(index);
                self.entries[index].val.take().unwrap()
            })
    }

    fn get(&mut self, key: &K) -> Option<&V> {
        if self.contains(key) {
            self.access(key);
        }

        let entries = &self.entries;
        self.map.get(key).and_then(move |&i| {
            entries[i].val.as_ref()
        })
    }

    fn get_mut(&mut self, key: &K) -> Option<&mut V> {
        if self.contains(key) {
            self.access(key);
        }

        let entries = &mut self.entries;
        self.map.get(key).and_then(move |&i| {
            entries[i].val.as_mut()
        })
    }

    fn contains(&mut self, key: &K) -> bool {
        self.map.contains_key(key)
    }

    // 获取某个 key 的值，移除原来位置的值并在头部加入
    fn access(&mut self, key: &K) {
        let i = *self.map.get(key).unwrap();
        self.remove_from_list(i);
        self.head = Some(i);
    }

    fn remove_from_list(&mut self, i: usize) {
        let (prev, next) = {
            let entry = self.entries.get_mut(i).unwrap();
            (entry.prev, entry.next)
        };

        match (prev, next) {
            // 数据项在缓存中间
            (Some(j), Some(k)) => {
                let head = &mut self.entries[j];
                head.next = next;
                let next = &mut self.entries[k];
                next.prev = prev;
            },
            // 数据项在缓存末尾
            (Some(j), None) => {
                let head = &mut self.entries[j];
                head.next = None;
                self.tail = prev;
            },
            // 数据项在缓存头部
            _ => {
                if self.len() > 1 {
                    let head = &mut self.entries[0];
                    head.next = None;
                    let next = &mut self.entries[1];
                    next.prev = None;
                }
            },
        }
    }

    // 确保容量足够，满了就移除末尾的元素
    fn ensure_room(&mut self) {
        if self.cap == self.len() {
            self.remove_tail();
        }
    }

    fn remove_tail(&mut self) {
        if let Some(index) = self.tail {
            self.remove_from_list(index);
            let key = &self.entries[index].key;
            self.map.remove(key);
        }

        if self.tail.is_none() {
            self.head = None;
        }
    }
}

fn main() {
    let mut cache = LRUCache::with_capacity(2);
    cache.insert("foo", 1);
    cache.insert("bar", 2);
    cache.insert("baz", 3);
    cache.insert("tik", 4);
    cache.insert("tok", 5);

    assert!(!cache.contains(&"foo"));
    assert!(!cache.contains(&"bar"));
    assert!(cache.contains(&"baz"));
    assert!(cache.contains(&"tik"));

    cache.insert("qux", 6);
    assert!(cache.contains(&"qux"));
}
