fn main() {
    let tests = [vec![5, 2, 0, 3, 1], vec![13], vec![13, 13, 0]];
    let answers = [vec![5, 7, 2, 3, 2], vec![13], vec![13, 0, 13]];
    for (test, expected_answer) in tests.into_iter().zip(answers) {
        let answer = Solution::find_array(test);
        assert_eq!(answer, expected_answer);
    }
}
struct Solution;
impl Solution {
    pub fn find_array(pref: Vec<i32>) -> Vec<i32> {
        pref.iter()
            .scan(0, |state, pi| {
                let ai = *pi ^ *state;
                *state = *pi;
                Some(ai)
            })
            .collect()
    }
}
