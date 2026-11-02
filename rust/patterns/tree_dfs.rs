use std::cell::RefCell;
use std::rc::Rc;

pub type Node = Option<Rc<RefCell<TreeNode>>>;

#[derive(Debug)]
pub struct TreeNode {
    pub val: i32,
    pub left: Node,
    pub right: Node,
}

pub fn has_path_sum(root: Node, target: i32) -> bool {
    fn dfs(node: &Node, remaining: i32) -> bool {
        let Some(rc) = node else { return false };
        let n = rc.borrow();
        let remaining = remaining - n.val;
        if n.left.is_none() && n.right.is_none() {
            return remaining == 0;
        }
        dfs(&n.left, remaining) || dfs(&n.right, remaining)
    }
    dfs(&root, target)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn cases() {
        fn build(vals: &[Option<i32>]) -> Node {
            let nodes: Vec<Node> = vals.iter().map(|v| v.map(|val| Rc::new(RefCell::new(TreeNode { val, left: None, right: None })))).collect();
            let mut child = 1;
            for n in nodes.iter().flatten() {
                if child < nodes.len() { n.borrow_mut().left = nodes[child].clone(); child += 1; }
                if child < nodes.len() { n.borrow_mut().right = nodes[child].clone(); child += 1; }
            }
            nodes.first().cloned().flatten()
        }
        assert!(has_path_sum(build(&[Some(5), Some(4), Some(8), Some(11), None, Some(13), Some(4), Some(7), Some(2), None, None, None, Some(1)]), 22));
        assert!(!has_path_sum(build(&[Some(1), Some(2), Some(3)]), 5));
        assert!(!has_path_sum(None, 0));
    }
}
