// find_province_num_dfs.rs

use std::collections::HashMap;
use std::hash::Hash;

// 颜色枚举
#[derive(Debug, Clone, PartialEq)]
enum Color {
    White, // 白色，未被探索
    Gray,  // 灰色，正被探索
}

// 城市点定义
#[derive(Debug, Clone)]
struct Vertex<T> {
    key: T,
    color: Color,
    neighbors: Vec<T>,
}
impl<T: PartialEq + Clone> Vertex<T> {
    fn new(key: T) -> Self {
        Self {
            key: key,
            color: Color::White,
            neighbors: Vec::new(),
        }
    }

    fn add_neighbor(&mut self, nbr: T) {
        self.neighbors.push(nbr);
    }

    fn get_neighbors(&self) -> Vec<&T> {
        let mut neighbors = Vec::new();
        for nbr in self.neighbors.iter() {
            neighbors.push(nbr);
        }

        neighbors
    }
}

// 省份图定义
#[derive(Debug, Clone)]
struct Graph<T> {
    vertnums: u32,
    edgenums: u32,
    vertices: HashMap<T, Vertex<T>>,
    edges: HashMap<T, Vec<T>>,
}
impl<T: Eq + PartialEq + Clone + Hash> Graph<T> {
    fn new() -> Self {
        Self {
            vertnums: 0,
            edgenums: 0,
            vertices: HashMap::<T, Vertex<T>>::new(),
            edges: HashMap::<T, Vec<T>>::new(),
        }
    }

    fn add_vertex(&mut self, key: &T) -> Option<Vertex<T>> {
        let vertex = Vertex::new(key.clone());
        self.vertnums += 1;
        self.vertices.insert(key.clone(), vertex)
    }

    fn add_edge(&mut self, src: &T, des: &T) {
        if !self.vertices.contains_key(src) { let _fv = self.add_vertex(src); }
        if !self.vertices.contains_key(des) { let _tv = self.add_vertex(des); }

        // 添加点
        self.edgenums += 1;
        self.vertices.get_mut(src)
                     .unwrap()
                     .add_neighbor(des.clone());

        // 添加边
        if !self.edges.contains_key(src) {
            let _eg = self.edges.insert(src.clone(), Vec::new());
        }
        self.edges.get_mut(src).unwrap().push(des.clone());
    }
}

// 构建城市连接关系图
fn build_city_graph<T>(city_connected: Vec<Vec<T>>) -> Graph<T>
    where T: Eq + PartialEq + Clone + Hash
{
    // 各点间设置边
    let mut city_graph = Graph::new();
    for v in city_connected.iter() {
        let src = v.first().unwrap();
        let des = v.last().unwrap();
        city_graph.add_edge(src, des);
    }

    city_graph
}

// 搜索当前节点 city 的邻点
fn search_city<T>(cg: &mut Graph<T>, city: Vertex<T>)
    where T: Eq + PartialEq + Clone + Hash
{
    // 逐个搜索当前节点的邻点
    for ct in city.get_neighbors() {
        let city = cg.vertices.get(ct).unwrap().clone();
        if Color::White == city.color {
            // 改变当前节点颜色
            cg.vertices.get_mut(ct).unwrap().color = Color::Gray;
            // 继续搜索当前节点的邻点
            search_city(cg, city);
        }
    }
}

fn find_province_num_dfs<T>(city_connected: Vec<Vec<T>>) -> u32
    where T: Eq + PartialEq + Clone + Hash
{
    let mut cg = build_city_graph(city_connected);
    let mut cities = Vec::new();

    // 获取各个主节点城市 key
    for key in cg.edges.keys() {
        cities.push(key.clone());
    }

    let mut province_num = 0;
    // 逐个处理省强连通分量
    for ct in &cities {
        let city = cg.vertices.get(ct).unwrap().clone();
        if Color::White == city.color {
            // 改变当前节点颜色
            cg.vertices.get_mut(ct).unwrap().color = Color::Gray;
            // 搜索当前节点的邻点
            search_city(&mut cg, city);
            // 处理完一个省强连通分量
            province_num += 1;
        }
    }

    province_num
}

fn main() {
    // 构建城市依赖关系
    let mut connected = Vec::<Vec<&str>>::new();
    connected.push(vec!["成都", "自贡"]);
    connected.push(vec!["成都", "绵阳"]);
    connected.push(vec!["成都", "德阳"]);
    connected.push(vec!["成都", "泸州"]);
    connected.push(vec!["成都", "内江"]);
    connected.push(vec!["成都", "乐山"]);
    connected.push(vec!["成都", "宜宾"]);
    connected.push(vec!["自贡", "成都"]);

    connected.push(vec!["广州", "深圳"]);
    connected.push(vec!["广州", "东莞"]);
    connected.push(vec!["广州", "珠海"]);
    connected.push(vec!["广州", "中山"]);
    connected.push(vec!["广州", "汕头"]);
    connected.push(vec!["广州", "佛山"]);
    connected.push(vec!["广州", "湛江"]);
    connected.push(vec!["深圳", "广州"]);

    connected.push(vec!["武汉", "荆州"]);
    connected.push(vec!["武汉", "宜昌"]);
    connected.push(vec!["武汉", "襄阳"]);
    connected.push(vec!["武汉", "荆门"]);
    connected.push(vec!["武汉", "孝感"]);
    connected.push(vec!["武汉", "黄冈"]);
    connected.push(vec!["荆州", "武汉"]);

    // 找到所有的省强连通分量，此处只有三个省：四川、广东、湖北
    let province_num = find_province_num_dfs(connected);
    println!("province nummber: {province_num}");
}
