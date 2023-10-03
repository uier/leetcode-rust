fn main() {
    let tests = ["Let's take LeetCode contest", "God Ding"];
    let answers = ["s'teL ekat edoCteeL tsetnoc", "doG gniD"];
    for (test, expected_answer) in tests.into_iter().zip(answers) {
        let test = test.to_string();
        let expected_answer = expected_answer.to_string();
        let answer = Solution::reverse_words(test);
        assert_eq!(answer, expected_answer);
    }
}
struct Solution;
impl Solution {
    pub fn reverse_words(s: String) -> String {
        s.split(' ')
            .map(|t| t.chars().rev().collect::<String>())
            .collect::<Vec<String>>()
            .join(" ")
    }
}
