fn main() {
    let tests = [
        vec![vec![1, 2, 3], vec![4, 5, 6], vec![7, 8, 9]],
        vec![
            vec![1, 2, 3, 4, 5],
            vec![6, 7],
            vec![8],
            vec![9, 10, 11],
            vec![12, 13, 14, 15, 16],
        ],
        vec![vec![1]],
        vec![vec![1], vec![2]],
    ];
    let answers = [
        vec![1, 4, 2, 7, 5, 3, 8, 6, 9],
        vec![1, 6, 2, 8, 7, 3, 9, 4, 12, 10, 5, 13, 11, 14, 15, 16],
        vec![1],
        vec![1, 2],
    ];
    for (test, expected_answer) in tests.into_iter().zip(answers) {
        let answer = Solution::find_diagonal_order(test);
        assert_eq!(expected_answer, answer);
    }
}
struct Solution;
impl Solution {
    pub fn find_diagonal_order(nums: Vec<Vec<i32>>) -> Vec<i32> {
        let mut ans = vec![];
        let mut buffer = vec![vec![]; 100001];
        for (i, arr) in nums.iter().enumerate() {
            for (j, &x) in arr.iter().enumerate() {
                buffer[i + j].push(x);
            }
        }
        for i in 0..100001 {
            while let Some(x) = buffer[i].pop() {
                ans.push(x);
            }
        }
        ans
    }
}
