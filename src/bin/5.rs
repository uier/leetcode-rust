fn main() {
    let tests = ["babad", "cbbd", "cbdbddbdb"];
    let answers = ["bab", "bb", "bdbddbdb"];
    for (test, expected_answer) in tests.into_iter().zip(answers) {
        let test = test.to_string();
        let expected_answer = expected_answer.to_string();
        let answer = Solution::longest_palindrome(test);
        assert_eq!(answer, expected_answer);
    }
}
struct Solution;
impl Solution {
    pub fn longest_palindrome_boundary(s: &[u8], mut l: i32, mut r: usize) -> (usize, usize) {
        while l >= 0 && r < s.len() && s[l as usize] == s[r] {
            l -= 1;
            r += 1;
        }
        ((l + 1) as usize, r - 1)
    }
    pub fn longest_palindrome(s: String) -> String {
        let s = s.as_bytes();
        let n = s.len();
        let mut max_p_len = 1;
        let mut ans_l = 0;
        for i in 1..n {
            let (l, r) = Self::longest_palindrome_boundary(s, i as i32, i);
            if r - l + 1 > max_p_len {
                ans_l = l;
                max_p_len = r - l + 1;
            }
            let (l, r) = Self::longest_palindrome_boundary(s, (i - 1) as i32, i);
            if r - l + 1 > max_p_len {
                ans_l = l;
                max_p_len = r - l + 1;
            }
        }
        String::from_utf8(s[ans_l..ans_l + max_p_len].to_vec()).unwrap()
    }
}
