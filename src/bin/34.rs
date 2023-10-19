fn main() {
    let tests = [
        (vec![5, 7, 7, 8, 8, 10], 8),
        (vec![5, 7, 7, 8, 8, 10], 6),
        (vec![], 0),
        (vec![1, 2, 3], 1),
        (vec![1, 1, 3], 1),
        (vec![1, 1, 1], 1),
        (vec![0, 1, 1], 1),
        (vec![1, 1, 2, 2], 1),
    ];
    let answers = [
        vec![3, 4],
        vec![-1, -1],
        vec![-1, -1],
        vec![0, 0],
        vec![0, 1],
        vec![0, 2],
        vec![1, 2],
        vec![0, 1],
    ];
    for (test, expected_answer) in tests.into_iter().zip(answers) {
        let answer = Solution::search_range(test.0, test.1);
        assert_eq!(answer, expected_answer);
    }
}
struct Solution;
impl Solution {
    pub fn search_range(nums: Vec<i32>, target: i32) -> Vec<i32> {
        if nums.len() == 0 || target < *nums.first().unwrap() || target > *nums.last().unwrap() {
            return vec![-1, -1];
        }
        let l = nums
            .binary_search_by(|v| match v.cmp(&target) {
                std::cmp::Ordering::Equal => std::cmp::Ordering::Greater,
                ord => ord,
            })
            .or_else(|i| Ok::<usize, usize>(i))
            .unwrap();
        if nums[l] != target {
            vec![-1, -1]
        } else {
            vec![
                l as i32,
                nums.binary_search_by(|v| match v.cmp(&(target + 1)) {
                    std::cmp::Ordering::Equal => std::cmp::Ordering::Greater,
                    ord => ord,
                })
                .or_else(|i| Ok::<usize, usize>(i))
                .unwrap() as i32
                    - 1,
            ]
        }
    }
}
