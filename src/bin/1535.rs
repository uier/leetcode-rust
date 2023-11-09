fn main() {
    let tests = [
        (vec![2, 1, 3, 5, 4, 6, 7], 2),
        (vec![3, 2, 1], 10),
        (vec![2, 1, 3, 5, 4, 6, 7], 6),
        (vec![2, 1, 3, 5, 4, 6, 7], 10),
    ];
    let answers = [5, 3, 7, 7];
    for ((nums, k), expected_answer) in tests.into_iter().zip(answers) {
        let answer = Solution::get_winner(nums, k);
        assert_eq!(expected_answer, answer);
    }
}
struct Solution;
impl Solution {
    pub fn get_winner(arr: Vec<i32>, k: i32) -> i32 {
        let mut winner = arr[0];
        let mut win_count = 0;
        for &opponent in arr.iter().skip(1) {
            if opponent > winner {
                winner = opponent;
                win_count = 1;
            } else {
                win_count += 1;
            }
            if win_count == k {
                break;
            }
        }
        winner
    }
}
