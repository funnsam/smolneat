mod mutate;

#[derive(Debug, Clone)]
pub struct NeatNetwork {
    nodes: Vec<Node>,
    connections: Vec<Connection>,
    topology: Vec<usize>,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct Node {
    pub typ: NodeType,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum NodeType {
    Input,
    Output,
    Hidden
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Connection {
    pub in_node: usize,
    pub out_node: usize,
    pub enabled: bool,
    pub weight: f32,
    pub innov_id: u32,
}

impl NeatNetwork {
    fn update_topology(&mut self) {
        self.topology.clear();

        while self.topology.len() < self.nodes.len() {
            for i in 0..self.nodes.len() {
                if self.topology.contains(&i) { continue };

                if self.connections.iter()
                    .filter(|c| c.out_node == i)
                    .filter(|c| !self.topology.contains(&c.in_node))
                    .next()
                    .is_none()
                {
                    self.topology.push(i);
                }
            }
        }
    }
}
