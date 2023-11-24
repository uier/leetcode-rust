fn main() {
    let tests = [
        vec![2, 4, 1, 2, 7, 8],
        // 1, 2, 2, 4, 7, 8
        vec![2, 4, 5],
        vec![9, 8, 7, 6, 5, 1, 2, 3, 4],
    ];
    let answers = [9, 4, 18];
    for (points, expected_answer) in tests.into_iter().zip(answers) {
        let answer = Solution::max_coins(points);
        assert_eq!(answer, expected_answer);
    }
}
struct Solution;
impl Solution {
    pub fn max_coins(piles: Vec<i32>) -> i32 {
        let mut piles = piles;
        piles.sort_unstable_by(|a, b| b.cmp(a));
        (1..(piles.len() / 3 * 2))
            .step_by(2)
            .map(|i| piles[i])
            .sum()
    }
}
