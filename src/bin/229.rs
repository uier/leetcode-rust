fn main() {
    let tests = [
        vec![3, 2, 3],
        vec![1],
        vec![1, 2],
        vec![2, 3, 4, 2, 5, 1, 4, 2, 5],
        vec![2, 3, 4, 2, 5, 1, 4, 2, 5, 2],
    ];
    let answers = [vec![3], vec![1], vec![1, 2], vec![], vec![2]];
    for (test, expected_answer) in tests.into_iter().zip(answers) {
        let answer = Solution::majority_element(test);
        assert_eq!(answer, expected_answer);
    }
}
struct Solution;
impl Solution {
    pub fn majority_element(nums: Vec<i32>) -> Vec<i32> {
        nums.iter()
            .fold(
                std::collections::HashMap::<i32, i32>::new(),
                |mut state, &x| {
                    state.entry(x).and_modify(|cnt| *cnt += 1).or_insert(1);
                    state
                },
            )
            .into_iter()
            .filter(|&(_, v)| v > (nums.len() / 3) as i32)
            .map(|(k, _)| k)
            .collect::<Vec<i32>>()
    }
}
