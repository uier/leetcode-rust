fn main() {
    let tests = [("ab#c", "ad#c"), ("ab##", "c#d#"), ("a#c", "b")];
    let answers = [true, true, false];
    for (test, expected_answer) in tests.into_iter().zip(answers) {
        let answer = Solution::backspace_compare(test.0.to_string(), test.1.to_string());
        assert_eq!(answer, expected_answer);
    }
}
struct Solution;
struct MyIter<T> {
    s: T,
}
impl<T: Iterator<Item = char>> MyIter<T> {
    fn new(s: T) -> Self {
        Self { s }
    }
}
impl<T: Iterator<Item = char>> Iterator for MyIter<T> {
    type Item = char;
    fn next(&mut self) -> Option<Self::Item> {
        let mut skip = 0;
        while let Some(c) = self.s.next() {
            if c == '#' {
                skip += 1;
            } else if skip > 0 {
                skip -= 1;
            } else {
                return Some(c);
            }
        }
        None
    }
}
impl Solution {
    pub fn backspace_compare(s: String, t: String) -> bool {
        let mut s = MyIter::new(s.chars().rev());
        let mut t = MyIter::new(t.chars().rev());
        loop {
            match (s.next(), t.next()) {
                (None, None) => break true,
                (si, ti) if si != ti => break false,
                _ => continue,
            }
        }
    }
}
