fn main() {
    let tests = [
        (vec![8, 10], vec![2, 2, 3, 1, 8, 7, 4, 5]),
        (vec![10, 20], vec![2, 3, 1, 2, 5, 8, 4, 3]),
        (vec![1, 3, 5], vec![8, 7, 6, 5, 4, 3, 2, 1, 1, 1, 1, 1]),
    ];
    let answers = [16, 23, 9];
    for (test, expected_answer) in tests.into_iter().zip(answers) {
        let answer = Solution::min_processing_time(test.0, test.1);
        assert_eq!(answer, expected_answer);
    }
}
struct Solution;
impl Solution {
    pub fn min_processing_time(processor_time: Vec<i32>, tasks: Vec<i32>) -> i32 {
        let mut times = processor_time;
        let mut tasks = tasks;
        times.sort();
        tasks.sort();
        tasks.reverse();
        let mut ans = 0;
        for (&time, task) in times.iter().zip(tasks.chunks(4)) {
            ans = ans.max(time + task.iter().max().unwrap());
        }
        ans
    }
}
