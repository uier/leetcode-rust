fn main() {
    let tests = [vec![3, 5, 2, 3], vec![3, 5, 4, 2, 4, 6]];
    let answers = [7, 8];
    for (test, expected_answer) in tests.into_iter().zip(answers) {
        let answer = Solution::min_pair_sum(test);
        assert_eq!(answer, expected_answer);
    }
}
struct Solution;
impl Solution {
    pub fn min_pair_sum(nums: Vec<i32>) -> i32 {
        let mut nums = nums;
        nums.sort();
        nums.iter()
            .zip(nums.iter().rev())
            .take(nums.len() / 2)
            .map(|(x, y)| x + y)
            .max()
            .unwrap()
    }
}
