fn main() {
    let tests = [
        vec![1, 2, 2, 3],
        vec![6, 5, 4, 4],
        vec![1, 3, 2],
        vec![1],
        vec![1, 2],
        vec![1, 2, 3, 2, 1],
    ];
    let answers = [true, true, false, true, true, false];
    for (nums, expected_answer) in tests.into_iter().zip(answers) {
        let answer = Solution::is_monotonic(nums);
        assert_eq!(answer, expected_answer);
    }
}
struct Solution;
// use std::collections::HashSet;
impl Solution {
    pub fn is_monotonic(nums: Vec<i32>) -> bool {
        nums.windows(2).all(|x| x[0] <= x[1]) || nums.windows(2).all(|x| x[0] >= x[1])
        // let h = nums
        //     .windows(2)
        //     .map(|x| std::cmp::max(-1, std::cmp::min(1, x[1] - x[0])))
        //     .into_iter()
        //     .collect::<HashSet<_>>();
        // !(h.contains(&-1) && h.contains(&1))
    }
}
