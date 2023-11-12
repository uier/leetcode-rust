// #![feature(test)]

// #[cfg(test)]
// mod tests {
//     extern crate test;
//     use super::Solution;

//     #[test]
//     fn test_case1() {
//         let tests = [
//             (vec![vec![1, 2, 7], vec![3, 6, 7]], 1, 6),
//             (
//                 vec![
//                     vec![7, 12],
//                     vec![4, 5, 15],
//                     vec![6],
//                     vec![15, 19],
//                     vec![9, 12, 13],
//                 ],
//                 15,
//                 12,
//             ),
//         ];
//         let expected_answers = [2, -1];
//         for (test, expected) in tests.iter().zip(expected_answers) {
//             assert_eq!(
//                 Solution::num_buses_to_destination(test.0, test.1, test.2),
//                 expected,
//             );
//         }
//     }
// }
fn main() {
    let tests = [
        (vec![vec![1, 2, 7], vec![3, 6, 7]], 1, 6),
        (
            vec![
                vec![7, 12],
                vec![4, 5, 15],
                vec![6],
                vec![15, 19],
                vec![9, 12, 13],
            ],
            15,
            12,
        ),
    ];
    let expected_answers = [2, -1];
    for (test, expected) in tests.into_iter().zip(expected_answers) {
        assert_eq!(
            Solution::num_buses_to_destination(test.0, test.1, test.2),
            expected,
        );
    }
}
struct Solution;
use std::collections::{HashMap, VecDeque};
impl Solution {
    pub fn num_buses_to_destination(routes: Vec<Vec<i32>>, source: i32, target: i32) -> i32 {
        let mut graph = HashMap::<i32, Vec<usize>>::new();
        for (i, route) in routes.iter().enumerate() {
            for &stop in route {
                graph.entry(stop).or_default().push(i);
            }
        }
        let mut queue = VecDeque::new();
        let mut distance = HashMap::<i32, i32>::new();
        let mut visited = vec![false; routes.len()];
        distance.insert(source, 0);
        queue.push_back(source);
        while let Some(u) = queue.pop_front() {
            if u == target {
                return *distance.get(&u).unwrap();
            }
            for &route_i in graph.get(&u).unwrap() {
                if visited[route_i] {
                    continue;
                }
                visited[route_i] = true;
                for &v in &routes[route_i] {
                    if distance.contains_key(&v) {
                        continue;
                    }
                    if v == target {
                        return *distance.get(&u).unwrap() + 1;
                    }
                    distance.insert(v, *distance.get(&u).unwrap() + 1);
                    queue.push_back(v);
                }
            }
        }
        -1
    }
}

// impl Solution {
//     pub fn num_buses_to_destination(routes: Vec<Vec<i32>>, source: i32, target: i32) -> i32 {
//         if source == target {
//             return 0;
//         }
//         let n = routes.len();
//         let mut buses: Vec<Vec<i32>> = vec![vec![]; 1000000];
//         let mut graph: Vec<Vec<i32>> = vec![vec![]; n];
//         for (i, route) in routes.iter().enumerate() {
//             for &stop in route {
//                 buses[stop as usize].push(i as i32);
//             }
//         }
//         for intersect_buses in &buses {
//             for (i, &bus_i) in intersect_buses.iter().enumerate() {
//                 for &bus_j in intersect_buses.iter().skip(i + 1) {
//                     graph[bus_i as usize].push(bus_j);
//                     graph[bus_j as usize].push(bus_i);
//                 }
//             }
//         }
//         let mut queue = VecDeque::new();
//         let mut distance = vec![i32::MAX; n];
//         for &bus in &buses[source as usize] {
//             queue.push_back(bus);
//             distance[bus as usize] = 0;
//         }
//         while let Some(u) = queue.pop_front() {
//             if (&buses[target as usize]).contains(&u) {
//                 return distance[u as usize] + 1;
//             }
//             for &v in &graph[u as usize] {
//                 if distance[v as usize] < i32::MAX {
//                     continue;
//                 }
//                 distance[v as usize] = distance[u as usize] + 1;
//                 queue.push_back(v);
//             }
//         }
//         -1
//     }
// }
