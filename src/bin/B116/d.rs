fn main() {
    assert_eq!(Solution::sum_counts(vec![1, 2, 1]), 15);
    assert_eq!(Solution::sum_counts(vec![2, 2]), 3);
    let mut test = vec![];
    for i in 1..=100000 {
        test.push(i);
    }
    assert_eq!(Solution::sum_counts(test), 497683716);
}
struct Solution;
struct Tree {
    seg: Vec<i64>,
    tag: Vec<i64>,
}
impl Tree {
    fn new(n: i32) -> Self {
        Tree {
            seg: vec![0; (n as usize + 1) * 4],
            tag: vec![0; (n as usize + 1) * 4],
        }
    }
    fn push(&mut self, l: usize, r: usize, d: usize) {
        let mid = l + r >> 1;
        self.seg[d << 1] += self.tag[d] * (mid - l + 1) as i64;
        self.seg[d << 1 | 1] += self.tag[d] * (r - mid) as i64;
        self.tag[d << 1] += self.tag[d];
        self.tag[d << 1 | 1] += self.tag[d];
        self.tag[d] = 0;
    }
    fn modify(&mut self, l: usize, r: usize, d: usize, ql: usize, qr: usize) {
        if qr < l || r < ql {
            return;
        }
        if ql <= l && r <= qr {
            self.seg[d] += (r - l + 1) as i64;
            self.tag[d] += 1;
            return;
        }
        if self.tag[d] > 0 {
            self.push(l, r, d);
        }
        let mid = l + r >> 1;
        self.modify(l, mid, d << 1, ql, qr);
        self.modify(mid + 1, r, d << 1 | 1, ql, qr);
        self.seg[d] = self.seg[d << 1] + self.seg[d << 1 | 1];
    }
    fn query(&mut self, l: usize, r: usize, d: usize, ql: usize, qr: usize) -> i64 {
        if qr < l || r < ql {
            return 0;
        }
        if ql <= l && r <= qr {
            return self.seg[d] as i64;
        }
        if self.tag[d] > 0 {
            self.push(l, r, d);
        }
        let mid = l + r >> 1;
        self.query(l, mid, d << 1, ql, qr) + self.query(mid + 1, r, d << 1 | 1, ql, qr)
    }
}
use std::collections::HashMap;
impl Solution {
    pub fn sum_counts(nums: Vec<i32>) -> i32 {
        const MOD: i64 = 1000000007;
        let mut ans = 0i64;
        let n = nums.len();
        let mut tree = Tree::new(n as i32);
        let mut last_i = HashMap::<i32, i32>::new();
        let mut prev = 0i64;
        for (i, &ni) in nums.iter().enumerate() {
            let j: i32 = *last_i.get(&ni).unwrap_or(&-1);
            let dpi = (prev
                + (i as i64 - j as i64)
                + (2i64 * (tree.query(1, n, 1, (j + 1 + 1) as usize, i + 1) % MOD)) % MOD)
                % MOD;
            tree.modify(1, n, 1, (j + 1 + 1) as usize, i + 1);
            ans = (ans + dpi) % MOD;
            prev = dpi;
            last_i
                .entry(ni)
                .and_modify(|v| *v = i as i32)
                .or_insert(i as i32);
        }
        ans as i32
    }
}
// [1, 2, 1]
// [1, ]
// dp[i] = dp[]
// a[0..=i], a[1..=i], a[2..=i], ...
// j = the right most index that a[j] == a[i]
// a[k(k<=j)..=i] = a[k(k<=j)..=i-1]
// ~~~~1^2 + 2^2 + ... + (i-j)^2~~~~
