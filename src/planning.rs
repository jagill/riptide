use crate::logical_nodes::LogicalNode;

pub fn visit<F>(mut node: LogicalNode, action: &mut F) -> LogicalNode
where
    F: FnMut(LogicalNode, LogicalNode) -> LogicalNode,
{
    if let Some(parent_box) = node.take_parent() {
        let parent = visit(*parent_box, action);
        action(node, parent)
    } else {
        node
    }
}

pub fn optimize<F>(mut root: LogicalNode, rules: &mut [F]) -> LogicalNode
where
    F: FnMut(LogicalNode, LogicalNode) -> LogicalNode,
{
    for rule in rules {
        root = visit(root, rule);
    }

    root
}
