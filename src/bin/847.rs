fn main() {
    let tests = [
        vec![vec![1, 2, 3], vec![0], vec![0], vec![0]],
        vec![vec![1], vec![0, 2, 4], vec![1, 3, 4], vec![2], vec![1, 2]],
    ];
    let answers = [4, 4];
    for (g, expected_answer) in tests.into_iter().zip(answers) {
        let answer = Solution::shortest_path_length(g);
        assert_eq!(expected_answer, answer);
    }
}
struct Solution;
use std::cmp::min;
use std::collections::VecDeque;
impl Solution {
    pub fn shortest_path_length(graph: Vec<Vec<i32>>) -> i32 {
        let n = graph.len();
        let mut dp = vec![vec![i32::MAX; 1 << n]; n];
        for i in 0..n {
            let mut queue = VecDeque::from([i as i32]);
            dp[i][1 << i] = 0;
            // for v in &graph[i] {
            //     dp[i][1<<i | 1 << v] = 1;
            // }
            while let Some(u) = queue.pop_front() {
                for v in &graph[u as usize] {
                    if dp[i][1 << v | 1 << i] != i32::MAX || *v == i as i32 {
                        continue;
                    }
                    dp[i][1 << v | 1 << i] = dp[i][1 << u | 1 << i] + 1;
                    queue.push_back(*v);
                }
            }
        }
        for state in 1..1 << n {
            for u in 0..n {
                if state & (1 << u) == 0 {
                    continue;
                }
                for v in 0..n {
                    if u == v || state & (1 << v) == 0 {
                        continue;
                    }
                    if dp[v][state ^ (1 << u)] == i32::MAX {
                        continue;
                    }
                    dp[u][state] = min(
                        dp[u][state],
                        dp[u][(1 << u) | (1 << v)] + dp[v][state ^ (1 << u)],
                    );
                }
            }
        }
        (0..n).map(|i| dp[i][(1 << n) - 1]).min().unwrap()
    }
}
