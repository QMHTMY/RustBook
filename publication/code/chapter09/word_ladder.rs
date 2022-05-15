// word_ladder.rs

use std::collections::HashMap;
use std::hash::Hash;

// 队列用于搜索
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
        if Self::len(&self) == self.cap {
            return Err("No space available!".to_string());
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

    fn len(&self) -> usize {
        self.data.len()
    }
}

// 颜色枚举，用于判断点是否被搜索过
#[derive(Clone, Debug, PartialEq)]
enum Color {
    White, // 白色，未被探索，
    Gray,  // 灰色，正被探索
    Black, // 黑色，探索完成
}

// 点定义
#[derive(Debug, Clone)]
struct Vertex<T> {
    color: Color,
    distance: u32, // 与起始点的最小距离，也即最小转换次数
    key: T,
    neighbors: Vec<(T, u32)>, // 邻点
}

impl<T: Clone + PartialEq> Vertex<T> {
    fn new(key: T) -> Self {
        Self {
            color: Color::White,
            distance: 0,
            key: key,
            neighbors: Vec::new(),
        }
    }

    fn add_neighbor(&mut self, nbr: T, wt: u32) {
        self.neighbors.push((nbr, wt));
    }

    // 获取邻点
    fn get_neighbors(&self) -> Vec<&T> {
        let mut neighbors = Vec::new();
        for (nbr, _wt) in self.neighbors.iter() {
            neighbors.push(nbr);
        }

        neighbors
    }
}

// 图定义
#[derive(Debug, Clone)]
struct Graph<T> {
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

    fn contains(&self, key: &T) -> bool {
        for (nbr, _vertex) in self.vertices.iter() {
            if nbr == key { return true; }
        }

        false
    }

    // 添加点
    fn add_vertex(&mut self, key: &T) -> Option<Vertex<T>> {
        let vertex = Vertex::new(key.clone());
        self.vertnums += 1;
        self.vertices.insert(key.clone(), vertex)
    }

    // 添加边
    fn add_edge(&mut self, from: &T, to: &T, wt: u32) {
        // 点不存在需要先添加
        if !self.contains(from) {
            let _fvert = self.add_vertex(from);
        }
        if !self.contains(to) {
            let _tvert = self.add_vertex(to);
        }

        self.edgenums += 1;
        self.vertices
            .get_mut(from)
            .unwrap()
            .add_neighbor(to.clone(), wt);
    }
}

// 根据单词及模式构建图
fn build_word_graph(words: Vec<&str>) -> Graph<String> {
    let mut hmap: HashMap<String, Vec<String>> = HashMap::new();

    // 构建单词-模式 hashMap
    for word in words {
        for i in 0..word.len() {
            let pattn = word[..i].to_string() + "_" + &word[i + 1..];
            if hmap.contains_key(&pattn) {
                hmap.get_mut(&pattn).unwrap().push(word.to_string());
            } else {
                hmap.insert(pattn, vec![word.to_string()]);
            }
        }
    }

    // 双向连接图，彼此距离为 1
    let mut word_graph = Graph::new();
    for word in hmap.keys() {
        for w1 in &hmap[word] {
            for w2 in &hmap[word] {
                if w1 != w2 {
                    word_graph.add_edge(w1, w2, 1);
                }
            }
        }
    }

    word_graph
}

// 字梯图-广度优先搜索
fn word_ladder(
    g: &mut Graph<String>,
    start: Vertex<String>,
    end: Vertex<String>,
    len: usize,
) -> u32 {
    // 判断起始点是否存在
    if !g.vertices.contains_key(&start.key) { return 0; }
    if !g.vertices.contains_key(&end.key) { return 0; }

    // 准备队列，加入起始点
    let mut vertex_queue = Queue::new(len);
    let _r = vertex_queue.enqueue(start);

    while vertex_queue.len() > 0 {
        // 节点出队
        let curr = vertex_queue.dequeue().unwrap();
        for nbr in curr.get_neighbors() {
            // 克隆，避免和图中数据起冲突
            // Graph 的 vertices 用 RefCell 包裹就不需要克隆
            let mut nbv = g.vertices.get(nbr).unwrap().clone();

            if end.key != nbv.key {
                // 只有白色的才可以入队列，其他颜色都处理过了
                if Color::White == nbv.color {
                    // 节点更新颜色和距离并加入队列
                    nbv.color = Color::Gray;
                    nbv.distance = curr.distance + 1;

                    // 图中的节点也需要更新颜色和距离
                    g.vertices.get_mut(nbr).unwrap().color = Color::Gray;
                    g.vertices.get_mut(nbr).unwrap().distance = curr.distance + 1;

                    // 白色节点加入队列
                    let _r = vertex_queue.enqueue(nbv);
                }
                // 其他颜色不需要处理，用两个颜色就够了
                // 所以代码里也没用 Black 枚举值
            } else {
                // curr 的邻点里有 end，所以再转换一次就够了
                return curr.distance + 1;
            }
        }
    }

    0
}

fn main() {
    let words = vec![
        "FOOL", "COOL", "POOL", "FOUL", "FOIL",
        "FAIL", "FALL", "POLL", "PALL", "POLE",
        "PALE", "SALE", "PAGE", "SAGE",
    ];
    let len = words.len();
    let mut g = build_word_graph(words);

    // 首节点加入队列表明正被探索，所以颜色变为灰色
    g.vertices.get_mut("FOOL").unwrap().color = Color::Gray;

    // 取出首尾点
    let start = g.vertices.get("FOOL").unwrap().clone();
    let end = g.vertices.get("SAGE").unwrap().clone();

    // 计算最小转换次数，也就是距离
    let distance = word_ladder(&mut g, start, end, len);
    println!("the shortest distance: {distance}");
}
