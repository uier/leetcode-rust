fn main() {
    let tests = [
        vec![42, 11, 1, 97],
        vec![13, 10, 35, 24, 76],
        vec![1],
        vec![8, 5],
    ];
    let answers = [2, 4, 0, 1];
    for (test, expected_answer) in tests.into_iter().zip(answers) {
        let answer = Solution::count_nice_pairs(test);
        assert_eq!(answer, expected_answer);
    }
}
struct Solution;
impl Solution {
    fn rev(n: i32) -> i32 {
        n.to_string()
            .chars()
            .rev()
            .collect::<String>()
            .parse::<i32>()
            .unwrap()
    }
    pub fn count_nice_pairs(nums: Vec<i32>) -> i32 {
        let revs = nums.iter().map(|&x| Self::rev(x)).collect::<Vec<i32>>();
        let mut diff = nums
            .iter()
            .zip(revs)
            .map(|(&a, b)| a - b)
            .collect::<Vec<i32>>();
        diff.sort();
        let mut ans = 0i64;
        const MOD: i64 = 1000000007;
        let mut cnt = 1i64;
        for (i, &x) in diff.iter().enumerate().skip(1) {
            if x == diff[i - 1] {
                cnt += 1;
            } else {
                ans = (ans + (cnt * (cnt - 1) / 2 % MOD) ) % MOD;
                cnt = 1;
            }
        }
        ans = (ans + (cnt * (cnt - 1) / 2 % MOD)) % MOD;
        ans as i32
    }
}
