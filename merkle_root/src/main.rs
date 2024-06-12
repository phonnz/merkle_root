use sha256::digest;
use std::{fs::read_to_string, path::Path};

#[derive(Debug, Clone)]
struct MerkleNode {
    hash: String,
    left: Option<Box<MerkleNode>>,
    right: Option<Box<MerkleNode>>,
}

impl MerkleNode {
    fn new(data: &str) -> Self {
        let hash = digest(data.to_string());
        MerkleNode {
            hash,
            left: None,
            right: None,
        }
    }

    fn from_children(left: MerkleNode, right: MerkleNode) -> Self {
        let concatenated_hash = format!("{}{}", left.hash, right.hash);
        let hash = digest(concatenated_hash);
        MerkleNode {
            hash,
            left: Some(Box::new(left)),
            right: Some(Box::new(right)),
        }
    }
}

struct MerkleTree {
    root: MerkleNode,
}

impl MerkleTree {
    fn new(transactions: Vec<&str>) -> Self {
        let mut nodes: Vec<MerkleNode> = transactions.into_iter().map(MerkleNode::new).collect();

        while nodes.len() > 1 {
            if nodes.len() % 2 != 0 {
                nodes.push(nodes.last().unwrap().clone());
            }

            let mut new_level = Vec::new();
            for i in (0..nodes.len()).step_by(2) {
                let left = nodes[i].clone();
                let right = nodes[i + 1].clone();
                let parent = MerkleNode::from_children(left, right);
                new_level.push(parent);
            }
            nodes = new_level;
        }

        MerkleTree {
            root: nodes[0].clone(),
        }
    }

    fn root_hash(&self) -> String {
        self.root.hash.clone()
    }
}

fn main() {
    let transactions: String = read_to_string(Path::new("../../input.txt")).unwrap();
    let merkle_tree = MerkleTree::new(transactions);

    println!("MerkleRoot Hash: {}", merkle_tree.root_hash());
}
