use std::cell::RefCell;
use std::collections::VecDeque;
use std::rc::Rc;

pub type Node = Option<Rc<RefCell<TreeNode>>>;

#[derive(Debug)]
pub struct TreeNode {
    pub val: i32,
    pub left: Node,
    pub right: Node,
}

pub fn level_order(root: Node) -> Vec<Vec<i32>> {
    let mut levels = Vec::new();
    let mut queue: VecDeque<Rc<RefCell<TreeNode>>> = root.into_iter().collect();
    while !queue.is_empty() {
        let size = queue.len();
        let mut level = Vec::with_capacity(size);
        for _ in 0..size {
            let Some(node) = queue.pop_front() else { break };
            let n = node.borrow();
            level.push(n.val);
            queue.extend(n.left.clone());
            queue.extend(n.right.clone());
        }
        levels.push(level);
    }
    levels
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
        assert_eq!(level_order(build(&[Some(3), Some(9), Some(20), None, None, Some(15), Some(7)])), vec![vec![3], vec![9, 20], vec![15, 7]]);
        assert_eq!(level_order(None), Vec::<Vec<i32>>::new());
    }
}
