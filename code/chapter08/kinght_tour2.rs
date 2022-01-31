use std::collections::HashMap;
use std::hash::Hash;

const BDSIZE: u32 =  8;

#[derive(Debug, Clone)]
struct Vertex<T> {
    key: T,
    color: u8, // 0 -> white  1 -> gray  2 -> black
    connects: Vec<(T, i32)>,
}

impl<T: PartialEq + Clone> Vertex<T> {
    fn new(key: T) -> Self {
        Self { key: key, color: 0, connects: Vec::new() }
    }

    fn get_key(&self) -> T {
        self.key.clone()
    }

    fn get_color(&self) -> u8 {
        self.color
    }

    fn set_color(&mut self, color: u8) {
        self.color = color;
    }

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

    fn get_connects(&self) -> Vec<&T> {
        let mut connects = Vec::new();
        for (nbr, _wt) in self.connects.iter() {
            connects.push(nbr);
        }
        connects
    }

    fn get_nbr_weight(&self, key: &T) -> &i32 {
        for (nbr, wt) in self.connects.iter() {
            if nbr == key {
                return wt;
            }
        }
        &0
    }
}

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
        for (nbr, _vt) in self.vertices.iter() {
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
        for vertex in self.vertex_keys() {
            if let Some(vt) = self.vertices.get_mut(&vertex) {
                if vt.adjacent_key(key) {
                    vt.connects.retain(|(k, _)| k != key);
                    self.edgenums -= 1;
                }
            }
        }

        old_vertex
    }

    fn add_edge(&mut self, from: &T, to: &T, wt: i32) {
        if !self.contains(from) {
            let _fv = self.add_vertex(from);
        }
        if !self.contains(to) {
            let _tv = self.add_vertex(to);
        }

        self.edgenums += 1;
        self.vertices.get_mut(from)
                     .unwrap()
                     .add_neighbor(to.clone(), wt);
    }

    fn adjacent(&self, from: &T, to: &T) -> bool {
        self.vertices.get(from).unwrap().adjacent_key(to)
    }
}

fn legal_moves(x: u32, y: u32, bdsize: u32) -> Vec<(u32, u32)> {
    let legal_pos = |a:i32, b:i32| -> bool { a >= 0 && a < b };

    // 马移动是日字型坐标变化，共 8 个方向，横纵坐标会相应增减，其值如下
    let move_offsets  = [(-1,-2),(-1,2),(-2,-1),(-2,1),(1,-2),(1,2),(2,-1),(2,1)];

    let mut moves = vec![];
    for offset in move_offsets.iter() {
        let newx = x as i32 + offset.0;
        let newy = y as i32 + offset.1;
        if legal_pos(newx, bdsize as i32) && legal_pos(newy, bdsize as i32) {
            moves.push((newx as u32, newy as u32));
        }
    }

    moves
}

fn knight_graph(bdsize: u32) -> Graph<u32> {
    let mut g = Graph::new();
    for row in 0..bdsize {
        for col in 0..bdsize {
            let moves = legal_moves(row, col, bdsize);
            for mv in moves {
                let from = row * bdsize + col;
                let to = mv.0 * bdsize + mv.1;
                g.add_edge(&from, &to, 1);
            }
        }
    }

    g
}

// depth: 走过的路径长度, u: 起始点, path: 保存访问过的点
fn knight_tour<T>(depth: u32, u: &mut Vertex<T>, path: &mut Vec<T>) -> bool {
    u.set_color(1);
    path.push(u.get_key());

    let mut done = false;
    if depth < BDSIZE * BDSIZE {
        let nbrs = u.get_connects();
        let mut i = 0;
        while i < nbrs.len() && !done {
            if 0 == nbrs[i].get_color() {
                done = knight_tour(depth+1, nbrs[i], path);
            }
            i += 1;
        }

        if !done {
            let _rm = path.pop();
            u.set_color(0);
        }
    } else {
        done = true;
    }

    done
}

fn main() {
    let mut kg: Graph<u32> = knight_graph(BDSIZE);

    let mut start = kg.vertices.get_mut(&0).unwrap();
    let mut path = Vec::new();
    let res = knight_tour(0, &mut start, &mut path);

    if res {
        println!("knight tour successed: {}", res);
        for vertex in path {
            println!("{:#?}", vertex);
        }
    }
}
