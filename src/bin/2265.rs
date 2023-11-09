fn main() {
    let tests = [
        build_tree(vec![
            Some(4),
            Some(8),
            Some(5),
            Some(0),
            Some(1),
            None,
            Some(6),
        ]),
        build_tree(vec![Some(1)]),
        build_tree(vec![Some(1), Some(4), Some(1)]),
    ];
    let answers = [5, 1, 2];
    for (test, expected_answer) in tests.into_iter().zip(answers) {
        let answer = Solution::average_of_subtree(test);
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
    pub left: Option<Rc<RefCell<TreeNode>>>,
    pub right: Option<Rc<RefCell<TreeNode>>>,
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
use std::rc::Rc;
impl Solution {
    // fn dp(
    //     root: Option<Rc<RefCell<TreeNode>>>,
    //     sum: &mut HashMap<i32, i32>,
    //     count: &mut HashMap<i32, i32>,
    // ) -> i32 {
    //     match root {
    //         Some(u) => {
    //             let mut ans = 0;
    //             let u = u.borrow();
    //             sum.insert(u.val, u.val);
    //             count.insert(u.val, 1);
    //             if u.left.is_some() {
    //                 ans += Self::dp(u.left.clone(), sum, count);
    //                 let vl = u.left.as_ref().unwrap().val;
    //                 let &&sum_l = &sum.get(&vl).unwrap();
    //                 let &&count_l = &count.get(&vl).unwrap();
    //                 sum.entry(u.val).and_modify(|e| *e += sum_l);
    //                 count.entry(u.val).and_modify(|e| *e += count_l);
    //             }
    //             if u.right.is_some() {
    //                 ans += Self::dp(u.right.clone(), sum, count);
    //                 let vr = u.right.as_ref().unwrap().borrow().val;
    //                 let &&sum_r = &sum.get(&vr).unwrap();
    //                 let &&count_r = &count.get(&vr).unwrap();
    //                 println!("l: {} {} {}", vr, sum_r, count_r);
    //                 println!("{} {} {}", ans, sum[&u.val], count[&u.val]);
    //                 sum.entry(u.val).and_modify(|e| *e += sum_r);
    //                 count.entry(u.val).and_modify(|e| *e += count_r);
    //             }
    //             println!("{} {} {}", ans, sum[&u.val], count[&u.val]);
    //             if sum[&u.val] / count[&u.val] == u.val {
    //                 ans += 1
    //             }
    //             ans
    //         }
    //         None => 0,
    //     }
    // }
    fn dp(root: Option<Rc<RefCell<TreeNode>>>) -> (i32, i32, i32) {
        match root {
            None => (0, 0, 0),
            Some(u) => {
                let mut sum = u.borrow().val;
                let mut count = 1;
                let subtree_l = Self::dp(u.borrow().left.clone());
                let subtree_r = Self::dp(u.borrow().right.clone());
                sum += subtree_l.1 + subtree_r.1;
                count += subtree_l.2 + subtree_r.2;
                if sum / count == u.borrow().val {
                    (subtree_l.0 + subtree_r.0 + 1, sum, count)
                } else {
                    (subtree_l.0 + subtree_r.0, sum, count)
                }
            }
        }
    }
    pub fn average_of_subtree(root: Option<Rc<RefCell<TreeNode>>>) -> i32 {
        // Self::dp(root, &mut HashMap::new(), &mut HashMap::new())
        Self::dp(root).0
    }
}
