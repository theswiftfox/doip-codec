use std::fmt;

#[derive(Clone, Copy, Debug, PartialEq)]
// Node Type//
pub enum NodeType {
    DoipGateway = 0x00,
    DoipNode = 0x01,
}

impl fmt::Display for NodeType {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let node_type_strings = match self {
            NodeType::DoipGateway => "DoIP gateway",
            NodeType::DoipNode => "DoIP node",
        };
        write!(f, "{}", node_type_strings)
    }
}
