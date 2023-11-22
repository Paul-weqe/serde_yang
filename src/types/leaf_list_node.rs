use super::base::{
    BuiltInType, built_in_type_mapping, BaseYangNode, node_type_mapping, NodeType
};

struct LeafNode {
    node_name: String,
    node_type: BuiltInType,
    node_description: Option<String>
}

impl BaseYangNode for LeafNode {
    
    fn to_yang(&self) -> String {

        let mut output = String::from("");
        output += node_type_mapping()[&NodeType::LeafListNode];
        output += self.node_name.as_str();
        output += "{\n";

        match self.node_type {
            BuiltInType::String => {
                output += "type ";
                output += built_in_type_mapping()[&BuiltInType::String];
                output += ";\n";
            },
            _ => unimplemented!()
        }
        if self.node_description.is_some() {
            output += "description \"";
            output += self.node_description.clone().unwrap().as_str();
            output += "\";\n";
        }
        output += "}";
        output
    }
    
}

