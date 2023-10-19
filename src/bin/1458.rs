fn main() {
    let tests = [
        (vec![2, 1, -2, 5], vec![3, 0, -6]),
        (vec![3, -2], vec![2, -6, 7]),
        (vec![-1, -1], vec![1, 1]),
        (
            vec![-3, -8, 3, -10, 1, 3, 9],
            vec![9, 2, 3, 7, -9, 1, -8, 5, -1, -1],
        ),
    ];
    let answers = [18, 21, -1, 200];
    for (test, expected_answer) in tests.into_iter().zip(answers) {
        let answer = Solution::max_dot_product(test.0, test.1);
        assert_eq!(answer, expected_answer);
    }
}
struct Solution;
impl Solution {
    pub fn max_dot_product(nums1: Vec<i32>, nums2: Vec<i32>) -> i32 {
        let n = nums1.len();
        let m = nums2.len();
        let mut dp = vec![vec![0; m]; n];
        dp[0][0] = nums1[0] * nums2[0];
        for i in 1..n {
            dp[i][0] = dp[i - 1][0].max(nums1[i] * nums2[0]);
        }
        for j in 1..m {
            dp[0][j] = dp[0][j - 1].max(nums1[0] * nums2[j]);
        }
        for i in 1..n {
            for j in 1..m {
                dp[i][j] = dp[i][j - 1]
                    .max(dp[i - 1][j].max(dp[i - 1][j - 1].max(0) + nums1[i] * nums2[j]));
            }
        }
        dp[n - 1][m - 1]
    }
}
