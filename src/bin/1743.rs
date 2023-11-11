fn main() {
    let tests = [
        vec![vec![2, 1], vec![3, 4], vec![3, 2]],
        vec![vec![4, -2], vec![1, 4], vec![-3, 1]],
        vec![vec![100000, -100000]],
    ];
    let answers = [vec![1, 2, 3, 4], vec![-2, 4, 1, -3], vec![100000, -100000]];
    for (test, expected_answer) in tests.into_iter().zip(answers) {
        let answer = Solution::restore_array(test);
        let mut revans = answer.clone();
        revans.reverse();
        assert!(answer == expected_answer || revans == expected_answer);
    }
}
struct Solution;
use std::collections::HashMap;
impl Solution {
    pub fn restore_array(adjacent_pairs: Vec<Vec<i32>>) -> Vec<i32> {
        let mut neighbors = HashMap::<i32, Vec<i32>>::with_capacity(adjacent_pairs.len() + 1);
        for pair in adjacent_pairs {
            neighbors.entry(pair[0]).or_default().push(pair[1]);
            neighbors.entry(pair[1]).or_default().push(pair[0]);
        }
        let ends: Vec<i32> = neighbors
            .iter()
            .filter(|(_, v)| v.len() == 1)
            .map(|(k, _)| *k)
            .collect();
        let mut ans = vec![ends[0]];
        let mut tail = ends[0];
        let mut prev_n = ends[0];
        while tail != ends[1] {
            for &n in neighbors.get(&tail).unwrap() {
                if n != prev_n {
                    ans.push(n);
                    prev_n = tail;
                    tail = n;
                    break;
                }
            }
        }
        ans
    }
}
