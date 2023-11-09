fn stringify(arr: Vec<&str>) -> Vec<String> {
    arr.iter().map(|&x| x.to_string()).collect()
}
fn main() {
    let tests = [
        (vec![1, 3], 3),
        (vec![1, 2, 3], 3),
        (vec![1, 2], 4),
        (vec![1, 4], 4),
        (vec![1, 3], 5),
    ];
    let answers = [
        stringify(vec!["Push", "Push", "Pop", "Push"]),
        stringify(vec!["Push", "Push", "Push"]),
        stringify(vec!["Push", "Push"]),
        stringify(vec!["Push", "Push", "Pop", "Push", "Pop", "Push"]),
        stringify(vec!["Push", "Push", "Pop", "Push"]),
    ];
    for (test, expected_answer) in tests.into_iter().zip(answers) {
        let answer = Solution::build_array(test.0, test.1);
        assert_eq!(answer, expected_answer);
    }
}
struct Solution;
use std::iter::once;
impl Solution {
    pub fn build_array(target: Vec<i32>, _n: i32) -> Vec<String> {
        let mut ans = vec![];
        for w in once(&0)
            .chain(target.iter())
            .collect::<Vec<&i32>>()
            .windows(2)
        {
            for _ in 0..w[1] - w[0] - 1 {
                ans.push("Push".to_string());
                ans.push("Pop".to_string());
            }
            ans.push("Push".to_string());
        }
        ans
    }
}
