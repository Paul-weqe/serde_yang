use crate::{
    node::{LeafListNode, LeafNode},
    node_types::{LeafNodeType, NodeType} 
};


struct NodeGenerator 
{
    node_type: NodeType
}

impl NodeGenerator
{

    fn new(node_type: NodeType) -> Self 
    {
        Self {
            node_type: node_type
        }
    }

    fn generate<R: BaseNode>(&mut self) -> Box<dyn BaseNode> 
    {
        match self.node_type
        {
            NodeType::Leaf => {
                Box::new(LeafNode::default())
            },
            NodeType::LeafList => {
                Box::new(LeafListNode::default())
            }
        }
    }
}

trait BaseNode {
    fn name(&mut self) -> String;
}

impl Default for LeafNode 
{
    fn default() -> Self 
    {
        Self 
        {
            identifier: String::from(""),
            n_type: LeafNodeType::Unidentified,
            description: String::from("")
        }
    }
}





impl BaseNode for LeafNode 
{

    fn name(&mut self) -> String 
    {
        String::from("leaf")
    }
}


impl Default for LeafListNode 
{
    fn default() -> Self 
    {
        Self 
        {
            identifier: String::from(""),
            n_type: LeafNodeType::Unidentified,
            description: String::from("")
        }
    }
}


impl BaseNode for LeafListNode 
{

    fn name(&mut self) -> String 
    {
        String::from("leaf-list")
    }

}
