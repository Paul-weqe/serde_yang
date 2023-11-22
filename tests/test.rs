use yang::ser::serializer::to_string;
use serde::Serialize;

fn debug(msg: &str) {
    println!("\x1b[93m");
    println!("{}", msg);
    println!("\x1b[0m");
}

#[test]
fn test_basic_container_node() {


    #[derive(Serialize, Debug, Clone)]
    #[serde(rename="container--system")]
    struct System {
        inner: Inner,
        interface: Interface
    }

    #[derive(Serialize, Debug, Clone)]
    #[serde(rename="leaf--message")]
    struct Inner {

        #[serde(rename="type--")]
        node_type: String,

        #[serde(rename="description--")]
        node_description: String,
        
    }

    #[derive(Serialize, Debug, Clone)]
    #[serde(rename="leaf-list--interfaces")]
    struct Interface {
        #[serde(rename="type--")]
        interface_type: String,

        #[serde(rename="description--")]
        interface_description: String
    }

    let inner = Inner{
        node_type: String::from("string"),
        node_description: String::from("Message given "),
    };

    let interface = Interface{
        interface_type: String::from("string"),
        interface_description: String::from("This is the Gigabit Ethernet")
    };

    let system = System{
        inner: inner.clone(),
        interface: interface.clone()
    };


    println!("-------------------------- INNER --------------------------\n");
    debug(to_string(&inner).unwrap().as_str());

    println!("---------------------- SYSTEM CONTAINER ---------------------\n");
    debug(to_string(&system).unwrap().as_str());
    
    println!("\n------------------------------------------------------------");

}

