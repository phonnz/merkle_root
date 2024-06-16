extern crate hex;
extern crate sha2;

use sha2::{Digest, Sha256};

#[derive(Clone)]
struct Node {
    left: Option<Box<Node>>,
    right: Option<Box<Node>>,
    value: String,
    content: String,
    is_copied: bool,
}

impl Node {
    fn new(
        left: Option<Box<Node>>,
        right: Option<Box<Node>>,
        value: String,
        content: String,
        is_copied: bool,
    ) -> Node {
        Node {
            left,
            right,
            value,
            content,
            is_copied,
        }
    }

    fn hash(val: &str) -> String {
        let mut hasher = Sha256::new();
        hasher.update(val);
        hex::encode(hasher.finalize())
    }

    fn copy(&self) -> Node {
        Node::new(
            self.left.clone(),
            self.right.clone(),
            self.value.clone(),
            self.content.clone(),
            true,
        )
    }
}

struct MerkleTree {
    root: Node,
}

impl MerkleTree {
    fn new(values: Vec<&str>) -> MerkleTree {
        MerkleTree {
            root: MerkleTree::build_tree(&values),
        }
    }

    fn build_tree(values: &Vec<&str>) -> Node {
        let mut leaves: Vec<Node> = values
            .iter()
            .map(|&e| Node::new(None, None, e.to_string(), e.to_string(), false))
            .collect();
        if leaves.len() % 2 == 1 {
            let last = leaves.last().unwrap().copy();
            leaves.push(last);
        }
        MerkleTree::build_tree_rec(&mut leaves)
    }

    fn build_tree_rec(nodes: &mut Vec<Node>) -> Node {
        if nodes.len() % 2 == 1 {
            let last = nodes.last().unwrap().copy();
            nodes.push(last);
        }

        let half = nodes.len() / 2;

        if nodes.len() == 2 {
            let value = Node::hash(&(nodes[0].value.clone() + &nodes[1].value));
            let content = nodes[0].content.clone() + "+" + &nodes[1].content;
            return Node::new(
                Some(Box::new(nodes.remove(0))),
                Some(Box::new(nodes.remove(0))),
                value,
                content,
                false,
            );
        }

        let mut left_nodes = nodes.drain(0..half).collect();
        let mut right_nodes = nodes.split_off(0);

        let mut left = MerkleTree::build_tree_rec(&mut left_nodes);
        let mut right = MerkleTree::build_tree_rec(&mut right_nodes);

        let value = Node::hash(&(left.value.clone() + &right.value));
        let content = left.content.clone() + "+" + &right.content;

        Node::new(
            Some(Box::new(left)),
            Some(Box::new(right)),
            value,
            content,
            false,
        )
    }

    fn print_tree(&self) {
        self.print_tree_rec(&self.root);
    }

    fn print_tree_rec(&self, node: &Node) {
        if let Some(ref left) = node.left {
            println!("Left: {}", left.value);
            println!("Right: {}", node.right.as_ref().unwrap().value);
        } else {
            println!("Input");
        }

        if node.is_copied {
            println!("(Padding)");
        }
        println!("Value: {}", node.value);
        println!("Content: {}", node.content);
        println!("");

        if let Some(ref left) = node.left {
            self.print_tree_rec(left);
        }
        if let Some(ref right) = node.right {
            self.print_tree_rec(right);
        }
    }

    fn get_root_hash(&self) -> &str {
        &self.root.value
    }
}

fn main() {
    let elems = vec![
        "77d519a56a3bb197bca02ed25f880a122487914556d587588e633c8368d13053",
        "915961583d426ff5d6726ee59ff7e1ad234d8343f60c57ab023b21741fdba723",
        "7a172559f818c9d9f750b20f9fb16ed89879df47c20e03ffeaa3026c1d297646",
        "2163a680ddcd3b7dcfb444d1e19395e59c63781c09e29c8d4b66bfd6460ca142",
    ];
    println!("Inputs: ");
    for elem in &elems {
        print!("{} | ", elem);
    }
    println!("\n");

    let mtree = MerkleTree::new(elems);
    println!("Root Hash: {}\n", mtree.get_root_hash());
    mtree.print_tree();
}
