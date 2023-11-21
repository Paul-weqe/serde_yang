use super::base::{
    BuiltInType, built_in_type_mapping, BaseYangNode
};

struct LeafNode {
    node_name: String,
    node_type: BuiltInType,
    node_description: Option<String>
}

impl BaseYangNode for LeafNode {
    
    fn to_yang(&self) -> String {
        let mut output = String::from("");
        output += "leaf ";
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

#[test]
fn test_basic_leaf_node(){ 

    let leaf_node = LeafNode {
        node_name: String::from("host-name"),
        node_type: BuiltInType::String,
        node_description: Some(String::from("Hostname for this system"))
    };
    let expected = "leaf host-name{\ntype string;\ndescription \"Hostname for this system\";\n}";
    assert_eq!(expected, leaf_node.to_yang());

}

