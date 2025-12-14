pub struct Graph<T> {
    nodes: Vec<Node<T>>,
}

struct Node<T> {
    data: T,
    edges: Vec<usize>,  // Индексы соседних узлов
}

impl<T> Graph<T> {
    pub fn new() -> Self {
        Graph { nodes: Vec::new() }
    }

    pub fn add_node(&mut self, data: T) -> usize {
        let index = self.nodes.len();
        self.nodes.push(Node {
            data,
            edges: Vec::new(),
        });
        index
    }

    pub fn add_edge(&mut self, from: usize, to: usize) {
        if from < self.nodes.len() && to < self.nodes.len() {
            self.nodes[from].edges.push(to);
        }
    }

    pub fn neighbors(&self, node: usize) -> Option<&[usize]> {
        self.nodes.get(node).map(|n| n.edges.as_slice())
    }

    pub fn get_data(&self, node: usize) -> Option<&T> {
        self.nodes.get(node).map(|n| &n.data)
    }
}

fn main() {
    let mut graph = Graph::new();

    let a = graph.add_node("A");
    let b = graph.add_node("B");
    let c = graph.add_node("C");

    graph.add_edge(a, b);
    graph.add_edge(b, c);
    graph.add_edge(a, c);

    println!("Neighbors of A: {:?}", graph.neighbors(a)); // [1, 2]

    // DFS
    fn dfs<T: std::fmt::Debug>(
        graph: &Graph<T>,
        node: usize,
        visited: &mut Vec<bool>,
    ) {
        visited[node] = true;
        println!("Visiting: {:?}", graph.get_data(node));

        if let Some(neighbors) = graph.neighbors(node) {
            for &neighbor in neighbors {
                if !visited[neighbor] {
                    dfs(graph, neighbor, visited);
                }
            }
        }
    }

    let mut visited = vec![false; 3];
    dfs(&graph, a, &mut visited);
}
