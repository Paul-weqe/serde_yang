/// 
/// used to build leaf nodes. 
/// For example the following code snippet
/// 
///  ```ignore
/// let mut output = String::new();
/// output += leaf_node_builder::open_node("host-name");
/// output += leaf_node_builder::add_type("string");
/// output += leaf_node_builder::description("Hostname for this system");
/// output += leaf_node_builder::close_node();
/// println!("{}", output);
/// ```
/// 
/// Will give us:
/// 
/// leaf host-name {
///     type string;
///     description "Hostname for this system";
/// }
/// 
/// According to YANG's IETF document, we can have the type of the leaf 
/// without having the type e.g:
/// 
/// 
///  ```ignore
/// let mut output = String::new();
/// output += leaf_node_builder::open_node("host-name");
/// output += leaf_node_builder::add_type("string");
/// output ++ leaf_node_builder::close_node();
/// println!("{}", output);
/// ```
/// 
pub (crate) mod leaf_node_builder {

    pub (crate) fn open_node<'a>(node_name: &str) -> String {
        let mut result = String::from("");
        result += "leaf ";
        result += node_name;
        result += "{\n";
        result
    }

    pub (crate) fn description(desc: &str) -> String {
        let mut result = String::from("");
        result += "description \"";
        result += desc;
        result += "\";\n";
        result
    }

    pub (crate) fn set_type(node_type: &str) -> String {
        let mut result = String::from("type ");
        result += node_type;
        result += ";\n";
        result
    }


    
    pub (crate) fn close_node() -> String {
        String::from("}")
    }

}

pub (crate) mod leaf_list_node_builder {
    pub (crate) fn open_node(node_name: &str) -> String {
        let mut result = String::from("");
        result += "leaf-list ";
        result += node_name;
        result += "{\n";
        result
    }    
}

pub (crate) mod container_node_builder {
    pub (crate) fn open_node(node_name: &str) -> String {
        let mut result = String::from("");
        result += "container ";
        result += node_name;
        result += "{\n";
        result
    }
}