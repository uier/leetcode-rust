fn main() {
    let tests = [1, 2, 5];
    let answers = [5, 10, 68];
    for (test, expected_answer) in tests.into_iter().zip(answers) {
        let answer = Solution::count_vowel_permutation(test);
        assert_eq!(answer, expected_answer);
    }
}
struct Solution;
impl Solution {
    pub fn count_vowel_permutation(n: i32) -> i32 {
        const MOD: i32 = 1000000007;
        let mut dp = vec![vec![1; 5]; 2];
        for i in 1..n {
            let cur = (i % 2) as usize;
            let pre = cur ^ 1;
            dp[cur][0] = ((dp[pre][1] + dp[pre][2]) % MOD + dp[pre][4]) % MOD;
            dp[cur][1] = (dp[pre][0] + dp[pre][2]) % MOD;
            dp[cur][2] = (dp[pre][1] + dp[pre][3]) % MOD;
            dp[cur][3] = dp[pre][2];
            dp[cur][4] = (dp[pre][2] + dp[pre][3]) % MOD;
        }
        dp[((n - 1) % 2) as usize]
            .iter()
            .fold(0, |acc, &x| (acc + x) % MOD)
    }
}
