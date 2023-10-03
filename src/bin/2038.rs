fn main() {
    let tests = ["AAABABB", "AA", "ABBBBBBBAAA", "AAABBBAAA"];
    let answers = [true, false, false, true];
    for (s, expected_answer) in tests.into_iter().zip(answers) {
        let answer = Solution::winner_of_game(s.to_string());
        assert_eq!(expected_answer, answer);
    }
}
struct Solution;
impl Solution {
    pub fn winner_of_game(colors: String) -> bool {
        let mut l = 0;
        let colors = colors.as_bytes();
        let mut turns = (0, 0);
        while l < colors.len() {
            let mut r = l + 1;
            while r < colors.len() && colors[l] == colors[r] {
                r += 1;
            }
            if r - l >= 3 {
                if colors[l] == 'A' as u8 {
                    turns.0 += r - l - 2;
                } else {
                    turns.1 += r - l - 2;
                }
            }
            l = r;
        }
        turns.0 > turns.1
    }
}
