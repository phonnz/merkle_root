mod common;
mod merkle_tree;

#[test]
fn test_merkle_node_new() {
    let data = "data";
    let node = MerkleNode::new(data);
    assert_eq!(node.hash, data.to_string());
    assert!(node.left.is_none());
    assert!(node.right.is_none());
}
