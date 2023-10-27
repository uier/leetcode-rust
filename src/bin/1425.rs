fn main() {
    let tests = [
        (vec![10, 2, -10, 5, 20], 2),
        (vec![-1, -2, -3], 1),
        (vec![10, -2, -10, -5, 20], 2),
        (vec![10, 2, -1, -10, -6, 3, 20], 2),
    ];
    let answers = [37, -1, 23, 28];
    for (test, expected_answer) in tests.into_iter().zip(answers) {
        let answer = Solution::constrained_subset_sum(test.0, test.1);
        assert_eq!(answer, expected_answer);
    }
}
struct Solution;
use std::collections::VecDeque;
impl Solution {
    pub fn constrained_subset_sum(nums: Vec<i32>, k: i32) -> i32 {
        let mut deq = VecDeque::<(i32, usize)>::new();
        let mut ans = std::i32::MIN;
        for (i, &ni) in nums.iter().enumerate() {
            while let Some(x) = deq.pop_front() {
                if i - x.1 <= k as usize {
                    deq.push_front(x);
                    break;
                }
            }
            let dp_i = (ni + deq.front().unwrap_or(&(0, 0)).0, i);
            while let Some(&tup) = deq.back() {
                if tup.0 < dp_i.0 {
                    deq.pop_back();
                } else {
                    break;
                }
            }
            if dp_i.0 > 0 {
                deq.push_back(dp_i);
            }
            ans = ans.max(dp_i.0);
        }
        ans
    }
}
