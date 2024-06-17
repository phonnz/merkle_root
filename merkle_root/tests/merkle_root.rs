mod common;

pub mod root {
    use crate::common;
    use merkle_tree::{MerkleTree, Sha256Algorithm};

    #[test]
    pub fn should_return_a_correct_root() {
        let test_cases = common::get_test_cases();
        for i in 0..test_cases.len() {
            println!("---------------");
            let test_data = common::setup(i);
            let merkle_tree = MerkleTree::<Sha256Algorithm>::from_leaves(&test_data.leaf_hashes);
            merkle_tree.print_tree();
            assert_eq!(
                merkle_tree.root_hex(),
                Some(test_data.expected_root_hex.to_string())
            );
        }
    }
}

pub mod tree_depth {
    use crate::common;
    use merkle_tree::{Hasher, MerkleTree, Sha256Algorithm};

    #[test]
    pub fn should_return_a_correct_tree_depth() {
        let test_data = common::setup(0);

        let merkle_tree = MerkleTree::<Sha256Algorithm>::from_leaves(&test_data.leaf_hashes);

        merkle_tree.print_tree();

        let paths = merkle_tree.find_path_to_root(test_data.leaf_hashes[0]);
        for leaf_node in paths {
            println!("{:?}", Sha256Algorithm::hash_to_string(leaf_node));
        }

        let depth = merkle_tree.depth();
        assert_eq!(depth, 3)
    }
}

use sha256::digest;
use std::error::Error;
use std::fs::File;
use std::io::{BufRead, BufReader};

#[derive(Debug, Clone)]
struct MerkleNode {
    hash: String,
    left: Option<Box<MerkleNode>>,
    right: Option<Box<MerkleNode>>,
}

impl MerkleNode {
    fn new(data: &str) -> Self {
        let hash = data.to_string();
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

fn main() -> Result<(), Box<dyn Error>> {
    let file = File::open("input.txt")?;
    let reader = BufReader::new(file);
    let mut lines: Vec<String> = Vec::new();
    for line in reader.lines() {
        lines.push(line?);
    }
    let transactions: Vec<&str> = lines.iter().map(|s| s as &str).collect();

    let merkle_tree = MerkleTree::new(transactions);

    println!("MerkleRoot Hash: {}", merkle_tree.root_hash());
    Ok(())
}u
