fn main() {
    let tests = [vec![3, 1, 2, 4], vec![0]];
    let answers = [vec![2, 4, 3, 1], vec![0]];
    for (nums, expected_answer) in tests.into_iter().zip(answers) {
        let answer = Solution::sort_array_by_parity(nums);
        assert_eq!(answer, expected_answer);
    }
}
struct Solution;
// impl Solution {
//     pub fn sort_array_by_parity(mut nums: Vec<i32>) -> Vec<i32> {
//         nums.sort_by_key(|x| x % 2);
//         nums
//     }
// }
impl Solution {
    pub fn sort_array_by_parity(nums: Vec<i32>) -> Vec<i32> {
        nums.iter()
            .filter(|x| *x % 2 == 0)
            .chain(nums.iter().filter(|x| *x % 2 == 1))
            .cloned()
            .collect()
    }
}
