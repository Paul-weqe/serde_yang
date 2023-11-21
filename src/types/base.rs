use std::collections::HashMap;


#[derive(Debug, PartialEq, Eq, Hash)]
pub enum BuiltInType {
    Binary,             // Any binary data
    Bits,               // A set of bits or flags
    Boolean,            // "true" or "false"
    Decimal64,          // 64-bit signed decimal number
    Empty,              // A leaf that does not have any value
    Enumeration,        // Enumerated Strings
    IdentityRef,        // A reference to an abstract identity
    InstanceIdentifier, // References a data tree node
    Int8,               // 8-bit signed integer
    Int16,              // 16-bit signed integer
    Int32,              // 32-bit signed integer
    Int64,              // 64-bit signed integer
    LeafRef,            // A reference to a leaf instance
    String,             // Human Readable string
    UInt8,              // 8-Bit unsigned Integer
    UInt16,             // 16-bit unsigned Integer
    UInt32,             // 32-bit unsigned Integer
    UInt64,             // 64-bit unsigned Integer
    Union               // choice of member types
}

#[derive(Debug, PartialEq, Eq, Hash)]
pub enum NodeType {
    LeafNode,
    LeafListNode,
    ContainerNode,
    ListNode,
    ModuleNode
}

pub fn built_in_type_mapping<'a>() -> HashMap<BuiltInType, &'a str> 
{
    HashMap::from([
        (BuiltInType::Binary, "binary"),
        (BuiltInType::Bits, "bits"),
        (BuiltInType::Boolean, "boolean"),
        (BuiltInType::Decimal64, "decimal64"),
        (BuiltInType::Empty, "empty"),
        (BuiltInType::Enumeration, "enumeration"),
        (BuiltInType::IdentityRef, "identityref"),
        (BuiltInType::InstanceIdentifier, "instance-identifier"),
        (BuiltInType::Int8, "int8"),
        (BuiltInType::Int16, "int16"),
        (BuiltInType::Int32, "int32"),
        (BuiltInType::Int64, "int64"),
        (BuiltInType::LeafRef, "leafref"),
        (BuiltInType::String, "string"),
        (BuiltInType::UInt8, "uint8"),
        (BuiltInType::UInt16, "uint16"),
        (BuiltInType::UInt32, "uint32"),
        (BuiltInType::UInt64, "uint64"),
        (BuiltInType::Union, "union")
    ])
}

pub fn node_type_mapping<'a>() -> HashMap<NodeType, &'a str>
{
    HashMap::from([
        (NodeType::LeafNode, "leaf"),
        (NodeType::LeafListNode, "leaf-list"),
        (NodeType::ContainerNode, "container"),
        (NodeType::ModuleNode, "module"),
        (NodeType::ListNode, "list")
    ])
}

pub trait BaseYangNode {
    fn to_yang(&self) -> String;
}

