fn main() {
    let tests = [
        (vec![1, 1, 4, 2, 3], 5),
        (vec![5, 6, 7, 8, 9], 4),
        (vec![3, 2, 20, 1, 1, 3], 10),
        (vec![1, 1], 3),
    ];
    let answers = [2, -1, 5];
    for ((nums, x), expected_answer) in tests.into_iter().zip(answers) {
        let answer = Solution::min_operations(nums, x);
        assert_eq!(expected_answer, answer);
    }
}
struct Solution;
use std::cmp::min;
use std::iter::once;
impl Solution {
    pub fn min_operations(nums: Vec<i32>, x: i32) -> i32 {
        if nums.iter().map(|x| *x as i64).sum::<i64>() < x as i64 {
            return -1;
        }
        const NO_SOLUTION: i32 = i32::MAX;
        let prefix: Vec<i64> = once(0)
            .chain(nums.iter().scan(0, |acc, &x| {
                *acc += x;
                Some(*acc as i64)
            }))
            .collect();
        let suffix: Vec<i64> = once(0)
            .chain(nums.iter().rev().scan(0, |acc, &x| {
                *acc += x;
                Some(*acc as i64)
            }))
            .collect();
        // println!("{prefix:?}");
        // println!("{suffix:?}");
        let mut ans = NO_SOLUTION;
        for i in 0..prefix.len() {
            match suffix.binary_search(&(x as i64 - prefix[i])) {
                Ok(j) => ans = min(ans, (i + j) as i32),
                _ => {}
            }
        }
        if ans == NO_SOLUTION {
            -1
        } else {
            ans
        }
    }
}
