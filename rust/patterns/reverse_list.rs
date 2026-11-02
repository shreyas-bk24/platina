#[derive(Debug)]
pub struct ListNode {
    pub val: i32,
    pub next: Option<Box<ListNode>>,
}

pub fn reverse_list(head: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
    let (mut prev, mut cur) = (None, head);
    while let Some(mut node) = cur {
        cur = node.next.take();
        node.next = prev;
        prev = Some(node);
    }
    prev
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn cases() {
        fn build(v: &[i32]) -> Option<Box<ListNode>> {
            v.iter().rev().fold(None, |next, &val| Some(Box::new(ListNode { val, next })))
        }
        fn collect(mut l: &Option<Box<ListNode>>) -> Vec<i32> {
            let mut out = Vec::new();
            while let Some(n) = l { out.push(n.val); l = &n.next; }
            out
        }
        assert_eq!(collect(&reverse_list(build(&[1, 2, 3, 4, 5]))), vec![5, 4, 3, 2, 1]);
        assert_eq!(collect(&reverse_list(build(&[7]))), vec![7]);
        assert!(reverse_list(None).is_none());
    }
}
