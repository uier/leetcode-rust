fn main() {
    let tests = [vec![10, 15, 20], vec![1, 100, 1, 1, 1, 100, 1, 1, 100, 1]];
    let answers = [15, 6];
    for (test, expected_answer) in tests.into_iter().zip(answers) {
        let answer = Solution::min_cost_climbing_stairs(test);
        assert_eq!(answer, expected_answer);
    }
}
struct Solution;
impl Solution {
    pub fn min_cost_climbing_stairs(mut cost: Vec<i32>) -> i32 {
        for i in 2..cost.len() {
            cost[i] += cost[i - 1].min(cost[i - 2])
        }
        cost[cost.len() - 1].min(cost[cost.len() - 2])
    }
}
