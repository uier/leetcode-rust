fn main() {
    let tests = [
        (3, vec![vec![1, 3], vec![2, 3]], vec![3, 2, 5]),
        (
            5,
            vec![vec![1, 5], vec![2, 5], vec![3, 5], vec![3, 4], vec![4, 5]],
            vec![1, 2, 3, 4, 5],
        ),
    ];
    let answers = [8, 12];
    for (test, expected_answer) in tests.into_iter().zip(answers) {
        let answer = Solution::minimum_time(test.0, test.1, test.2);
        assert_eq!(answer, expected_answer, "{}", test.0);
    }
}
struct Solution;
use std::collections::VecDeque;
use std::iter::FromIterator;
impl Solution {
    pub fn minimum_time(n: i32, relations: Vec<Vec<i32>>, time: Vec<i32>) -> i32 {
        let mut next_courses: Vec<Vec<usize>> = vec![vec![]; (n + 1) as usize];
        let mut in_edges: Vec<i32> = vec![0; (n + 1) as usize];
        let mut complete_time: Vec<i32> = time.clone();
        for course_pair in relations {
            next_courses[course_pair[0_usize] as usize].push(course_pair[1] as usize);
            in_edges[course_pair[1] as usize] += 1;
        }
        let mut queue = VecDeque::from_iter(
            in_edges
                .iter()
                .enumerate()
                .skip(1)
                .filter(|&(_, in_edge)| *in_edge == 0)
                .map(|(i, _)| i),
        );
        // println!("{:?}", next_courses);
        while !queue.is_empty() {
            let u = queue.pop_front().unwrap();
            for &course in &next_courses[u] {
                // println!("{} {} {}", u, course, course - 1);
                complete_time[course - 1] =
                    complete_time[course - 1].max(complete_time[u - 1] + time[course - 1]);
                in_edges[course] -= 1;
                if in_edges[course] == 0 {
                    queue.push_back(course);
                }
            }
        }
        complete_time.into_iter().max().unwrap()
    }
}
