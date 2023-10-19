fn main() {
    let tests = [
        (3, 2),
        (2, 4),
        (4, 2),
        (1, 2),
        (2, 2),
        (2, 3),
        (3, 3),
        (500, 969997),
    ];
    let answers = [4, 2, 8, 1, 2, 2, 4, 374847123];
    for (test, expected_answer) in tests.into_iter().zip(answers) {
        let answer = Solution::num_ways(test.0, test.1);
        assert_eq!(answer, expected_answer);
    }
}
struct Solution;
impl Solution {
    pub fn num_ways(steps: i32, arr_len: i32) -> i32 {
        // steps <= 500, meaning that rightest location is 250
        let mut dp = vec![vec![0; 251]; 2];
        let arr_len = arr_len.min(251);
        const MOD: i32 = 1000000007;
        dp[0][0] = 1;
        for i in 1..=steps as usize {
            let cur = i % 2;
            let pre = cur ^ 1;
            for j in 0..arr_len as usize {
                dp[cur][j] = dp[pre][j];
                if j as i32 - 1 >= 0 {
                    dp[cur][j] = (dp[cur][j] + dp[pre][j - 1]) % MOD;
                }
                if j as i32 + 1 < arr_len {
                    dp[cur][j] = (dp[cur][j] + dp[pre][j + 1]) % MOD;
                }
            }
            // println!("{:?}\n{:?}\n", dp[pre], dp[cur]);
        }
        dp[(steps % 2) as usize][0]
    }
}
