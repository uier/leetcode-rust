use leetcode_prelude::vec_string;

fn main() {
    let tests = vec_string!("aabca", "adc", "bbcbaba", "aea", "uuuuu");
    let answers = [3, 0, 4, 1, 1];
    for (test, expected_answer) in tests.into_iter().zip(answers) {
        let answer = Solution::count_palindromic_subsequence(test);
        assert_eq!(answer, expected_answer);
    }
}
struct Solution;
impl Solution {
    fn get_index(c: char) -> usize {
        (c as u32 - 'a' as u32) as usize
    }
    pub fn count_palindromic_subsequence(s: String) -> i32 {
        let n = s.len();
        let mut prefix = vec![vec![0; 26]; n + 1];
        let mut palind = vec![vec![false; 26]; 26];
        for (i, c) in s.chars().enumerate() {
            for j in 0..26 {
                prefix[i + 1][j] = prefix[i][j];
            }
            prefix[i + 1][Self::get_index(c)] += 1;
        }
        for (i, c) in s.chars().enumerate() {
            let k = Self::get_index(c);
            for j in 0..26 {
                if prefix[i][j] > 0 && (prefix[n][j] - prefix[i + 1][j]) > 0 {
                    palind[k][j] = true;
                }
            }
        }
        palind
            .iter()
            .map(|arr| arr.iter().map(|&b| b as i32).sum::<i32>())
            .sum()
    }
}
