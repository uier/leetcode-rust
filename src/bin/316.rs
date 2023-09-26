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
        // println!("{:?}", set);
        // println!("{:?}", s);
        let n = s.len();
        let mut ans = n;
        for i in (0..n).rev() {
            set.remove(&s[i]);
            if set.len() == 0 && backup.contains(&s[i]) && (ans == n || s[i] <= s[ans]) {
                ans = i;
            }
        }
        ans
    }
    pub fn remove_duplicate_letters(s: String) -> String {
        let n = s.len();
        let mut l = 0;
        let mut set = HashSet::from_iter(s.bytes());
        let m = set.len();
        let mut ans = String::new();
        for _ in 0..m {
            let ret =
                l + Self::min_begin_cover_all_char(&s.as_bytes()[(l as usize)..n], set.clone());
            set.remove(&s.bytes().nth(ret).unwrap());
            ans.push(s.chars().nth(ret).unwrap());
            l = ret + 1;
        }
        ans
    }
}
