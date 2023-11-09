fn main() {
    let tests = [
        vec![0, 1, 2, 3, 4, 5, 6, 7, 8],
        vec![1024, 512, 256, 128, 64, 32, 16, 8, 4, 2, 1],
    ];
    let answers = [
        vec![0, 1, 2, 4, 8, 3, 5, 6, 7],
        vec![1, 2, 4, 8, 16, 32, 64, 128, 256, 512, 1024],
    ];
    for (test, expected_answer) in tests.into_iter().zip(answers) {
        let answer = Solution::sort_by_bits(test);
        assert_eq!(answer, expected_answer);
    }
}
struct Solution;
impl Solution {
    pub fn sort_by_bits(arr: Vec<i32>) -> Vec<i32> {
        let mut arr: Vec<(i32, i32)> = arr
            .iter()
            .map(|&x| {
                let mut bit = 0;
                let mut y = x;
                while y > 0 {
                    bit += y & 1;
                    y >>= 1;
                }
                (bit, x)
            })
            .collect();
        arr.sort_by(|&a, &b| match a.0.cmp(&b.0) {
            std::cmp::Ordering::Equal => a.1.cmp(&b.1),
            ord => ord,
        });
        arr.iter().map(|&(_, x)| x).collect()
    }
}
