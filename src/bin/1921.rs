fn main() {
    let tests = [
        (vec![1, 3, 4], vec![1, 1, 1]),
        (vec![1, 1, 2, 3], vec![1, 1, 1, 1]),
        (vec![3, 2, 4], vec![5, 3, 2]),
        (vec![3, 3, 3], vec![1, 1, 1]),
        (vec![3, 5, 7, 4, 5], vec![2, 3, 6, 3, 2]),
    ];
    let answers = [3, 1, 1, 3, 2];
    for (test, expected_answer) in tests.into_iter().zip(answers) {
        let answer = Solution::eliminate_maximum(test.0, test.1);
        assert_eq!(answer, expected_answer);
    }
}
struct Solution;
impl Solution {
    pub fn eliminate_maximum(dist: Vec<i32>, speed: Vec<i32>) -> i32 {
        let mut time_of_arrival: Vec<i32> = dist
            .into_iter()
            .zip(speed)
            .map(&|(d, s)| d / s + (d % s > 0) as i32)
            .collect();
        time_of_arrival.sort();
        let mut next_weapon_ready_time = 1;
        for (i, &toa) in time_of_arrival.iter().enumerate().skip(1) {
            if next_weapon_ready_time >= toa {
                return i as i32;
            }
            next_weapon_ready_time += 1;
        }
        time_of_arrival.len() as i32
    }
}
