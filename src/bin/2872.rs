fn main() {
    let tests = [
        (
            5,
            vec![vec![0, 2], vec![1, 2], vec![1, 3], vec![2, 4]],
            vec![1, 8, 1, 4, 4],
            6,
        ),
        (
            7,
            vec![
                vec![0, 1],
                vec![0, 2],
                vec![1, 3],
                vec![1, 4],
                vec![2, 5],
                vec![2, 6],
            ],
            vec![3, 0, 6, 1, 5, 2, 1],
            3,
        ),
        (1, vec![], vec![6], 3),
        (2, vec![vec![0, 1]], vec![3, 3], 3),
        (2, vec![vec![0, 1]], vec![3, 5], 4),
    ];
    let answers = [2, 3, 1, 2, 1];
    for ((n, edges, values, k), expected_answer) in tests.into_iter().zip(answers) {
        let answer = Solution::max_k_divisible_components(n, edges, values, k);
        assert_eq!(answer, expected_answer);
    }
}
struct Solution;
impl Solution {
    pub fn dfs(
        x: usize,
        fa: usize,
        tree: &Vec<Vec<usize>>,
        values: &mut Vec<i32>,
        dp: &mut Vec<i32>,
        k: i32,
    ) {
        for &i in &tree[x] {
            if i == fa {
                continue;
            }
            Self::dfs(i, x, tree, values, dp, k);
            values[x] += values[i];
            dp[x] += dp[i];
        }
        values[x] %= k;
        dp[x] += (values[x] == 0) as i32;
    }
    pub fn max_k_divisible_components(
        n: i32,
        edges: Vec<Vec<i32>>,
        values: Vec<i32>,
        k: i32,
    ) -> i32 {
        let mut tree: Vec<Vec<usize>> = vec![vec![]; n as usize];
        for edge in edges {
            tree[edge[0] as usize].push(edge[1] as usize);
            tree[edge[1] as usize].push(edge[0] as usize);
        }
        let mut dp = vec![0; n as usize];
        let mut values = values.iter().map(|x| x % k).collect();
        Self::dfs(0, 0, &tree, &mut values, &mut dp, k);
        dp[0]
    }
}
