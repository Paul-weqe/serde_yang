pub mod built_in {
    
    use std::collections::HashMap;

    /// These are the default built in types that are supported by YANG modelling 
    /// language. When one creates a leaf or a leaf-list, they must give the type 
    /// of the variable(s) that will be held there, this is a list of the allowed variables
    #[derive(Debug, PartialEq, Eq, Hash)]
    pub enum Type {
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

    /// gives us a mapping between the types in BuiltInTypeEnum 
    /// and the actual string of the type used in the YANG.
    /// 
    /// e.g 
    /// 
    /// ```
    /// use serde_yang::ser::types::built_in::Type;
    /// use serde_yang::ser::types::built_in::type_mapping;
    /// 
    /// println!("{}", type_mapping()[&Type::Empty]);
    /// // This will print out "empty"
    /// ```
    pub fn type_mapping<'a>() -> HashMap<Type, &'a str> 
    {
        HashMap::from([
            (Type::Binary, "binary"),
            (Type::Bits, "bits"),
            (Type::Boolean, "boolean"),
            (Type::Decimal64, "decimal64"),
            (Type::Empty, "empty"),
            (Type::Enumeration, "enumeration"),
            (Type::IdentityRef, "identityref"),
            (Type::InstanceIdentifier, "instance-identifier"),
            (Type::Int8, "int8"),
            (Type::Int16, "int16"),
            (Type::Int32, "int32"),
            (Type::Int64, "int64"),
            (Type::LeafRef, "leafref"),
            (Type::String, "string"),
            (Type::UInt8, "uint8"),
            (Type::UInt16, "uint16"),
            (Type::UInt32, "uint32"),
            (Type::UInt64, "uint64"),
            (Type::Union, "union")
        ])
    }
    
}


pub (crate) mod node {
    use std::collections::HashMap;
    
    #[derive(Debug, PartialEq, Eq, Hash)]
    pub enum Type 
    {
        LeafNode,
        LeafListNode,
        ContainerNode,
        ListNode,
        ModuleNode
    }   

    /// Most of the following has been picked from the YANG IETF
    /// https://datatracker.ietf.org/doc/html/rfc6020
    /// 
    /// 
    pub fn type_mapping<'a>() -> HashMap<Type, &'a str>
    {
        HashMap::from([
            (Type::LeafNode, "leaf"),
            (Type::LeafListNode, "leaf-list"),
            (Type::ContainerNode, "container"),
            (Type::ModuleNode, "module"),
            (Type::ListNode, "list")
        ])
    }



    pub fn serde_mapping<'a>() -> HashMap<Type, &'a str>
    {
        HashMap::from([
            (Type::LeafNode, "leaf--"),
            (Type::LeafListNode, "leaf-list--"),
            (Type::ContainerNode, "container--"),
            (Type::ModuleNode, "module--"),
            (Type::ListNode, "list--")
        ])
    }
}


