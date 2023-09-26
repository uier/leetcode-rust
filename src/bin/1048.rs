fn main() {
    let tests = [
        vec!["a", "b", "ba", "bca", "bda", "bdca"],
        vec!["xbc", "pcxbcf", "xb", "cxbc", "pcxbc"],
        vec!["abcd", "dbqca"],
        vec!["aaa"],
        vec!["abc", "ab", "a"],
    ];
    let tests_in_string = tests
        .iter()
        .map(|t| t.iter().map(|s| s.to_string()).collect());
    let answers = [4, 5, 1, 1, 3];
    for (words, expected_answer) in tests_in_string.into_iter().zip(answers) {
        let answer = Solution::longest_str_chain(words);
        assert_eq!(answer, expected_answer);
    }
}
struct Solution;
impl Solution {
    pub fn is_predecessor(s1: &[u8], s2: &[u8]) -> bool {
        let mut iter = s1.iter().peekable();
        for b2 in s2 {
            iter.next_if(|b1| **b1 == *b2);
        }
        iter.next().is_none()
    }
    pub fn longest_str_chain(mut words: Vec<String>) -> i32 {
        let n = words.len();
        words.sort_by_key(|x| x.len());
        let mut dp = vec![1; n];
        for i in 0..n {
            for j in 0..i {
                if words[j].len() != words[i].len() - 1 {
                    continue;
                }
                if Self::is_predecessor(&words[j].as_bytes(), &words[i].as_bytes()) {
                    dp[i] = std::cmp::max(dp[i], dp[j] + 1);
                }
            }
            // println!("{} {}", words[i], dp[i]);
        }
        *(dp.iter().max()).unwrap()
    }
}
