fn main() {
    let tests = [vec![5, 1, 3], vec![1, 1, 1], vec![1, 1, 2, 2, 3]];
    let answers = [3, 0, 4];
    for (test, expected_answer) in tests.into_iter().zip(answers) {
        let answer = Solution::reduction_operations(test);
        assert_eq!(answer, expected_answer);
    }
}
struct Solution;
impl Solution {
    pub fn reduction_operations(nums: Vec<i32>) -> i32 {
        let mut nums = nums;
        nums.sort();
        let mut prev = *nums.last().unwrap();
        let mut ans = 0;
        for (i, &num) in nums.iter().rev().skip(1).enumerate() {
            ans += (i + 1) as i32 * (prev != num) as i32;
            prev = num;
        }
        ans
    }
}
