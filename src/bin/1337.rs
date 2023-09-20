fn main() {
    let tests = [
        (
            vec![
                vec![1, 1, 0, 0, 0],
                vec![1, 1, 1, 1, 0],
                vec![1, 0, 0, 0, 0],
                vec![1, 1, 0, 0, 0],
                vec![1, 1, 1, 1, 1],
            ],
            3,
        ),
        (
            vec![
                vec![1, 0, 0, 0],
                vec![1, 1, 1, 1],
                vec![1, 0, 0, 0],
                vec![1, 0, 0, 0],
            ],
            2,
        ),
    ];
    let answers = [vec![2, 0, 3], vec![0, 2]];
    for ((mat, k), expected_answer) in tests.into_iter().zip(answers) {
        let answer = Solution::k_weakest_rows(mat, k);
        assert_eq!(answer, expected_answer);
    }
}
struct Solution;
impl Solution {
    pub fn k_weakest_rows(mat: Vec<Vec<i32>>, k: i32) -> Vec<i32> {
        let mut weakness = mat
            .iter()
            .enumerate()
            .map(|(i, v)| (v.iter().sum::<i32>(), i))
            .collect::<Vec<_>>();

        weakness.sort();

        weakness
            .iter()
            .take(k as usize)
            .map(|(_, i)| *i as i32)
            .collect()
    }
}
