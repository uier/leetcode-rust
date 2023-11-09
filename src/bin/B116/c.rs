struct Solution;
impl Solution {
    pub fn length_of_longest_subsequence(nums: Vec<i32>, target: i32) -> i32 {
        let sum: i32 = nums.iter().sum();
        if sum < target {
            return -1;
        }
        let mut dp = vec![i32::MIN; 1001];
        let n = nums.len();
        dp[0] = 0;
        for (i, &ni) in nums.iter().enumerate() {
            for j in (0..=(1000 - ni) as usize).rev() {
                if dp[j] != i32::MIN {
                    dp[ni as usize + j] = dp[ni as usize + j].max(dp[j] + 1);
                }
            }
        }
        dp[target as usize].max(-1)
    }
}
