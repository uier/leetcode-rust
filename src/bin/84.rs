fn main() {
    let tests = [
        vec![2, 1, 5, 6, 2, 3],
        vec![2, 4],
        vec![4, 3, 2, 5], // 0, s = [2, 3
        vec![4, 3, 1, 5, 3, 3],
    ];
    let answers = [10, 4, 8, 9];
    for (test, expected_answer) in tests.into_iter().zip(answers) {
        let answer = Solution::largest_rectangle_area(test);
        assert_eq!(answer, expected_answer);
    }
}
struct Solution;
impl Solution {
    pub fn largest_rectangle_area(heights: Vec<i32>) -> i32 {
        let mut heights = heights;
        heights.push(0);
        let mut ans = heights[0];
        let mut stack = vec![0; 1];
        for (i, &hi) in heights.iter().enumerate().skip(1) {
            while let Some(&j) = stack.last() {
                if hi > heights[j] {
                    stack.push(i);
                    break;
                } else {
                    stack.pop();
                    ans = ans.max(match stack.last() {
                        None => heights[j] * i as i32,
                        Some(&k) => heights[j] * (i - k - 1) as i32,
                    })
                }
            }
            if stack.is_empty() {
                stack.push(i);
            }
        }
        ans
    }
}
