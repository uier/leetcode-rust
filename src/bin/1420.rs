fn main() {
    let tests = [(2, 3, 1), (5, 2, 3), (9, 1, 1)];
    let answers = [6, 0, 1];
    for ((n, m, k), expected_answer) in tests.into_iter().zip(answers) {
        let answer = Solution::num_of_arrays(n, m, k);
        assert_eq!(answer, expected_answer);
    }
}
struct Solution;
impl Solution {
    pub fn num_of_arrays(n: i32, m: i32, k: i32) -> i32 {
        const MOD: i64 = 1000000007;
        let mut dp = vec![vec![vec![0_i64; (k + 1) as usize]; (m + 1) as usize]; (n) as usize];
        for j in 1..=(m as usize) {
            dp[0][j][1] = 1;
        }
        for i in 1..(n as usize) {
            for j in 1..=(m as usize) {
                for l in 1..=(k as usize) {
                    dp[i][j][l] += (j as i64) * dp[i - 1][j][l];
                    dp[i][j][l] %= MOD;
                    for p in 1..(j as usize) {
                        dp[i][j][l] += dp[i - 1][p][l - 1];
                        dp[i][j][l] %= MOD;
                    }
                }
            }
        }
        let mut answer = 0;
        for j in 1..=(m as usize) {
            answer += dp[(n - 1) as usize][j][k as usize];
            answer %= MOD;
        }
        answer as i32
    }
}
