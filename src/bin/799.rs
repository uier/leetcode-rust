fn main() {
    let tests = [
        (1, 1, 1),
        (2, 1, 1),
        (100000009, 33, 17),
        (8, 3, 0),
        (8, 3, 1),
        (8, 4, 1),
    ];
    let answers = [0.0, 0.5, 1.0, 0.125, 0.875, 0.0];
    for ((p, i, j), expected_answer) in tests.into_iter().zip(answers) {
        let answer = Solution::champagne_tower(p, i, j);
        assert_eq!(expected_answer, answer);
    }
}
struct Solution;
impl Solution {
    pub fn champagne_tower(poured: i32, query_row: i32, query_glass: i32) -> f64 {
        let mut dp = vec![vec![(0.0, 0.0); 100]; 2];
        let mut cur: usize = 0;
        dp[cur][0] = (
            (1.0_f64).min(poured as f64),
            (0.0_f64).max((poured - 1) as f64),
        );
        for i in 1..=query_row as usize {
            cur ^= 1;
            let pre: usize = cur ^ 1;
            dp[cur][0] = (
                (1.0_f64).min(dp[pre][0].1 / 2.0),
                (0.0_f64).max(dp[pre][0].1 / 2.0 - 1.0),
            );
            dp[cur][i] = (
                (1.0_f64).min(dp[pre][i - 1].1 / 2.0),
                (0.0_f64).max(dp[pre][i - 1].1 / 2.0 - 1.0),
            );
            for j in 1..i {
                let sum = dp[pre][j - 1].1 / 2.0 + dp[pre][j].1 / 2.0;
                dp[cur][j] = ((1.0_f64).min(sum), (0.0_f64).max(sum - 1.0));
            }
        }
        dp[cur][query_glass as usize].0
    }
}
