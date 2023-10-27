fn main() {
    let tests = [
        vec![5, 3, 4, 1, 1],
        vec![6, 5, 3, 9, 2, 7],
        vec![3, 2, 5, 5, 2, 3],
        vec![5, 5, 2, 6],
        vec![5, 5, 2, 10],
    ];
    let answers = [13, 22, 18, 14, 16];
    for (test, expected_answer) in tests.into_iter().zip(answers) {
        let answer = Solution::maximum_sum_of_heights(test);
        assert_eq!(answer, expected_answer);
    }
}
struct Solution;
impl Solution {
    pub fn maximum_sum_of_heights(max_heights: Vec<i32>) -> i64 {
        let mut ans = 0_i64;
        for (i, &ni) in max_heights.iter().enumerate() {
            let mut sum: i64 = ni as i64;
            let mut mx_l = ni;
            let mut mx_r = ni;
            for &max_h in max_heights.iter().take(i).rev() {
                mx_l = mx_l.min(max_h);
                sum += mx_l as i64;
            }
            for &max_h in max_heights.iter().skip(i + 1) {
                mx_r = mx_r.min(max_h);
                sum += mx_r as i64;
            }
            ans = ans.max(sum);
        }
        ans
    }
}
