fn main() {
    let tests = [
        build_tree(vec![
            Some(1),
            Some(3),
            Some(2),
            Some(5),
            Some(3),
            None,
            Some(9),
        ]),
        build_tree(vec![Some(1), Some(2), Some(3)]),
    ];
    let answers = [vec![1, 3, 9], vec![1, 3]];
    for (test, expected_answer) in tests.into_iter().zip(answers) {
        let answer = Solution::largest_values(test);
        assert_eq!(answer, expected_answer);
    }
}
type Node = Option<Rc<RefCell<TreeNode>>>;
fn build_tree(arr: Vec<Option<i32>>) -> Node {
    let nodes = arr
        .iter()
        .map(|&x| x.map(|v| Rc::new(RefCell::new(TreeNode::new(v)))))
        .collect::<Vec<Node>>();
    let n = nodes.len();
    for i in 0..n {
        match &nodes[i] {
            Some(v) => {
                if (i * 2 + 1) < n {
                    v.borrow_mut().left = nodes[i * 2 + 1].clone();
                }
                if (i * 2 + 2) < n {
                    v.borrow_mut().right = nodes[i * 2 + 2].clone();
                }
            }
            None => {}
        }
    }
    nodes[0].clone()
}
struct Solution;
// Definition for a binary tree node.
#[derive(Debug, PartialEq, Eq)]
pub struct TreeNode {
    pub val: i32,
    pub left: Node,
    pub right: Node,
}
impl TreeNode {
    #[inline]
    pub fn new(val: i32) -> Self {
        TreeNode {
            val,
            left: None,
            right: None,
        }
    }
}
use std::cell::RefCell;
use std::collections::VecDeque;
use std::rc::Rc;
impl Solution {
    pub fn largest_values(root: Option<Rc<RefCell<TreeNode>>>) -> Vec<i32> {
        if root.is_none() {
            return vec![];
        }
        let mut queue = VecDeque::new();
        let mut ans = vec![];
        queue.push_back((root.unwrap(), 0));
        while let Some((u, i)) = queue.pop_front() {
            if i >= ans.len() {
                ans.push(u.borrow().val);
            } else {
                ans[i] = ans[i].max(u.borrow().val);
            }
            if let Some(v) = &u.borrow().left {
                queue.push_back((Rc::clone(v), i + 1));
            }
            if let Some(v) = &u.borrow().right {
                queue.push_back((Rc::clone(v), i + 1));
            }
        }
        ans
    }
}
