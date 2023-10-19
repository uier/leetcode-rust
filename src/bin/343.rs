fn main() {
    let tests = [2, 10, 3, 4, 5, 6, 7, 8, 9, 10];
    let answers = [1, 36, 2, 4, 6, 9, 12, 18, 27, 36];
    for (test, expected_answer) in tests.into_iter().zip(answers) {
        let answer = Solution::integer_break(test);
        assert_eq!(answer, expected_answer);
    }
}
struct Solution;
impl Solution {
    pub fn integer_break(n: i32) -> i32 {
        match n <= 3 {
            true => n - 1,
            false => match n % 3 {
                0 => 3_i32.pow((n / 3) as u32),
                1 => 4 * 3_i32.pow((n / 3 - 1) as u32),
                2 => 2 * 3_i32.pow((n / 3) as u32),
                _ => panic!(),
            },
        }
    }
}
