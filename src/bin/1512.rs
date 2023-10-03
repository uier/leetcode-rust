fn main() {
    let tests = [
        vec![1, 2, 3, 1, 1, 3],
        vec![1, 1, 1, 1],
        vec![1, 2, 3],
        vec![1, 3, 4, 2, 2, 3, 1],
    ];
    let answers = [4, 6, 0, 3];
    for (test, expected_answer) in tests.into_iter().zip(answers) {
        let answer = Solution::num_identical_pairs(test);
        assert_eq!(answer, expected_answer);
    }
}
struct Solution;
use std::collections::HashMap;
impl Solution {
    pub fn num_identical_pairs(nums: Vec<i32>) -> i32 {
        nums.iter()
            .fold(HashMap::<i32, i32>::from([(0, 0)]), |mut acc, &x| {
                *acc.get_mut(&0).unwrap() +=
                    *acc.entry(x).and_modify(|cnt| *cnt += 1).or_insert(1) - 1;
                acc
            })[&0]
    }
}
