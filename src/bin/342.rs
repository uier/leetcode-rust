fn main() {
    let tests = [16, 5, 1, 2, 3, 4, 9, 15, 17, 63, 64, 65, 256, 0, -1, -2, 1];
    let answers = [
        true, false, true, false, false, true, false, false, false, false, true, false, true,
        false, false, false, true,
    ];
    for (test, expected_answer) in tests.into_iter().zip(answers) {
        let answer = Solution::is_power_of_four(test);
        assert_eq!(answer, expected_answer);
    }
}
struct Solution;
impl Solution {
    pub fn is_power_of_four(n: i32) -> bool {
        n > 0 && {
            let log_n = f64::log2(n as f64); // should = 2x
            let floor_log_n = log_n as i64;
            (log_n - floor_log_n as f64).abs() < f64::EPSILON && floor_log_n % 2 == 0
        }
    }
}
