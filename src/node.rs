use crate::node_types::LeafNodeType; 

#[derive(Debug)]
pub struct LeafNode {
    pub identifier: String,
    pub n_type: LeafNodeType,
    pub description: String
}

#[derive(Debug)]
pub struct LeafListNode {
    pub identifier: String,
    pub n_type: LeafNodeType,
    pub description: String
}
