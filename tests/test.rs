use serde_yang::ser::serializer::to_string;
use serde::Serialize;

#[derive(Serialize, Debug, Clone)]
#[serde(rename="leaf--message")]
struct MessageLeaf {

    #[serde(rename="type--")]
    message_type: String,

    #[serde(rename="description--")]
    message_description: String

}

#[derive(Serialize, Debug, Clone)]
#[serde(rename="leaf-list--domain-search")]
struct DomainSearchLeafList {
    #[serde(rename="type--")]
    domain_type: String,

    #[serde(rename="description--")]
    domain_description: String
}

#[derive(Serialize, Debug, Clone)]
#[serde(rename="container--login")]
struct LoginContainer {
    message_leaf: MessageLeaf
}

#[test]
fn test_basic_leaf() {
    let message_leaf = MessageLeaf {
        message_type: String::from("string"),
        message_description: String::from(r#"This is a very important message"#)
    };
    println!("{}", to_string(&message_leaf).unwrap().as_str());
}

#[test]
fn test_basic_container() {

    let message_leaf = MessageLeaf {
        message_type: String::from("string"),
        message_description: String::from(r#"This is a very important message"#)
    };
    let login_container = LoginContainer {
        message_leaf: message_leaf
    };
    println!("{}", to_string(&login_container).unwrap().as_str());
}


#[test]
fn test_basic_leaf_list() {
    let domain_list = DomainSearchLeafList{
        domain_type: String::from("string"),
        domain_description: String::from(r#"these are where we get all the domains"#)
    };
    println!("{}", to_string(&domain_list).unwrap().as_str());
}