fn main() {
    let tests = [
        vec![vec![1, 2], vec![1, 3], vec![2, 3]],
        vec![vec![1, 2], vec![2, 3], vec![3, 4], vec![1, 4], vec![1, 5]],
    ];
    let answers = vec![vec![2, 3], vec![1, 4]];
    for (edges, expected_answer) in tests.into_iter().zip(answers) {
        let answer = Solution::find_redundant_connection(edges);
        assert_eq!(expected_answer, answer);
    }
}
struct Solution;
impl Solution {
    pub fn find(parent: &mut Vec<usize>, x: usize) -> usize {
        if parent[x] == x {
            x
        } else {
            parent[x] = Self::find(parent, parent[x]);
            parent[x]
        }
    }
    pub fn union(parent: &mut Vec<usize>, x: usize, y: usize) -> bool {
        let px = Self::find(parent, x);
        let py = Self::find(parent, y);
        if px == py {
            false
        } else {
            parent[px] = py;
            true
        }
    }
    pub fn find_redundant_connection(edges: Vec<Vec<i32>>) -> Vec<i32> {
        let mut parent = (0..=edges.len()).collect::<Vec<usize>>();
        for edge in edges.iter() {
            if !Self::union(&mut parent, edge[0] as usize, edge[1] as usize) {
                return edge.clone();
            }
        }
        unreachable!("No redundant connection found");
    }
}
