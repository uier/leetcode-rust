use leetcode_prelude::vec_string;

fn main() {
    let tests = [
        (vec_string!("G", "P", "GP", "GG"), vec![2, 4, 3]),
        (vec_string!("MMM", "PGM", "GP"), vec![3, 10]),
    ];
    let answers = [21, 37];
    for (test, expected_answer) in tests.into_iter().zip(answers) {
        let answer = Solution::garbage_collection(test.0, test.1);
        assert_eq!(answer, expected_answer);
    }
}
struct Solution;
impl Solution {
    pub fn garbage_collection(garbage: Vec<String>, travel: Vec<i32>) -> i32 {
        let (mut last_g, mut last_m, mut last_p) = (0, 0, 0);
        let mut ans = 0i32;
        let prefix: Vec<i32> = vec![0]
            .iter()
            .chain(travel.iter())
            .scan(0, |acc, &x| {
                *acc += x;
                Some(*acc)
            })
            .collect();
        for (i, s) in garbage.iter().enumerate() {
            for c in s.chars() {
                match c {
                    'G' => last_g = i,
                    'M' => last_m = i,
                    'P' => last_p = i,
                    _ => {}
                }
            }
            ans += s.len() as i32;
        }
        ans + prefix[last_g] + prefix[last_m] + prefix[last_p]
    }
}
