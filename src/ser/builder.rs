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

    pub (crate) fn add_type(node_type: &str) -> String {
        let mut result = String::from("type ");
        result += node_type;
        result += ";\n";
        result
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