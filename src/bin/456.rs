fn main() {
    let tests = [
        vec![1, 2, 3, 4],
        vec![3, 1, 4, 2],
        vec![-1, 3, 2, 0],
        vec![1, 3, 2],
        vec![1],
        vec![1, 2],
        vec![1, 2, 3, 2, 1],
        vec![1, 0, 1, -4, -3],
        vec![-2, 1, -2],
        vec![1, 4, 0, -1, -2, -3, -1, -2],
        vec![3, 1, 2],
    ];
    let answers = [
        false, true, true, true, false, false, true, false, false, true, false,
    ];
    for (nums, expected_answer) in tests.into_iter().zip(answers) {
        let answer = Solution::find132pattern(nums.clone());
        assert_eq!(answer, expected_answer, "case: {:?}", nums);
    }
}
struct Solution;
struct SegTree {
    seg: Vec<i32>,
}
impl SegTree {
    fn new(size: usize) -> SegTree {
        SegTree {
            seg: vec![std::i32::MAX; size * 4],
        }
    }
    fn update(&mut self, l: usize, r: usize, d: usize, x: usize, v: i32) {
        if r < x || l > x {
            return;
        }
        if l == r && r == x {
            self.seg[d] = v;
            return;
        }
        let mid = (l + r) >> 1;
        self.update(l, mid, d << 1, x, v);
        self.update(mid + 1, r, d << 1 | 1, x, v);
        self.seg[d] = self.seg[d << 1].min(self.seg[d << 1 | 1]);
    }
    fn query(&self, l: usize, r: usize, d: usize, ql: usize, qr: usize) -> i32 {
        if r < ql || l > qr {
            return std::i32::MAX;
        }
        if ql <= l && r <= qr {
            return self.seg[d];
        }
        let mid = (l + r) >> 1;
        self.query(l, mid, d << 1, ql, qr)
            .min(self.query(mid + 1, r, d << 1 | 1, ql, qr))
    }
}
impl Solution {
    pub fn find132pattern(nums: Vec<i32>) -> bool {
        let mut nums = nums;
        let mut tmp = nums.clone();
        tmp.sort();
        tmp.dedup();
        let n = nums.len();
        let mut tree32 = SegTree::new(n);
        let mut cur_min = std::i32::MAX;
        for elem in &mut nums {
            *elem = tmp.binary_search(elem).unwrap() as i32;
            if *elem > tree32.query(0, n - 1, 1, (*elem + 1) as usize, n - 1) {
                return true;
            }
            tree32.update(0, n - 1, 1, *elem as usize, cur_min);
            cur_min = cur_min.min(*elem);
        }
        false
    }
}
