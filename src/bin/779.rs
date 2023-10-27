fn main() {
    let tests = [
        (1, 1),
        (2, 1),
        (2, 2),
        (3, 1),
        (3, 2),
        (3, 3),
        (3, 4),
        (29, 1),
        (29, 126870912),
        (30, 4),
        (30, 526870912),
    ];
    let answers = [0, 0, 1, 0, 1, 1, 0, 0, 0, 0, 1];
    for (test, expected_answer) in tests.into_iter().zip(answers) {
        let answer = Solution::kth_grammar(test.0, test.1);
        assert_eq!(answer, expected_answer, "{} {}", test.0, test.1);
    }
}
struct Solution;
impl Solution {
    pub fn kth_grammar(n: i32, k: i32) -> i32 {
        let mut k = k;
        let mut ans = 0;
        for i in (2..=n).rev() {
            if k > (1 << (i - 2)) {
                k &= !(1 << (i - 2));
                ans ^= 1;
            }
        }
        ans
    }
}
