fn main() {
    let tests = [
        (vec![4, 6, 5, 9, 3, 7], vec![0, 0, 2], vec![2, 3, 5]),
        (
            vec![-12, -9, -3, -12, -6, 15, 20, -25, -20, -15, -10],
            vec![0, 1, 6, 4, 8, 7],
            vec![4, 4, 9, 7, 9, 10],
        ),
    ];
    let answers = [
        vec![true, false, true],
        vec![false, true, false, false, true, true],
    ];
    for (test, expected_answer) in tests.into_iter().zip(answers) {
        let answer = Solution::check_arithmetic_subarrays(test.0, test.1, test.2);
        assert_eq!(expected_answer, answer);
    }
}
struct Solution;
impl Solution {
    pub fn check_arithmetic_subarrays(nums: Vec<i32>, l: Vec<i32>, r: Vec<i32>) -> Vec<bool> {
        let mut ans = vec![];
        for (&li, ri) in l.iter().zip(r) {
            ans.push(true);
            let mut arr = nums[li as usize..=ri as usize].to_vec();
            arr.sort();
            let diff = arr[0] - arr[1];
            for (i, &x) in arr.iter().enumerate().skip(2) {
                if diff != arr[i - 1] - x {
                    *ans.last_mut().unwrap() = false;
                    break;
                }
            }
        }
        ans
    }
}
