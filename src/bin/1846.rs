fn main() {
    let tests = [
        vec![2, 2, 1, 2, 1],
        vec![100, 1, 1000],
        vec![1, 2, 3, 4, 5],
        vec![1],
        vec![1, 2],
        vec![1, 3],
        vec![73, 98, 9],
    ];
    let answers = [2, 3, 5, 1, 2, 2, 3];
    for (test, expected_answer) in tests.into_iter().zip(answers) {
        let answer = Solution::maximum_element_after_decrementing_and_rearranging(test);
        assert_eq!(answer, expected_answer);
    }
}
struct Solution;
impl Solution {
    pub fn maximum_element_after_decrementing_and_rearranging(arr: Vec<i32>) -> i32 {
        let mut arr = arr;
        arr.sort();
        arr.iter().fold(0, |state, &x| x.min(state + 1))
    }
}
