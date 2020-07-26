use crate::logical_nodes::{LogicalNode, OpType};
use crate::Cols;

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

fn switch(mut child: LogicalNode, mut parent: LogicalNode) -> LogicalNode {
    let grandparent = parent.take_parent();
    child.set_parent(grandparent);
    parent.set_parent(Some(Box::new(child)));
    return parent;
}

pub fn merge_projections(child: LogicalNode, mut parent: LogicalNode) -> LogicalNode {
    match (child.op_type(), parent.op_type()) {
        (OpType::Project { cols: cols1 }, OpType::Project { cols: cols2 }) => {
            let new_cols: Cols = cols1.intersection(cols2);
            return LogicalNode::new(
                format!("Project [{}]", new_cols.to_string()),
                OpType::Project { cols: new_cols },
                parent.take_parent(),
            );
        }
        _ => child,
    }
}

pub fn push_proj_past_extend(mut child: LogicalNode, mut parent: LogicalNode) -> LogicalNode {
    match (child.op_type(), parent.op_type()) {
        (OpType::Project { cols }, OpType::Extend { in_cols, out_col }) => {
            if !cols.contains(out_col) {
                // This projects out the Extend, so we can drop it
                child.set_parent(parent.take_parent());
            } else if in_cols.is_subset(cols) {
                // We can push the project past extend
                // TODO: Need to drop out_col
                child = switch(child, parent);
            } // TODO: Push past cols union in_cols minus out_col
        }
        _ => (),
    }
    child
}

// static OPTIMIZATION_RULES: [dyn OptimizationRule; 2] = [merge_projections, push_proj_past_extend];
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_merge_projections() {
        let cols1 = Cols::new(vec![1, 2, 3]);
        let p1 = LogicalNode::new("p1".to_owned(), OpType::Project { cols: cols1 }, None);
        let cols2 = Cols::new(vec![1, 3, 5]);
        let p2 = LogicalNode::new(
            "p2".to_owned(),
            OpType::Project { cols: cols2 },
            Some(Box::new(p1)),
        );

        let new_p = visit(p2, &mut merge_projections);
        let expected_cols = Cols::new(vec![1, 3]);
        assert_eq!(
            new_p.op_type(),
            &OpType::Project {
                cols: expected_cols
            }
        );
    }
}
