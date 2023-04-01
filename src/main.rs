use std::collections::HashMap;
use fxhash::FxBuildHasher;

type FxHashMap<K, V> = HashMap<K, V, FxBuildHasher>;

#[derive(Debug, Default)]
struct Node {
    at_end: bool,
    children: FxHashMap<u8, Node>,
}

#[derive(Debug, Default)]
struct Trie {
    root: Node,
    len: usize,
}

impl Trie {
    fn new() -> Self {
        Trie::default()
    }

    fn insert(&mut self, text: &str) {
        let mut current_node = &mut self.root;

        for c in text.bytes() {
            current_node = current_node.children.entry(c).or_default();
        }

        current_node.at_end = true;
        self.len += 1;
    }

    fn contains(&self, text: &str) -> bool {
        let mut current_node = &self.root;

        for c in text.bytes() {
            match current_node.children.get(&c) {
                Some(node) => current_node = node,
                None => return false,
            }
        }

        false
    }

    fn len(&self) -> usize {
        self.len
    }
}

fn main() {
    let mut urls = Trie::new();

    println!("{urls:?}");
    // let mut urls = HashSet::new();
    urls.insert("https://www.reddit.com/r/rust/");
    urls.insert("https://www.manning.com/books/rust-in-action");
    urls.insert("https://www.amazon.co.uk/Rust-Action-Tim-McNamara/dp/1617294551");

    let stub = "https://www";
    let contains_stub = urls.contains(stub);

    println!("Does urls contain '{stub}'?\n {contains_stub}");
    println!("{}", urls.len());
    println!("{urls:#?}");
}
