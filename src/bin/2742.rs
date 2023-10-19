fn main() {
    let tests = [
        (vec![1, 2, 3, 2], vec![1, 2, 3, 2]),
        (vec![2, 3, 4, 2], vec![1, 1, 1, 1]),
        (vec![777], vec![777]),
        (vec![8, 7, 5, 15], vec![1, 1, 2, 1]),
        (vec![17, 3, 19, 8, 24], vec![2, 1, 2, 2, 2]),
    ];
    let answers = [3, 4, 777, 12, 11];
    for (test, expected_answer) in tests.into_iter().zip(answers) {
        let answer = Solution::paint_walls(test.0, test.1);
        assert_eq!(answer, expected_answer);
    }
}
struct Solution;
impl Solution {
    pub fn paint_walls(cost: Vec<i32>, time: Vec<i32>) -> i32 {
        // try finding min cost subset to cover n walls
        // dp[i][j] = the min cost to cover j walls from 0 to i
        let n = cost.len();
        let mut dp = vec![vec![std::i32::MAX as i64; n + 1]; 2];
        dp[0][0] = 0;
        dp[1][0] = 0;
        for (i, (ci, ti)) in cost.into_iter().zip(time).enumerate() {
            let cur = i % 2;
            let pre = cur ^ 1;
            let walls = n.min((ti + 1) as usize);
            for j in (0..=n).rev() {
                dp[cur][n.min(j + walls)] = dp[cur][n.min(j + walls)]
                    .min(dp[pre][n.min(j + walls)].min(dp[pre][j] + ci as i64));
            }
            for j in 0..walls {
                dp[cur][j] = dp[pre][j];
            }
        }
        dp[(n - 1) % 2][n] as i32
    }
}
