fn main() {
    assert_eq!(Solution::find_mode(None), vec![]);
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
    fn find_max_count(
        root: Option<Rc<RefCell<TreeNode>>>,
        cur_val: &mut i32,
        cur_cnt: &mut i32,
        max_cnt: &mut i32,
    ) {
        if root.is_none() {
            return;
        }
        let root = root.unwrap();
        Self::find_max_count(root.borrow().left.clone(), cur_val, cur_cnt, max_cnt);
        if root.borrow().val != *cur_val {
            *cur_val = root.borrow().val;
            *cur_cnt = 1;
        } else {
            *cur_cnt += 1;
        }
        *max_cnt = *max_cnt.max(cur_cnt);
        Self::find_max_count(root.borrow().right.clone(), cur_val, cur_cnt, max_cnt);
    }
    fn collect_modes(
        root: Option<Rc<RefCell<TreeNode>>>,
        cur_val: &mut i32,
        cur_cnt: &mut i32,
        max_cnt: i32,
    ) -> Vec<i32> {
        if root.is_none() {
            return vec![];
        }
        let root = root.unwrap();
        let mut arr = Self::collect_modes(root.borrow().left.clone(), cur_val, cur_cnt, max_cnt);
        if root.borrow().val != *cur_val {
            *cur_val = root.borrow().val;
            *cur_cnt = 1;
        } else {
            *cur_cnt += 1;
        }
        if *cur_cnt == max_cnt {
            arr.push(*cur_val);
        }
        arr.append(&mut Self::collect_modes(
            root.borrow().right.clone(),
            cur_val,
            cur_cnt,
            max_cnt,
        ));
        arr
    }
    pub fn find_mode(root: Option<Rc<RefCell<TreeNode>>>) -> Vec<i32> {
        let mut cur_val = i32::MAX;
        let mut cur_cnt = 0;
        let mut max_cnt = 0;
        Self::find_max_count(root.clone(), &mut cur_val, &mut cur_cnt, &mut max_cnt);
        let mut cur_val = i32::MAX;
        let mut cur_cnt = 0;
        Self::collect_modes(root, &mut cur_val, &mut cur_cnt, max_cnt)
    }
}
