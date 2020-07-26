use crate::Cols;

#[derive(PartialEq, Debug)]
pub enum OpType {
    Filter { in_cols: Cols },
    Project { cols: Cols },
    Extend { in_cols: Cols, out_col: usize },
}

pub struct LogicalNode {
    name: String,
    op_type: OpType,
    parent: Option<Box<LogicalNode>>,
}

impl LogicalNode {
    pub fn new(name: String, op_type: OpType, parent: Option<Box<LogicalNode>>) -> Self {
        LogicalNode {
            name,
            op_type,
            parent,
        }
    }
    pub fn name(&self) -> &str {
        &self.name
    }

    pub fn op_type(&self) -> &OpType {
        &self.op_type
    }

    pub fn take_parent(&mut self) -> Option<Box<LogicalNode>> {
        self.parent.take()
    }

    pub fn set_parent(&mut self, parent: Option<Box<LogicalNode>>) {
        self.parent = parent;
    }
}
