use leetcode_prelude::vec_string;
fn main() {
    let tests = vec![
        vec_string!("01", "10"),
        vec_string!("00", "01"),
        vec_string!("111", "011", "001"),
    ];
    let answers = vec_string!("00", "10", "000");
    for (test, expected_answer) in tests.into_iter().zip(answers) {
        let answer = Solution::find_different_binary_string(test);
        assert_eq!(answer, expected_answer);
    }
}
struct Solution;
use std::collections::HashSet;
impl Solution {
    pub fn find_different_binary_string(nums: Vec<String>) -> String {
        let n = nums.len();
        let mut set = HashSet::new();
        for str in nums {
            let num = i32::from_str_radix(&str, 2).unwrap();
            set.insert(num);
        }
        let mut ans = vec!['0'; n];
        for i in 0..(1 << n) {
            if !set.contains(&i) {
                let mut i = i;
                for j in 1..=n {
                    if i >= (1 << (n - j)) {
                        i -= 1 << (n - j);
                        ans[j - 1] = '1';
                    }
                }
                break;
            }
        }
        ans.iter().collect()
    }
}
