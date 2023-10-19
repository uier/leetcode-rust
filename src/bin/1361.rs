fn main() {
    let tests = [
        (4, vec![1, -1, 3, -1], vec![2, -1, -1, -1]),
        (4, vec![1, -1, 3, -1], vec![2, 3, -1, -1]),
        (2, vec![1, 0], vec![-1, -1]),
        (3, vec![-1, -1, -1], vec![1, 2, 0]),
        (5, vec![-1, -1, -1, -1, -1], vec![1, 2, 0, 4, -1]),
        (4, vec![1, 0, 3, -1], vec![-1, -1, -1, -1]),
    ];
    let answers = [true, false, false, false, false];
    for (test, expected_answer) in tests.into_iter().zip(answers) {
        let answer = Solution::validate_binary_tree_nodes(test.0, test.1, test.2);
        assert_eq!(answer, expected_answer, "{}", test.0);
    }
}
struct Solution;
impl Solution {
    fn identify_parents(parent: &mut Vec<i32>, child: &Vec<i32>) -> bool {
        for (i, &c) in child.iter().enumerate() {
            if c != -1 {
                if parent[c as usize] != -1 && parent[c as usize] != i as i32 {
                    return false;
                }
                parent[c as usize] = i as i32
            }
        }
        true
    }
    fn dfs(root: i32, left_child: &Vec<i32>, right_child: &Vec<i32>, visited: &mut Vec<bool>) {
        if root == -1 {
            return;
        }
        visited[root as usize] = true;
        Self::dfs(left_child[root as usize], left_child, right_child, visited);
        Self::dfs(right_child[root as usize], left_child, right_child, visited);
    }
    pub fn validate_binary_tree_nodes(n: i32, left_child: Vec<i32>, right_child: Vec<i32>) -> bool {
        let mut parent = vec![-1; n as usize];
        let mut visited = vec![false; n as usize];
        if !Self::identify_parents(&mut parent, &left_child) {
            return false;
        }
        if !Self::identify_parents(&mut parent, &right_child) {
            return false;
        }
        let mut root: i32 = -1;
        for (i, &p) in parent.iter().enumerate() {
            if p != -1 {
                continue;
            }
            if root != -1 {
                return false;
            }
            root = i as i32;
        }
        if root == -1 {
            return false;
        }
        Self::dfs(root, &left_child, &right_child, &mut visited);
        visited.iter().all(|&x| x)
    }
}
