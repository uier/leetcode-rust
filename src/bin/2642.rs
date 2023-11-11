use leetcode_prelude::leetcode_test;

fn main() {
    leetcode_test!(
      ["Graph", "shortestPath", "shortestPath", "addEdge", "shortestPath"]
      [[4, [[0, 2, 5], [0, 1, 2], [1, 2, 1], [3, 0, 3]]], [3, 2], [0, 3], [[1, 3, 4]], [0, 3]]
      [null, 6, -1, null, 6]
    );
}
struct Graph {
    n: usize,
    g: Vec<Vec<i32>>,
}
impl Graph {
    #![allow(clippy::needless_range_loop)]
    fn new(n: i32, edges: Vec<Vec<i32>>) -> Self {
        const INF: i32 = 1e9 as i32;
        let mut g = vec![vec![INF; n as usize]; n as usize];
        for edge in edges {
            let (u, v, w) = (edge[0], edge[1], edge[2]);
            g[u as usize][v as usize] = w;
        }
        for i in 0..n as usize {
            g[i][i] = 0;
        }
        for k in 0..n as usize {
            for i in 0..n as usize {
                for j in 0..n as usize {
                    g[i][j] = g[i][j].min(g[i][k] + g[k][j]);
                }
            }
        }
        Self { n: n as usize, g }
    }

    fn add_edge(&mut self, edge: Vec<i32>) {
        let (u, v, w) = (edge[0], edge[1], edge[2]);
        for i in 0..self.n {
            for j in 0..self.n {
                self.g[i][j] = self.g[i][j].min(self.g[i][u as usize] + w + self.g[v as usize][j]);
                self.g[i][j] = self.g[i][j].min(self.g[i][v as usize] + w + self.g[u as usize][j]);
            }
        }
    }

    fn shortest_path(&self, node1: i32, node2: i32) -> i32 {
        if self.g[node1 as usize][node2 as usize] == 1e9 as i32 {
            -1
        } else {
            self.g[node1 as usize][node2 as usize]
        }
    }
}
