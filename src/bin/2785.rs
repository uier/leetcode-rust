use leetcode_prelude::vec_string;

fn main() {
    let tests = vec_string!("lEetcOde", "lYmpH", "a", "c", "ea");
    let answers = vec_string!("lEOtcede", "lYmpH", "a", "c", "ae");
    for (test, expected_answer) in tests.into_iter().zip(answers) {
        let answer = Solution::sort_vowels(test);
        assert_eq!(answer, expected_answer);
    }
}
struct Solution;
impl Solution {
    pub fn sort_vowels(s: String) -> String {
        let s = s.chars();
        let mut vowels = s
            .clone()
            .filter(|&c| "AEIOUaeiou".contains(c))
            .collect::<Vec<char>>();
        vowels.sort();
        vowels.reverse();
        s.map(|c| {
            if "AEIOUaeiou".contains(c) {
                vowels.pop().unwrap()
            } else {
                c
            }
        })
        .collect()
    }
}
