fn main() {
    let tests = ["abbcccaa", "xy", "zzzzz"];
    let answers = [13, 2, 15];
    for (test, expected_answer) in tests.into_iter().zip(answers) {
        let answer = Solution::count_homogenous(test.to_string());
        assert_eq!(answer, expected_answer);
    }
}
struct Solution;
impl Solution {
    pub fn count_homogenous(s: String) -> i32 {
        let mut current_repeated_char_count = (None, 0);
        let mut ans = 0;
        const MOD: i32 = 1000000007;
        for c in s.chars().chain(".".chars()) {
            if Some(c) == current_repeated_char_count.0 {
                current_repeated_char_count.1 += 1;
                continue;
            }
            ans = (ans
                + (current_repeated_char_count.1 as i64
                    * (current_repeated_char_count.1 as i64 + 1)
                    / 2
                    % MOD as i64) as i32)
                % MOD;
            current_repeated_char_count = (Some(c), 1);
        }
        ans
    }
}
// impl Solution {
//     pub fn count_homogenous(s: String) -> i32 {
//         let mut repeated_char_count: Vec<(char, i32)> = vec![];
//         const MOD: i32 = 1000000007;
//         for c in s.chars() {
//             if let Some((x, v)) = repeated_char_count.last_mut() {
//                 if *x == c {
//                     *v += 1;
//                     continue;
//                 }
//             }
//             repeated_char_count.push((c, 1));
//         }
//         repeated_char_count.iter().fold(0, |acc, &(_, v)| {
//             (acc + ((v as i64 * (v as i64 + 1) / 2) % MOD as i64) as i32) % MOD
//         })
//     }
// }
