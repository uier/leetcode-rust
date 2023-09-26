fn main() {
    let tests = [
        ("abcd", "abcde"),
        ("", "y"),
        ("aaaa", "aaaab"),
        ("aaaa", "aacaa"),
        ("aaaa", "aaaaa"),
        ("aabbcc", "bbcacaa"),
    ];
    let answers = ['e', 'y', 'b', 'c', 'a', 'a'];
    for ((s, t), expected_answer) in tests.into_iter().zip(answers) {
        let answer = Solution::find_the_difference(s.to_string(), t.to_string());
        assert_eq!(answer, expected_answer);
    }
}
struct Solution;
impl Solution {
    pub fn find_the_difference(s: String, t: String) -> char {
        let mut s = s.chars().collect::<Vec<char>>();
        let mut t = t.chars().collect::<Vec<char>>();
        s.sort();
        t.sort();
        for (si, ti) in s.into_iter().zip(&t) {
            if si != *ti {
                return *ti;
            }
        }
        t[t.len() - 1]
    }
}
