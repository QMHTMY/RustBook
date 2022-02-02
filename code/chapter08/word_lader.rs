use std::io::prelude::*;
use std::io::BufReader;
use std::fs::File;
use std::hash::Hash;
use std::collections::HashMap;

// 队列实现
#[derive(Debug)]
struct Queue<T> {
    cap: usize,
    data: Vec<T>,
}

impl<T> Queue<T> {
    fn new(size: usize) -> Self {
        Queue {
            cap: size,
            data: Vec::with_capacity(size),
        }
    }

    fn enqueue(&mut self, val: T) -> Result<(), String> {
        if Self::size(&self) == self.cap {
            return Err("No space available".to_string());
        }
        self.data.insert(0, val);

        Ok(())
    }

    fn dequeue(&mut self) -> Option<T> {
        if self.size() > 0 {
            self.data.pop()
        } else {
            None
        }
    }

    fn is_empty(&self) -> bool {
        0 == self.size()
    }

    fn size(&self) -> usize {
        self.data.len()
    }
}

// 点定义
#[derive(Debug, Clone)]
struct Vertex<T> {
    key: T,
    color: u8,
    dist: u32,
    pred: Option<T>,
    connects: Vec<(T, i32)>,
}

impl<T: Clone + PartialEq> Vertex<T> {
    fn new(key: T) -> Self {
        Self {
            key: key,
            color: 0, // 0 -> white, 1 -> gray, 2 -> black
            dist: 0,
            pred: None,
            connects: Vec::new()
        }
    }

    // 判断是否相邻
    fn adjacent_key(&self, key: &T) -> bool {
        for (nbr, _wt) in self.connects.iter() {
            if nbr == key {
                return true;
            }
        }
        false
    }

    fn add_neighbor(&mut self, nbr: T, wt: i32) {
        self.connects.push((nbr, wt));
    }

    // 获取邻点
    fn get_connects(&self) -> Vec<&T> {
        let mut connects = Vec::new();
        for (nbr, _wt) in self.connects.iter() {
            connects.push(nbr);
        }
        connects
    }

    // 获取到邻点的边权重
    fn get_nbr_weight(&self, key: &T) -> &i32 {
        for (nbr, wt) in self.connects.iter() {
            if nbr == key {
                return wt;
            }
        }
        &0
    }

    fn set_distance(&mut self, dist: u32) {
        self.dist = dist;
    }

    fn set_pred(&mut self, pred: Option<T>) {
        self.pred = pred;
    }

    fn set_color(&mut self, color: u8) {
        self.color = color;
    }
}

// 图定义
#[derive(Debug, Clone)]
struct Graph <T> {
    vertnums: u32,
    edgenums: u32,
    vertices: HashMap<T, Vertex<T>>,
}

impl<T: Hash + Eq + PartialEq + Clone> Graph<T> {
    fn new() -> Self {
        Self {
            vertnums: 0,
            edgenums: 0,
            vertices: HashMap::<T, Vertex<T>>::new(),
        }
    }

    fn is_empty(&self) -> bool {
        0 == self.vertnums
    }

    fn vertex_num(&self) -> u32 {
        self.vertnums
    }

    fn edge_num(&self) -> u32 {
        self.edgenums
    }

    fn contains(&self, key: &T) -> bool {
        for (nbr, _vertex) in self.vertices.iter() {
            if nbr == key {
                return true;
            }
        }
        false
    }

    fn add_vertex(&mut self, key: &T) -> Option<Vertex<T>> {
        let vertex = Vertex::new(key.clone());
        self.vertnums += 1;
        self.vertices.insert(key.clone(), vertex)
    }

    fn get_vertex(&self, key: &T) -> Option<&Vertex<T>> {
        if let Some(vertex) = self.vertices.get(key) {
            Some(&vertex)
        } else {
            None
        }
    }

    fn vertex_keys(&self) -> Vec<T> {
        let mut keys = Vec::new();
        for key in self.vertices.keys() {
            keys.push(key.clone());
        }
        keys
    }

    fn remove_vertex(&mut self, key: &T) -> Option<Vertex<T>> {
        let old_vertex = self.vertices.remove(key);
        self.vertnums -= 1;

        // 删除从当前点出发的边
        self.edgenums -= old_vertex.clone()
                                   .unwrap()
                                   .get_connects()
                                   .len() as u32;

        // 删除到当前点的边
        let mut i = 0;
        for vertex in self.vertex_keys() {
            if let Some(vt) = self.vertices.get_mut(&vertex) {
                if vt.adjacent_key(key) {
                    vt.connects.retain(|(k, _)| k != key);
                    i += 1;
                }
            }
        }
        self.edgenums -= i;

        old_vertex
    }

    fn add_edge(&mut self, from: &T, to: &T, wt: i32)  {
        // 点不存在需要先添加
        if !self.contains(from) {
            let _fvert = self.add_vertex(from);
        }
        if !self.contains(to) {
            let _tvert = self.add_vertex(to);
        }

        self.edgenums += 1;
        self.vertices.get_mut(from)
                     .unwrap()
                     .add_neighbor(to.clone(), wt);
    }

    // 判断点是否相邻
    fn is_adjacent(&self, from: &T, to: &T) -> bool {
        self.vertices.get(from).unwrap().adjacent_key(to)
    }
}

// 读单词
fn read_word(filename: &str) -> Vec<String> {
    let mut words = Vec::new();

    let fp = File::open(filename).unwrap();
    let bf = BufReader::new(fp);
    for line in bf.lines() {
        words.push(line.unwrap());
    }

    words
}

// 根据单词构建图
fn build_word_graph(words: &[String]) -> Graph<String> {
    let mut d: HashMap<String, Vec<String>> = HashMap::new();
    for word in words {
        for i in 0..word.len() {
            let bucket = (word[..i].to_string() + "_") + &word[i+1..];
            if d.contains_key(&bucket) {
                d.get_mut(&bucket).unwrap().push(word.to_string());
            } else {
                d.insert(bucket, vec![word.to_string()]);
            }
        }
    }

    let mut g = Graph::new();
    for bucket in d.keys() {
        for wd1 in &d[bucket] {
            for wd2 in &d[bucket] {
                if wd1 != wd2 {
                    g.add_edge(wd1,wd2,1);
                }
            }
        }
    }

    g
}

// 字梯图
fn word_lader(mut g: Graph<String>, mut start: Vertex<String>, end: Vertex<String>, len: usize) -> u32 {
    start.set_distance(0);
    start.set_pred(None);

    let mut vertex_queue = Queue::new(len);
    let _r = vertex_queue.enqueue(start);

    while vertex_queue.size() > 0 {
        let mut currv = vertex_queue.dequeue().unwrap();
        for nbr in currv.get_connects() {
            let nbv = g.vertices.get_mut(nbr).unwrap();
            if 0 == nbv.color {
                nbv.set_color(1);
                nbv.set_distance(currv.dist + 1);
                nbv.set_pred(Some(currv.key.clone()));
                let v = g.vertices.get(nbr).unwrap().clone();
                let _r = vertex_queue.enqueue(v);
            }
        }
        currv.set_color(2);
    }

    g.vertices.get(&end.key).unwrap().dist
}

fn main() {
    //let words = read_word("words.txt");
    let words = ["FOOL".to_string(), "COOL".to_string(), "POOL".to_string(),
                 "FOUL".to_string(), "FOIL".to_string(), "FAIL".to_string(),
                 "FALL".to_string(), "POLL".to_string(), "PALL".to_string(),
                 "POLE".to_string(), "PALE".to_string(), "SALE".to_string(),
                 "PAGE".to_string(), "SAGE".to_string()];
    let g = build_word_graph(&words);
    let start = Vertex::new(words[0].clone());
    let end = Vertex::new(words[words.len()-1].clone());
    let dist = word_lader(g, start, end, words.len());
    println!("the smallest dist: {dist}");
}
