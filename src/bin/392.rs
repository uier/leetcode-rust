fn main() {
    let tests = [("abc", "ahbgdc"), ("axc", "ahbgdc")];
    let answers = [true, false];
    for ((s, t), expected_answer) in tests.into_iter().zip(answers) {
        let answer = Solution::is_subsequence(s.to_string(), t.to_string());
        assert_eq!(expected_answer, answer);
    }
}
struct Solution;
impl Solution {
    pub fn is_subsequence(s: String, t: String) -> bool {
        let mut matching = 0;
        let sv = s.as_bytes();
        for tb in t.as_bytes() {
            if sv[matching] == *tb {
                matching += 1;
            }
            if matching == s.len() {
                return true;
            }
        }
        false
    }
}
