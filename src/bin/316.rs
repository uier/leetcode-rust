fn main() {
    let tests = ["bcabc", "cbacdcbc", "cbacdc", "cdadabcc", "abacb"];
    let answers = ["abc", "acdb", "bacd", "adbc", "abc"];
    for (s, expected_answer) in tests.into_iter().zip(answers) {
        let answer = Solution::remove_duplicate_letters(s.to_string());
        assert_eq!(answer, expected_answer);
    }
}
struct Solution;
use std::collections::HashSet;
use std::iter::FromIterator;
impl Solution {
    pub fn min_begin_cover_all_char(s: &[u8], mut set: HashSet<u8>) -> usize {
        let backup = set.clone();
        let n = s.len();
        let mut ans = n;
        for i in (0..n).rev() {
            set.remove(&s[i]);
            if set.is_empty() && backup.contains(&s[i]) && (ans == n || s[i] <= s[ans]) {
                ans = i;
            }
        }
        ans
    }
    pub fn remove_duplicate_letters(s: String) -> String {
        let n = s.len();
        let mut char_set = HashSet::from_iter(s.bytes());
        let mut ans = String::new();
        let mut l = 0;
        for _ in 0..char_set.len() {
            l += Self::min_begin_cover_all_char(&s.as_bytes()[l..n], char_set.clone());
            char_set.remove(&s.as_bytes()[l]);
            ans.push(s.chars().nth(l).unwrap());
            l += 1;
        }
        ans
    }
}
