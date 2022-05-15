// hashmap.rs

// 用 slot 保存位置，data 保存数据，cap 控制容量
#[derive(Debug, Clone, PartialEq)]
struct HashMap <T> {
    cap: usize,
    slot: Vec<usize>,
    data: Vec<T>,
}

impl<T: Clone + PartialEq + Default> HashMap<T> {
    fn new(cap: usize) -> Self {
        // 初始化 slot 和 data
        let mut slot = Vec::with_capacity(cap);
        let mut data = Vec::with_capacity(cap);
        for _i in 0..cap{
            slot.push(0);
            data.push(Default::default());
        }

        HashMap { cap, slot, data }
    }

    fn len(&self) -> usize {
        let mut len = 0;
        for &d in self.slot.iter() {
            // 槽中数据不为 0，表示有数据，len 加 1
            if 0 != d  {
                len += 1;
            }
        }

        len
    }

    fn is_empty(&self) -> bool {
        let mut empty = true;
        for &d in self.slot.iter() {
            if 0 != d  {
                empty = false;
                break;
            }
        }

        empty
    }

    fn clear(&mut self) {
        let mut slot = Vec::with_capacity(self.cap);
        let mut data = Vec::with_capacity(self.cap);
        for _i in 0..self.cap{
            slot.push(0);
            data.push(Default::default());
        }

        self.slot = slot;
        self.data = data;
    }

    fn hash(&self, key: usize) -> usize {
        key % self.cap
    }

    fn rehash(&self, pos: usize) -> usize {
        (pos + 1) % self.cap
    }

    fn insert(&mut self, key: usize, value: T) {
        if 0 == key { panic!("Error: key must > 0"); }

        let pos = self.hash(key);
        if 0 == self.slot[pos] {
            // 槽无数据直接插入
            self.slot[pos] = key;
            self.data[pos] = value;
        } else {
            // 插入槽有数据再找下一个可行的位置
            let mut next = self.rehash(pos);
            while 0 != self.slot[next]
                && key != self.slot[next] {
                next = self.rehash(next);

                // 槽满了就退出
                if next == pos {
                    println!("Error: slot is full!");
                    return;
                }
            }

            // 在找到的槽插入数据
            if 0 == self.slot[next] {
                self.slot[next] = key;
                self.data[next] = value;
            } else {
                self.data[next] = value;
            }
        }
    }

    fn remove(&mut self, key: usize) -> Option<T> {
        if 0 == key { panic!("Error: key must > 0"); }

        let pos = self.hash(key);
        if 0 == self.slot[pos] {
            // 槽中无数据，返回 None
            None
        } else if key == self.slot[pos] {
            // 找到相同 key，更新 slot 和 data
            self.slot[pos] = 0;
            let data = Some(self.data[pos].clone());
            self.data[pos] = Default::default();
            data
        } else {
            let mut data: Option<T>  = None;
            let mut stop = false;
            let mut found = false;
            let mut curr = pos;

            while 0 != self.slot[curr] && !found && !stop {
                if key == self.slot[curr] {
                    // 找到了值，删除数据
                    found = true;
                    self.slot[curr] = 0;
                    data = Some(self.data[curr].clone());
                    self.data[curr] = Default::default();
                } else {
                    // 再哈希回到了最初位置，说明找了一圈还没有
                    curr = self.rehash(curr);
                    if curr == pos {
                        stop = true;
                    }
                }
            }

            data
        }
    }

    fn get_pos(&self, key: usize) -> usize {
        if 0 == key { panic!("Error: key must > 0"); }

        // 计算数据位置
        let pos = self.hash(key);
        let mut stop = false;
        let mut found = false;
        let mut curr = pos;

        // 循环查找数据
        while 0 != self.slot[curr] && !found && !stop {
            if key == self.slot[curr] {
                found = true;
            } else {
                // 再哈希回到了最初位置，说明找了一圈还没有
                curr = self.rehash(curr);
                if curr == pos {
                    stop = true;
                }
            }
        }

        curr
    }

    // 获取 val 的引用及可变引用
    fn get(&self, key: usize) -> Option<&T> {
        let curr = self.get_pos(key);
        self.data.get(curr)
    }

    fn get_mut(&mut self, key: usize) -> Option<&mut T> {
        let curr = self.get_pos(key);
        self.data.get_mut(curr)
    }

    fn contains(&self, key: usize) -> bool {
        if 0 == key { panic!("Error: key must > 0"); }
        self.slot.contains(&key)
    }

    // 为 hashmap 实现的迭代及可变迭代功能
    fn iter(&self) -> Iter<T> {
        let mut iterator = Iter { stack: Vec::new() };
        for item in self.data.iter() {
            iterator.stack.push(item);
        }

        iterator
    }

    fn iter_mut(&mut self) -> IterMut<T> {
        let mut iterator = IterMut { stack: Vec::new() };
        for item in self.data.iter_mut() {
            iterator.stack.push(item);
        }

        iterator
    }
}

// 实现迭代功能
struct Iter<'a, T: 'a> { stack: Vec<&'a T>, }
impl<'a, T> Iterator for Iter<'a, T> {
    type Item = &'a T;
    fn next(&mut self) -> Option<Self::Item> {
        self.stack.pop()
    }
}

struct IterMut<'a, T: 'a> { stack: Vec<&'a mut T>, }
impl<'a, T> Iterator for IterMut<'a, T> {
    type Item = &'a mut T;
    fn next(&mut self) -> Option<Self::Item> {
        self.stack.pop()
    }
}

fn main() {
    basic();
    iter();

    fn basic() {
        let mut hmap = HashMap::new(11);
        hmap.insert(2,"dog");
        hmap.insert(3,"tiger");
        hmap.insert(10,"cat");

        println!("empty: {}, size: {:?}", hmap.is_empty(), hmap.len());
        println!("contains key 2: {}", hmap.contains(2));

        println!("key 3: {:?}", hmap.get(3));
        let val_ptr = hmap.get_mut(3).unwrap();
        *val_ptr = "fish";
        println!("key 3: {:?}", hmap.get(3));

        println!("remove key 3: {:?}", hmap.remove(3));
        println!("remove key 3: {:?}", hmap.remove(3));

        hmap.clear();
        println!("empty: {}, size: {:?}", hmap.is_empty(), hmap.len());
    }

    fn iter() {
        let mut hmap = HashMap::new(11);
        hmap.insert(2,"dog");
        hmap.insert(3,"tiger");
        hmap.insert(10,"cat");

        for item in hmap.iter() {
            println!("val: {item}");
        }

        for item in hmap.iter_mut() {
            *item = "fish";
        }

        for item in hmap.iter() {
            println!("val: {item}");
        }
    }
}
