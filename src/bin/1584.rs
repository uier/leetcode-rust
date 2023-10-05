fn main() {
    let tests = [
        vec![vec![0, 0], vec![2, 2], vec![3, 10], vec![5, 2], vec![7, 0]],
        vec![vec![3, 12], vec![-2, 5], vec![-4, 1]],
    ];
    let answers = [20, 18];
    for (points, expected_answer) in tests.into_iter().zip(answers) {
        let answer = Solution::min_cost_connect_points(points);
        assert_eq!(expected_answer, answer);
    }
}
struct Solution;
impl Solution {
    pub fn get_dist(x: &[i32], y: &[i32]) -> i32 {
        (x[0] - y[0]).abs() + (x[1] - y[1]).abs()
    }
    pub fn find(parent: &mut Vec<usize>, x: usize) -> usize {
        if parent[x] == x {
            x
        } else {
            parent[x] = Self::find(parent, parent[x]);
            parent[x]
        }
    }
    pub fn min_cost_connect_points(points: Vec<Vec<i32>>) -> i32 {
        let n = points.len();
        let mut edges: Vec<(usize, usize, i32)> = vec![];
        for i in 0..n {
            for j in 0..i {
                edges.push((i, j, Self::get_dist(&points[i], &points[j])));
            }
        }
        let mut parent: Vec<usize> = (0..n).collect();
        let mut ans = 0;
        let mut cnt = 0;
        edges.sort_by_key(|x| x.2);
        for (u, v, w) in edges {
            let pu = Self::find(&mut parent, u);
            let pv = Self::find(&mut parent, v);
            if pu != pv {
                parent[pu] = pv;
                ans += w;
                cnt += 1;
                if cnt == n - 1 {
                    break;
                }
            }
        }
        ans
    }
}
