
[![crates.io](https://img.shields.io/crates/v/serde_yang.svg)](https://crates.io/crates/serde_yang)

**FURTHER CHANGES WILL BE UPDATED**
<p style="color: red;">Current changes are being made for the serializer, work on the deserializer is still in TODO</p>

# Serde YANG

Serde is a rust library used for <u>ser</u>ialization and <u>de</u>serialization. serde YANG is used for serialization/deserialization of the YANG modelling language.

```
[dependencies]
serde_yang = "0.1.0"
```

YANG is a data serialization format that is capable of conveying one or multiple documents in a single resource. The format is typically not used to transport actual data but rather to model data that will be sent via other formats e.g JSON, XML etc. 

The language is heavily utilized in the networking world with it being used to model data conveyed in NETCONF and RESTCONF. 

A sample basic data model by YANG:

```yang
leaf host-name {
    type string;
    description "Hostname for this system";
}
```

## Creating YANG by serializing data structures

Let us see how we serialize data from Rust data types to yang strings:

```rust
use serde_yang::ser::serializer::to_string;
use serde::Serialize;

#[derive(Serialize, Debug)]
#[serde(rename="leaf--message")]
struct MessageLeaf {

    #[serde(name="type--")]
    message_type: String,

    #[serde(name="description--")]
    message_description: String

}

fn main() {
    let leaf = MessageLeaf { 
        message_type: String::from("string"),
        message_description: String::from(r#"This field will be used to hold the message"#)
     };
     println!("{}", to_string(&leaf).unwrap().as_str());

     /*
     * Expected:
     * leaf message {
     * type string;
     * description "This field will be used to hold the message";
     * }
     */
}

```

### A sample YANG Node
Let us look at the lines very important for the serde yang operation:
```rust
#[serde(rename="leaf--message")]
```

This field declares a YANG leaf node for us. Leaf nodes will be defined in the form of `leaf--{nodename}` where nodename is the name of the node. Which will lead to creation of `leaf nodename {`

The leaf node is just an example of a node defined in YANG. We have various nodes that are defined by YANG. For a list of all the fields supported by serde_yang, have a look at [these node types](https://docs.rs/serde_yang/latest/serde_yang/ser/types/enum.NodeType.html). 

The nodes may hold individual fields in them or may contain other nodes inside them. 


### A sample YANG field
While nodes contain either fields or other nodes, fields hold information only about themselves. They may define the type, description and so much more. 

In the example above, we have defined the following types:
```rust
#[serde(name="type--")]
message_type: String,

#[serde(name="description--")]
message_description: String
```

You may have noticed that unlike the `leaf--{nodename}`, here we just have `type--` or `description--`. This is a characteristic of fields in serde YANG. Currently the fields being supported are `type` and `description` but more will be added as time goes. 

<p style="color: red;">Major changes are being made for the serializer, work on the deserializer is still in TODO. Version 1.0.0 should have a more comprehensive serde implementation but have fun in the meanwhile. :)</p>