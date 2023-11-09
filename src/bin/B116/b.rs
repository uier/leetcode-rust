struct Solution;
impl Solution {
    pub fn min_changes(s: String) -> i32 {
        let mut ans = 0;
        let s = s.as_bytes();
        let n = s.len();
        for i in 0..n / 2 {
            if s[i * 2] != s[i * 2 + 1] {
                ans += 1;
            }
        }
        ans
    }
}
