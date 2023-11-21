use yang::ser::to_string;
use serde::Serialize;

fn debug(msg: &str) {
    println!("\x1b[93m");
    println!("{}", msg);
    println!("\x1b[0m");

}

#[test]
fn test_basic_leaf_node() {

    #[serde(rename="leaf-node--host-name")]
    #[derive(Serialize, Debug)]
    struct HostName { }

    let test = HostName {  };
    let result = to_string(&test).unwrap();
    let expected = "leaf host-name{}";
    assert_eq!(
        result, 
        expected
    );
    debug(result.as_str());
}



#[test]
fn test_basic_leaf_list_node() {
    #[serde(rename="leaf-list-node--domain-search")]
    #[derive(Serialize, Debug)]
    struct DomainSearch { }


    let domain_list = DomainSearch{};
    let result = to_string(&domain_list).unwrap();
    let expected = "leaf-list domain-search{}";
    assert_eq!(
        result, // result we have gotten 
        expected// result we expect
    );
    debug(result.as_str());
}

#[test]
fn test_basic_container_node() {

    #[serde(rename="container--system")]
    #[derive(Serialize, Debug)]
    struct System {  }

    let system = System{};
    let result = to_string(&system).unwrap();
    let expected = "container system{}";

    debug(&result.as_str());
    assert_eq!(
        result, 
        expected
    );
}

