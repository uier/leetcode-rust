fn main() {
    let tests = [
        vec![1, 3, 4, 2, 2],
        vec![3, 1, 3, 4, 2],
        vec![2, 2, 2, 2, 2],
        vec![1, 1, 2],
        vec![
            8, 7, 1, 10, 17, 15, 18, 11, 16, 9, 19, 12, 5, 14, 3, 4, 2, 13, 18, 18,
        ],
    ];
    let answers = [2, 3, 2, 1, 18];
    for (test, expected_answer) in tests.into_iter().zip(answers) {
        let answer = Solution::find_duplicate(test);
        assert_eq!(answer, expected_answer);
    }
}
struct Solution;
impl Solution {
    pub fn find_duplicate(mut nums: Vec<i32>) -> i32 {
        for i in 0..nums.len() {
            let mut index = nums[i] as usize - 1;
            while i != index {
                if nums[index] == nums[i] {
                    return nums[i];
                }
                nums.swap(i, index);
                index = nums[i] as usize - 1;
            }
        }
        unreachable!("No duplicate found");
    }
}
// impl Solution {
//     pub fn find_duplicate(mut nums: Vec<i32>) -> i32 {
//         nums.sort();
//         let mut prev = nums[0];
//         for i in 1..nums.len() {
//             if nums[i] == prev {
//                 return prev;
//             }
//             prev = nums[i];
//         }
//         unreachable!("No duplicate found")
//     }
// }
