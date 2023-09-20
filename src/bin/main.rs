fn main() {
    let tests = [1, 2, 3];
    let answers = [1, 4, 9];
    for ((mat, k), expected_answer) in tests.into_iter().zip(answers) {
        let answer = Solution::k_weakest_rows(mat, k);
        assert_eq!(answer, expected_answer);
    }
}
struct Solution;
