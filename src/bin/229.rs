fn main() {
    let tests = [
        vec![3, 2, 3],
        vec![1],
        vec![1, 2],
        vec![2, 3, 4, 2, 5, 1, 4, 2, 5],
        vec![2, 3, 4, 2, 5, 1, 4, 2, 5, 2],
        vec![2, 2],
    ];
    let answers = [vec![3], vec![1], vec![1, 2], vec![], vec![2], vec![2]];
    for (test, expected_answer) in tests.into_iter().zip(answers) {
        let answer = Solution::majority_element(test);
        assert_eq!(answer, expected_answer);
    }
}
struct Solution;
impl Solution {
    pub fn majority_element(nums: Vec<i32>) -> Vec<i32> {
        let mut major = (std::i32::MAX, std::i32::MAX);
        let mut cnt = (0, 0);
        for (_, &n_i) in nums.iter().enumerate() {
            if n_i == major.0 {
                cnt.0 += 1;
            } else if n_i == major.1 {
                cnt.1 += 1;
            } else if cnt.0 == 0 {
                major.0 = n_i;
                cnt.0 = 1
            } else if cnt.1 == 0 {
                major.1 = n_i;
                cnt.1 = 1;
            } else {
                cnt = (cnt.0 - 1, cnt.1 - 1);
            }
        }
        vec![major.0, major.1]
            .into_iter()
            .filter(|x| nums.iter().filter(|&y| y == x).count() > nums.len() / 3)
            .collect()
        // nums.iter()
        //     .fold(
        //         std::collections::HashMap::<i32, i32>::new(),
        //         |mut state, &x| {
        //             state.entry(x).and_modify(|cnt| *cnt += 1).or_insert(1);
        //             state
        //         },
        //     )
        //     .into_iter()
        //     .filter(|&(_, v)| v > (nums.len() / 3) as i32)
        //     .map(|(k, _)| k)
        //     .collect::<Vec<i32>>()
    }
}
