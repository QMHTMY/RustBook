// cooking_topological_sort.rs

use std::collections::HashMap;
use std::hash::Hash;
use std::fmt::Display;

// 颜色枚举
#[derive(Debug, Clone, PartialEq)]
enum Color {
    White, // 白色，未被探索
    Gray,  // 灰色，正被探索
    Black, // 黑色，已被探索
}

// 课程点定义
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
}

// 课程关系图定义
#[derive(Debug, Clone)]
struct Graph<T> {
    vertnums: u32,
    edgenums: u32,
    vertices: HashMap<T, Vertex<T>>, // 所有点
    edges: HashMap<T, Vec<T>>, // 所有边
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
        if !self.vertices.contains_key(src) { let _sv = self.add_vertex(src); }
        if !self.vertices.contains_key(des) { let _dv = self.add_vertex(des); }

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

// 构建课程关系图
fn build_course_graph<T>(pre_requisites: Vec<Vec<T>>) -> Graph<T>
    where T: Eq + PartialEq + Clone + Hash
{
    // 为依赖的课程创建边关系
    let mut course_graph = Graph::new();
    for v in pre_requisites.iter() {
        let prev = v.first().unwrap();
        let last = v.last().unwrap();
        course_graph.add_edge(prev, last);
    }

    course_graph
}

// 课程规划
fn course_scheduling<T>(
    cg: &mut Graph<T>,
    course: Vertex<T>,
    schedule: &mut Vec<String>,
    mut has_circle: bool)
    where T: Eq + PartialEq + Clone + Hash + Display
{
    // 克隆，防止可变引用冲突
    let edges = cg.edges.clone();
    // 对依赖课程进行探索
    let dependencies = edges.get(&course.key);
    if !dependencies.is_none() {
        for dependency in dependencies.unwrap().iter() {
            let course = cg.vertices.get(dependency).unwrap().clone();
            if Color::White == course.color {
                cg.vertices.get_mut(dependency).unwrap().color = Color::Gray;
                course_scheduling(cg, course, schedule, has_circle);
                if has_circle {
                    return; // 遇到环，退出
                }
            } else if Color::Gray == course.color {
                has_circle = true; // 遇到环，退出
                return;
            }
        }
    }

    // 修改课程节点颜色，表示当前课程节点探索完成，加入 schedule
    cg.vertices.get_mut(&course.key).unwrap().color = Color::Black;
    schedule.push(course.key.to_string());
}

fn find_topological_order<T>(course_num: usize, pre_requisites: Vec<Vec<T>>)
    where T: Eq + PartialEq + Clone + Hash + Display
{
    // 构建课程关系图
    let mut cg = build_course_graph(pre_requisites);

    // 获取所有的课程节点到 courses
    let vertices = cg.vertices.clone();
    let mut courses = Vec::new();
    for key in vertices.keys() {
        courses.push(key);
    }
    // 保存可行的课程安排
    let mut schedule = Vec::new();
    // 是否有环
    let has_circle = false;

    // 对课程进行拓扑排序
    for i in 0..course_num {
        let course = cg.vertices.get(&courses[i]).unwrap().clone();
        // 无环且课程节点未被探索过才进行下一步探索
        if !has_circle && Color::White == course.color {
            // 修改课程节点颜色，表示当前节点正被探索
            cg.vertices.get_mut(&courses[i]).unwrap().color = Color::Gray;
            course_scheduling(&mut cg, course, &mut schedule, has_circle);
        }
    }

    if !has_circle {
        println!("{:#?}", schedule);
    }
}

fn main() {
    let operation_num = 9;

    // 构建做菜顺序依赖关系
    let mut pre_requisites = Vec::<Vec<&str>>::new();
    pre_requisites.push(vec!["混合", "3/4杯牛奶"]);
    pre_requisites.push(vec!["混合", "一个鸡蛋"]);
    pre_requisites.push(vec!["混合", "一勺橄榄油"]);
    pre_requisites.push(vec!["倒入1/4杯", "混合"]);
    pre_requisites.push(vec!["倒入1/4杯", "加热锅"]);
    pre_requisites.push(vec!["底面金黄翻面", "倒入1/4杯"]);
    pre_requisites.push(vec!["享用", "底面金黄翻面"]);
    pre_requisites.push(vec!["享用", "加热糖浆"]);

    // 找到拓扑排序结果，即合理的做菜顺序
    find_topological_order(operation_num, pre_requisites);
}
