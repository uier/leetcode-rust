fn main() {
    let tests = [
        ("leet2code3", 10),
        ("ha22", 5),
        ("a2345678999999999999999", 1),
        ("leetcode", 1),
        ("leetcode", 8),
        ("leet2code3", 1),
        ("leet2code3", 2),
        ("leet2code3", 3),
        ("leet2code3", 4),
        ("leet2code3", 5),
        ("leet2code3", 6),
        ("leet2code3", 7),
        ("leet2code3", 8),
        ("leet2code3", 9),
        ("leet2code3", 10),
        ("leet2code3", 11),
        ("leet2code3", 12),
        ("leet2code3", 13),
    ];
    let answers = [
        "o", "h", "a", "l", "e", "l", "e", "e", "t", "l", "e", "e", "t", "c", "o", "d", "e", "l",
    ];
    for ((s, k), expected_answer) in tests.into_iter().zip(answers) {
        let answer = Solution::decode_at_index(s.to_string(), k);
        assert_eq!(answer, expected_answer);
        println!("PASS: {:?} {}", s, k)
    }
}
struct Solution;
impl Solution {
    pub fn decode_at_index(s: String, k: i32) -> String {
        let mut k = (k - 1) as i64;
        let mut segments = Vec::<(String, i64)>::new();
        let mut segment = String::new();
        let mut total_len = 0_i64;
        for c in s.as_bytes() {
            if c.is_ascii_digit() {
                let digit = (c - b'0') as i64;
                total_len = (total_len + segment.len() as i64) * digit;
                segments.push((segment, digit));
                segment = String::new();
                if k < total_len {
                    break;
                }
            } else {
                segment.push(*c as char);
            }
        }
        if !segment.is_empty() {
            total_len += segment.len() as i64;
            segments.push((segment, 1));
        }
        for (segment, digit) in segments.into_iter().rev() {
            let pattern_len = total_len / digit;
            let prefix_len = pattern_len - segment.len() as i64;
            k %= pattern_len;
            if k >= prefix_len {
                return segment
                    .chars()
                    .nth((k - prefix_len) as usize)
                    .unwrap()
                    .to_string();
            }
            total_len = prefix_len;
        }
        unreachable!()
    }
}
