// trie.rs

// 字典树
#[derive(Default)]
struct Trie {
    root: Node,
}

// 节点
#[derive(Default)]
struct Node {
    end: bool,
    children: [Option<Box<Node>>; 26], // 字母节点列表
}

impl Trie {
    fn new() -> Self {
        Self::default()
    }

    // 单词插入
    fn insert(&mut self, word: &str) {
        let mut node = &mut self.root;
        // 逐个字符插入
        for c in word.as_bytes() {
            let index = (c - b'a') as usize;
            let next = &mut node.children[index];
            node = next.get_or_insert_with(Box::<Node>::default);
        }
        node.end = true;
    }

    // 判断单词是否存在
    fn contains(&self, word: &str) -> bool {
        self.word_node(word).map_or(false, |n| n.end)
    }

    // 判断是否存在以某个前缀开头的单词
    fn start_with(&self, prefix: &str) -> bool {
        self.word_node(prefix).is_some()
    }

    // 前缀字符串
    // wps: word_prefix_string
    fn word_node(&self, wps: &str) -> Option<&Node> {
        let mut node = &self.root;
        for c in wps.as_bytes() {
            let index = (c - b'a') as usize;
            match &node.children[index] {
                None => return None,
                Some(next) => node = next.as_ref(),
            }
        }

        Some(node)
    }
}

fn main() {
    let mut trie = Trie::new();
    trie.insert("box"); trie.insert("insert");
    trie.insert("apple"); trie.insert("appeal");

    let res1 = trie.contains("apple");
    let res2 = trie.contains("apples");
    let res3 = trie.start_with("ins");
    let res4 = trie.start_with("ina");
    println!("word 'apple' in Trie: {res1}");
    println!("word 'apples' in Trie: {res2}");
    println!("prefix 'ins' in Trie: {res3}");
    println!("prefix 'ina' in Trie: {res4}");
}
