fn main() {
    let tests = [vec![2, 3, 5], vec![1, 4, 6, 8, 10]];
    let answers = [vec![4, 3, 5], vec![24, 15, 13, 15, 21]];
    for (test, expected_answer) in tests.into_iter().zip(answers) {
        let answer = Solution::get_sum_absolute_differences(test);
        assert_eq!(answer, expected_answer);
    }
}
struct Solution;
impl Solution {
    pub fn get_sum_absolute_differences(nums: Vec<i32>) -> Vec<i32> {
        let n = nums.len();
        let sum: i32 = nums.iter().sum();
        nums.iter()
            .enumerate()
            .scan(0, |prefix_sum, (i, x)| {
                *prefix_sum += *x;
                let l = (*x) * (i + 1) as i32 - *prefix_sum;
                let r = sum - *prefix_sum - (*x) * (n - i - 1) as i32;
                Some(l + r)
            })
            .collect()
    }
}
