fn main() {
    let tests = [vec![3, 2, 3], vec![2, 2, 1, 1, 1, 2, 2], vec![1, 2, 1]];
    let answers = [3, 2, 1];
    for (test, expected_answer) in tests.into_iter().zip(answers) {
        let answer = Solution::majority_element(test);
        assert_eq!(answer, expected_answer);
    }
}
struct Solution;
impl Solution {
    pub fn majority_element(nums: Vec<i32>) -> i32 {
        nums.iter()
            .skip(1)
            .fold((nums[0], 1i32), |(major, count), &n_i| {
                if count == 0 {
                    (n_i, 1)
                } else {
                    (major, count + [-1, 1][(major == n_i) as usize])
                }
            })
            .0
        // let mut ans = std::i32::MAX;
        // let mut cnt = 0;
        // for (i, &n_i) in nums.iter().enumerate().take(nums.len() / 2) {
        //     if n_i == nums[nums.len() - 1 - i] {
        //         if cnt == 0 {
        //             ans = n_i;
        //             cnt = 2;
        //         } else if n_i == ans {
        //             cnt += 2;
        //         } else {
        //             cnt -= 2;
        //         }
        //     }
        // }
        // if nums.len() % 2 == 1 && cnt == 0 {
        //     nums[nums.len() / 2]
        // } else {
        //     ans
        // }
    }
}
