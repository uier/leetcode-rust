fn main() {
    let tests = [
        (vec![1, 4, 3, 7, 4, 5], 3),
        //       3(15)   3(12)   7, 4(8), 4(12)
        (vec![5, 5, 4, 5, 4, 1, 1, 1], 0),
        (vec![2], 0),
        (vec![2, 3, 4, 3, 1], 2),
        (vec![2, 3, 4, 3, 1], 3),
        (vec![2, 3, 4, 3, 1], 4),
    ];
    let answers = [15, 20, 2, 9, 9, 5];
    for (test, expected_answer) in tests.into_iter().zip(answers) {
        let answer = Solution::maximum_score(test.0, test.1);
        assert_eq!(answer, expected_answer);
    }
}
struct Solution;
impl Solution {
    pub fn maximum_score(nums: Vec<i32>, k: i32) -> i32 {
        let mut minimum = nums[k as usize];
        let mut ans = minimum;
        let mut l = k - 1;
        let mut r = k + 1;
        let n = nums.len() as i32;
        while l >= 0 || r < n {
            if (r >= n) || (l >= 0 && nums[l as usize] >= nums[r as usize]) {
                minimum = minimum.min(nums[l as usize]);
                l -= 1;
            } else {
                minimum = minimum.min(nums[r as usize]);
                r += 1;
            }
            ans = ans.max(minimum * (r - l - 1));
        }
        ans
    }
}
