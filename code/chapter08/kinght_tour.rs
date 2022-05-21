// knight_tour.rs

use std::collections::HashMap;
use std::hash::Hash;
use std::fmt::Display;

// 棋盘宽度
const BDSIZE: u32 = 8;

// 颜色枚举
#[derive(Debug, Clone, PartialEq)]
enum Color {
    White, // 白色，未被探索
    Gray,  // 灰色，正被探索
}

// 棋盘上的点定义
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

// 旅游图定义
#[derive(Debug, Clone)]
struct Graph<T> {
    vertnums: u32,
    edgenums: u32,
    vertices: HashMap<T, Vertex<T>>,
}

impl<T: Eq + PartialEq + Clone + Hash> Graph<T> {
    fn new() -> Self {
        Self {
            vertnums: 0,
            edgenums: 0,
            vertices: HashMap::<T, Vertex<T>>::new(),
        }
    }

    fn add_vertex(&mut self, key: &T) -> Option<Vertex<T>> {
        let vertex = Vertex::new(key.clone());
        self.vertnums += 1;
        self.vertices.insert(key.clone(), vertex)
    }

    fn add_edge(&mut self, src: &T, des: &T) {
        if !self.vertices.contains_key(src) {
            let _fv = self.add_vertex(src);
        }
        if !self.vertices.contains_key(des) {
            let _tv = self.add_vertex(des);
        }

        self.edgenums += 1;
        self.vertices.get_mut(src)
                     .unwrap()
                     .add_neighbor(des.clone());
    }
}

fn legal_moves(x: u32, y: u32, bdsize: u32) -> Vec<(u32, u32)> {
    // 骑士移动是马在移动，而马移动是按照日字形移动: 马走日
    // 马走日横纵坐标值会相应增减，共八个方向，具体变化如下
    let move_offsets = [
                (-1,  2), ( 1,  2),
        (-2,  1),                 ( 2,  1),
        (-2, -1),                 ( 2, -1),
                (-1, -2), ( 1, -2),
    ];

    // 闭包函数，判断新坐标是否合法(不超出棋盘范围)
    let legal_pos = |a: i32, b: i32| { a >= 0 && a < b };

    let mut legal_positions = Vec::new();
    for (x_offset, y_offset) in move_offsets.iter() {
        let new_x = x as i32 + x_offset;
        let new_y = y as i32 + y_offset;

        // 判断坐标并加入可移动到的点集合
        if legal_pos(new_x, bdsize as i32) && legal_pos(new_y, bdsize as i32) {
            legal_positions.push((new_x as u32, new_y as u32));
        }
    }

    // 返回可移动到的点集合
    legal_positions
}

// 构建可移动路径图
fn build_knight_graph(bdsize: u32) -> Graph<u32> {
    // 闭包函数，计算点值 [0, 63]
    let calc_point = |row: u32, col: u32, size: u32| {
        (row % size) * size + col
    };

    // 各点间设置边
    let mut knight_graph = Graph::new();
    for row in 0..bdsize {
        for col in 0..bdsize {
            let dests = legal_moves(row, col, bdsize);
            for des in dests {
                let src_p = calc_point(row, col, bdsize);
                let des_p = calc_point(des.0, des.1, bdsize);
                knight_graph.add_edge(&src_p, &des_p);
            }
        }
    }

    knight_graph
}

// depth: 走过的路径长度, curr: 当前节点, path: 保存访问过的点
fn knight_tour<T>(
    kg: &mut Graph<T>,
    curr: Vertex<T>,
    path: &mut Vec<String>,
    depth: u32) -> bool
    where T: Eq + PartialEq + Clone + Hash + Display
{
    // 当前节点字符串值加入 path
    path.push(curr.key.to_string());

    let mut done = false;
    if depth < BDSIZE * BDSIZE - 1 {
        let mut i = 0;
        let nbrs = curr.get_neighbors();

        // 骑士在邻点间旅行
        while i < nbrs.len() && !done {
            // 克隆邻点，避免多个可变引用
            let nbr = kg.vertices.get(nbrs[i]).unwrap().clone();

            if Color::White == nbr.color {
                // 图对应点更新为灰色
                kg.vertices.get_mut(nbrs[i]).unwrap().color = Color::Gray;

                // 搜索下一个合适的点
                done = knight_tour(kg, nbr, path, depth + 1);
                if !done {
                    // 没找到，path 中去除当前点
                    // 并将图对应点颜色恢复为白色
                    let _rm = path.pop();
                    kg.vertices.get_mut(nbrs[i]).unwrap().color = Color::White;
                }
            }

            // 探索下一个邻点
            i += 1;
        }
    } else {
        done = true;
    }

    done
}

fn main() {
    // 构建骑士旅游图
    let mut kg: Graph<u32> = build_knight_graph(BDSIZE);

    // 选择起始点并更新图中点颜色
    let point = 0;
    kg.vertices.get_mut(&point).unwrap().color = Color::Gray;
    let start = kg.vertices.get(&point).unwrap().clone();

    // 开始骑士之旅，path 保存所有访问过的点
    let mut path = Vec::new();
    let successed = knight_tour(&mut kg, start, &mut path, 0);

    // 将结果格式化输出
    if successed {
        for row in 0..BDSIZE {
            let row_s = ((row % BDSIZE) * BDSIZE) as usize;
            let row_e = row_s + BDSIZE as usize;
            let row_str = path[row_s..row_e].join("\t");
            println!("{row_str}");
        }
    }
}
