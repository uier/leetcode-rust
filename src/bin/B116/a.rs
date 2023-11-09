struct Solution;
struct Tree {
    seg: Vec<i32>,
    tag: Vec<i32>,
};
impl Tree {
    fn new(n: i32) -> Self {
        Tree {
            seg: vec![0; (n as usize+1) * 4],
            tag: vec![0; (n as usize+1) * 4],
        }
    }
    fn push(&mut self, l: usize, r: usize, d: usize) {
        let mid = l+r>>1;
        self.seg[d<<1] = self.tag[d] * (mid - l + 1) as i32;
        self.seg[d<<1|1] = self.tag[d] * (r - mid) as i32;
        self.tag[d<<1] = self.tag[d];
        self.tag[d<<1|1] = self.tag[d];
        self.tag[d] = 0;
    }
    fn modify(&mut self, l: usize, r: usize, d: usize, ql: usize, qr: usize) {
        if qr < l || r < ql {
            return;
        }
        if ql <= l && r <= qr {
            self.seg[d] += (r - l + 1) as i32;
            self.tag[d] += 1;
            return;
        }
        if self.tag[d] > 0 {
            self.push(l, r, d);
        }
        let mid = l+r>>1;
        self.modify(l, mid, d<<1, ql, qr);
        self.modify(mid+1, r, d<<1|1, ql, qr);
        self.seg[d] = self.seg[d<<1] + self.seg[d<<1|1];
    }
    fn query(&self, l: usize, r: usize, d: usize, ql: usize, qr: usize) -> i64 {
        if qr < l || r < ql {
            return 0;
        } 
        if ql <= l && r <= qr {
            return self.seg[d] as i64;
        }
        if self.tag[d] > 0 {
            self.push(l, r, d);
        }
        let mid = l+r>>1;
        self.query(l, mid, d<<1, ql, qr) + self.query(mid+1, r, d<<1|1, ql, qr)
    }
}
impl Solution {
    pub fn sum_counts(nums: Vec<i32>) -> i32 {
        0
    }
}
// [1, 2, 1]
// [1, ]
// dp[i] = dp[]
// a[0..=i], a[1..=i], a[2..=i], ...
// j = the right most index that a[j] == a[i]
// a[k(k<=j)..=i] = a[k(k<=j)..=i-1]
// ~~~~1^2 + 2^2 + ... + (i-j)^2~~~~
