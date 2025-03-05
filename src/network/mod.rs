use std::collections::HashMap;

pub mod activation;
pub mod mutate;

#[derive(Debug, Clone)]
pub struct NeatNetwork {
    nodes: Vec<Node>,
    connections: HashMap<usize, Vec<Connection>>,
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

                if self.connections[&i].iter()
                    .filter(|c| !self.topology.contains(&c.in_node))
                    .next()
                    .is_none()
                {
                    self.topology.push(i);
                }
            }
        }
    }

    pub fn new_empty(in_nodes: usize, out_nodes: usize) -> Self {
        let mut connections = HashMap::new();

        for i in 0..in_nodes + out_nodes {
            connections.insert(i, Vec::new());
        }

        Self {
            nodes: (0..in_nodes + out_nodes).map(|i| if i < in_nodes {
                Node {
                    typ: NodeType::Input,
                }
            } else {
                Node {
                    typ: NodeType::Output,
                }
            }).collect(),
            connections,
            topology: (0..in_nodes + out_nodes).collect(),
        }
    }

    pub fn evaluate<ActIn, ActHid, ActOut>(&self, input: &[f32], output: &mut [f32], act_in: ActIn, act_hid: ActHid, act_out: ActOut) where
        ActIn: Fn(f32) -> f32,
        ActHid: Fn(f32) -> f32,
        ActOut: Fn(f32) -> f32,
    {
        let mut value = vec![0.0; self.nodes.len()];
        value[..input.len()].copy_from_slice(input);

        for n in self.topology.iter() {
            for c in self.connections[n].iter() {
                if c.enabled {
                    value[*n] += value[c.in_node] * c.weight;
                }
            }

            value[*n] = self.nodes[*n].typ.activate(value[*n], &act_in, &act_hid, &act_out);
        }

        println!("{value:?}");
        output.copy_from_slice(&value[input.len()..input.len() + output.len()]);
    }
}
