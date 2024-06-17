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
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_merkle_node_creation() {
        // Create leaf nodes
        let left_node = MerkleNode {
            hash: "left_hash".to_string(),
            left: None,
            right: None,
        };

        let right_node = MerkleNode {
            hash: "right_hash".to_string(),
            left: None,
            right: None,
        };

        // Create a parent node with the left and right nodes as children
        let parent_node = MerkleNode {
            hash: "parent_hash".to_string(),
            left: Some(Box::new(left_node.clone())),
            right: Some(Box::new(right_node.clone())),
        };

        // Verify the parent node's hash
        assert_eq!(parent_node.hash, "parent_hash");

        // Verify the left child's hash
        assert!(parent_node.left.is_some());
        if let Some(left_child) = &parent_node.left {
            assert_eq!(left_child.hash, "left_hash");
        }

        // Verify the right child's hash
        assert!(parent_node.right.is_some());
        if let Some(right_child) = &parent_node.right {
            assert_eq!(right_child.hash, "right_hash");
        }

        // Verify cloning works correctly
        let cloned_parent_node = parent_node.clone();
        assert_eq!(cloned_parent_node.hash, "parent_hash");
        if let Some(left_child) = &cloned_parent_node.left {
            assert_eq!(left_child.hash, "left_hash");
        }
        if let Some(right_child) = &cloned_parent_node.right {
            assert_eq!(right_child.hash, "right_hash");
        }
    }
}
