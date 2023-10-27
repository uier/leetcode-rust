fn main() {
    let tests = [
        vec![2, 3, 3, 2, 2, 4, 2, 3, 4],
        vec![2, 1, 2, 2, 3, 3],
        vec![2, 2, 2, 3, 3, 1, 1, 1, 1],
        vec![2, 2, 2, 3, 3, 1, 1, 1, 1, 1],
        vec![2, 2, 2, 3, 3, 1, 1, 1, 1, 1, 1],
        vec![2, 2, 2, 3, 3, 1, 1, 1, 1, 1, 1, 1],
        vec![2, 2, 2, 3, 3, 1, 1, 1, 1, 1, 1, 1, 1],
    ];
    let answers = [4, -1, 4, 4, 4, 5, 5];
    for (test, expected_answer) in tests.into_iter().zip(answers) {
        let answer = Solution::min_operations(test);
        assert_eq!(answer, expected_answer);
    }
}
struct Solution;
impl Solution {
    pub fn min_operations(nums: Vec<i32>) -> i32 {
        let mut ans = 0;
        let mut counter = std::collections::HashMap::<i32, i32>::new();
        for num in nums {
            counter.entry(num).and_modify(|v| *v += 1).or_insert(1);
        }
        for (_k, v) in counter {
            if v < 2 {
                return -1;
            }
            ans += v / 3;
            if v % 3 > 0 {
                ans += 1;
            }
        }
        ans
    }
}
