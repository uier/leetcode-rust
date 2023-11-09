fn main() {
    let tests = [
        (2, 4, 7, 7, 6),
        (3, 1, 7, 3, 3),
        (1, 2, 1, 2, 1),
        (1, 2, 1, 2, 0),
        (1, 2, 1, 2, 2),
        (1, 2, 1, 2, 3),
    ];
    let answers = [true, false, false, true, true, true];
    for (test, expected_answer) in tests.into_iter().zip(answers) {
        let answer = Solution::is_reachable_at_time(test.0, test.1, test.2, test.3, test.4);
        assert_eq!(answer, expected_answer);
    }
}
struct Solution;
impl Solution {
    pub fn is_reachable_at_time(sx: i32, sy: i32, fx: i32, fy: i32, t: i32) -> bool {
        ((fx - sx).abs().max((fy - sy).abs()) > 0 || t != 1)
            && (fx - sx).abs().max((fy - sy).abs()) <= t
    }
}
