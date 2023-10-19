fn main() {
    let tests = [0, 1, 2, 3, 4, 5];
    let answers = [
        vec![1],
        vec![1, 1],
        vec![1, 2, 1],
        vec![1, 3, 3, 1],
        vec![1, 4, 6, 4, 1],
        vec![1, 5, 10, 10, 5, 1],
    ];
    for (test, expected_answer) in tests.into_iter().zip(answers) {
        let answer = Solution::get_row(test);
        assert_eq!(answer, expected_answer);
    }
}
struct Solution;
impl Solution {
    pub fn get_row(row_index: i32) -> Vec<i32> {
        let mut ans = vec![0; row_index as usize + 1];
        ans[0] = 1;
        for i in 1..=row_index as usize {
            for j in (1..=i).rev() {
                ans[j] += ans[j - 1];
            }
        }
        ans
    }
}
