fn main() {
    let tests = [
        (
            vec![vec![1, 6], vec![3, 7], vec![9, 12], vec![4, 13]],
            // [1, 3, 4, 9]
            // [7, 8, 13, 14]
            vec![2, 3, 7, 11],
        ),
        (vec![vec![1, 10], vec![3, 3]], vec![3, 3, 2]),
    ];
    let answers = [vec![1, 2, 2, 2], vec![2, 2, 1]];
    for (test, expected_answer) in tests.into_iter().zip(answers) {
        let answer = Solution::full_bloom_flowers(test.0, test.1);
        assert_eq!(answer, expected_answer);
    }
}
struct Solution;
impl Solution {
    pub fn full_bloom_flowers(flowers: Vec<Vec<i32>>, people: Vec<i32>) -> Vec<i32> {
        let mut left: Vec<i32> = flowers.iter().map(|x| x[0]).collect();
        let mut right: Vec<i32> = flowers.iter().map(|x| x[1] + 1).collect();
        let mut ans: Vec<i32> = vec![];
        left.sort();
        right.sort();
        for p in people {
            let i = left.partition_point(|&x| x < p + 1);
            let j = right.partition_point(|&x| x < p + 1);
            ans.push((i - j) as i32);
        }
        ans
    }
}
