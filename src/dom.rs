use std::collections::{HashMap};

#[derive(Debug)]
pub struct Node {
    // data common to all nodes:
    children: Vec<Node>,

    // data specific to each node type:
    node_type: NodeType,
}

#[derive(Debug)]
enum NodeType {
    Text(String),
    Element(ElementData),
    Comment(String),
}
#[derive(Debug)]
struct ElementData {
    tag_name: String,
    attrs: AttrMap,
}

pub type AttrMap = HashMap<String, String>;

pub fn text(data: String) -> Node {
    Node { children: Vec::new(), node_type: NodeType::Text(data) }
}

pub fn elem(tag_name: String, attrs: AttrMap, children: Vec<Node>) -> Node {
    Node {
        children,
        node_type: NodeType::Element(ElementData { tag_name, attrs })
    }
}

pub fn comment(data: String) -> Node {
    Node { children: Vec::new(), node_type: NodeType::Comment(data)}
}

pub fn pprint(node: Node) -> String {  
    "{node:?}".to_string()
}