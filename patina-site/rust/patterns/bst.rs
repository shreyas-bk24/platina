use std::cell::RefCell;
use std::rc::Rc;

pub type Node = Option<Rc<RefCell<TreeNode>>>;

#[derive(Debug)]
pub struct TreeNode {
    pub val: i32,
    pub left: Node,
    pub right: Node,
}

pub fn is_valid_bst(root: Node) -> bool {
    fn check(node: &Node, lo: i64, hi: i64) -> bool {
        let Some(rc) = node else { return true };
        let n = rc.borrow();
        let v = i64::from(n.val);
        lo < v && v < hi
            && check(&n.left, lo, v)
            && check(&n.right, v, hi)
    }
    check(&root, i64::MIN, i64::MAX)
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
        assert!(is_valid_bst(build(&[Some(2), Some(1), Some(3)])));
        assert!(!is_valid_bst(build(&[Some(8), Some(3), Some(10), Some(1), Some(6), None, Some(14), None, None, Some(4), Some(9)])));
        assert!(is_valid_bst(build(&[Some(i32::MIN), None, Some(i32::MAX)])));
        assert!(is_valid_bst(None));
    }
}
