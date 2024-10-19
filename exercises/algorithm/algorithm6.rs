/*
	dfs
	This problem requires you to implement a basic DFS traversal
*/

use std::collections::HashSet;

// 定义一个图结构
struct Graph {
    adj: Vec<Vec<usize>>,
}

impl Graph {
    // 创建一个具有n个顶点的图
    fn new(n: usize) -> Self {
        Graph {
            adj: vec![vec![]; n],
        }
    }

    // 添加一条边到图中
    fn add_edge(&mut self, src: usize, dest: usize) {
        self.adj[src].push(dest);
        self.adj[dest].push(src);
    }

    // 深度优先搜索辅助函数
    fn dfs_util(&self, v: usize, visited: &mut HashSet<usize>, visit_order: &mut Vec<usize>) {
        visited.insert(v);
        visit_order.push(v);

        for &neighbor in &self.adj[v] {
            if !visited.contains(&neighbor) {
                self.dfs_util(neighbor, visited, visit_order);
            }
        }
    }

    // 执行深度优先搜索，并返回访问顺序
    fn dfs(&self, start: usize) -> Vec<usize> {
        let mut visited = HashSet::new();
        let mut visit_order = Vec::new();
        self.dfs_util(start, &mut visited, &mut visit_order);
        visit_order
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_dfs_simple() {
        let mut graph = Graph::new(3);
        graph.add_edge(0, 1);
        graph.add_edge(1, 2);
        let visit_order = graph.dfs(0);
        assert_eq!(visit_order, vec![0, 1, 2]);
    }

    #[test]
    fn test_dfs_with_cycle() {
        let mut graph = Graph::new(4);
        graph.add_edge(0, 1);
        graph.add_edge(0, 2);
        graph.add_edge(1, 2);
        graph.add_edge(2, 3);
        graph.add_edge(3, 3);
        let visit_order = graph.dfs(0);
        assert_eq!(visit_order, vec![0, 1, 2, 3]);
    }

    #[test]
    fn test_dfs_disconnected_graph() {
        let mut graph = Graph::new(5);
        graph.add_edge(0, 1);
        graph.add_edge(0, 2);
        graph.add_edge(3, 4);
        let visit_order = graph.dfs(0);
        assert_eq!(visit_order, vec![0, 1, 2]);

        let visit_order_disconnected = graph.dfs(3);
        assert_eq!(visit_order_disconnected, vec![3, 4]);
    }
}
