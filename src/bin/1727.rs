fn main() {
    let tests = [
        vec![vec![0, 0, 1], vec![1, 1, 1], vec![1, 0, 1]],
        vec![vec![1, 0, 1, 0, 1]],
    ];
    let answers = [4, 3];
    for (test, expected_answer) in tests.into_iter().zip(answers) {
        let answer = Solution::largest_submatrix(test);
        assert_eq!(answer, expected_answer);
    }
}
struct Solution;
impl Solution {
    pub fn largest_submatrix(matrix: Vec<Vec<i32>>) -> i32 {
        let mut mat = matrix;
        let n = mat.len();
        let m = mat[0].len();
        for j in 0..m {
            for i in 0..n {
                if mat[i][j] == 1 && i != 0 {
                    mat[i][j] += mat[i - 1][j];
                }
            }
        }
        let mut ans = 0;
        for row in mat.iter_mut() {
            row.sort();
            row.reverse();
            for (i, val) in row.iter().enumerate() {
                ans = ans.max((i as i32 + 1) * val);
            }
        }
        ans
    }
}
