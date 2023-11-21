use std::collections::HashMap;


// used to match the description of the node to the 
// pattern we will look for in the struct / enum
// e.g for the leaf, we will be looking for the leaf-node-{nodename} 
// pattern
pub fn node_name_map<'a>() -> HashMap<&'a str, &'a str>{
    HashMap::from([
        ("leaf", "leaf-node--"),
        ("leaf-list", "leaf-list-node--"),
        ("container", "container--")
    ])
}