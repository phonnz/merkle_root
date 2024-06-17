use sha256::digest;
use std::error::Error;
use std::fs::File;
use std::io::{BufRead, BufReader};
use std::path::Path;

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
    // Define the file path
    let file_path = "input.txt";

    // Open the file
    let file = File::open(file_path)?;

    // Create a buffered reader
    let reader = BufReader::new(file);

    // Initialize an empty vector to store lines
    let mut lines: Vec<String> = Vec::new();

    // Iterate over lines in the file
    for line in reader.lines() {
        lines.push(line?);
    }

    // Print the resulting vector (optional)
    println!("Lines as vector of words: {:?}", lines);

    // Define the file path
    let file_path = "input.txt";

    // Open the file
    let file = File::open(file_path)?;

    // Create a buffered reader
    let reader = BufReader::new(file);

    // Initialize an empty vector to store lines
    let mut lines: Vec<String> = Vec::new();

    // Iterate over lines in the file
    for line in reader.lines() {
        lines.push(line?);
    }
    let transactions: Vec<&str> = lines.iter().map(|s| s as &str).collect();
    // Print the resulting vector (optional)
    println!("Lines as vector of words: {:?}", lines);

    /*
    let file = File::open(Path::new("input.txt"));
    let reader = BufReader::new(file);
    let mut transactions: Vec<&str> = Vec::new();
    for line in reader.lines() {
        transactions.push(line);
    }
    //let transactions: String = read_to_string(Path::new("../../input.txt")).unwrap();
    * let transactions = vec![
        "77d519a56a3bb197bca02ed25f880a122487914556d587588e633c8368d13053",
        "915961583d426ff5d6726ee59ff7e1ad234d8343f60c57ab023b21741fdba723",
        "7a172559f818c9d9f750b20f9fb16ed89879df47c20e03ffeaa3026c1d297646",
        "2163a680ddcd3b7dcfb444d1e19395e59c63781c09e29c8d4b66bfd6460ca142",
    ];*/
    let merkle_tree = MerkleTree::new(transactions);

    println!("MerkleRoot Hash: {}", merkle_tree.root_hash());
    Ok(())
}
