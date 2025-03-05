use fastrand::Rng;

use crate::network::{Connection, Node, NodeType};

use super::NeatNetwork;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum MutationKind {
    AddConnection,
    SplitConnection,
    ChangeWeight,
}

impl MutationKind {
    pub const ALL: &[Self] = &[
        Self::AddConnection,
        Self::SplitConnection,
        Self::ChangeWeight,
    ];
}

impl NeatNetwork {
    pub fn mutate(&mut self, rng: &mut Rng) {
        self.mutate_kind(*rng.choice(MutationKind::ALL).unwrap(), rng);
    }

    pub fn mutate_kind(&mut self, kind: MutationKind, rng: &mut Rng) {
        use MutationKind as M;

        match kind {
            M::AddConnection => {
                let in_t = rng.usize(..self.topology.len() - 1);
                let out_t = rng.usize(in_t + 1..self.topology.len());

                let in_node = self.topology[in_t];
                let out_node = self.topology[out_t];

                if !self.connections[&out_node].iter().any(|c| c.in_node == in_node) {
                    self.connections.get_mut(&out_node).unwrap().push(Connection {
                        in_node,
                        enabled: true,
                        weight: rng.f32(),
                        // TODO:
                        innov_id: 0,
                    });
                    self.update_topology();
                }
            },
            M::SplitConnection => {
                if let Some((out_node, c)) = rng.choice(self.connections.iter_mut()) {
                    if let Some(c) = rng.choice(c.iter_mut()) {
                        if c.enabled {
                            c.enabled = false;

                            let mid_node = self.nodes.len();
                            self.nodes.push(Node {
                                typ: NodeType::Hidden,
                            });

                            let in_node = c.in_node;
                            let out_node = *out_node;

                            self.connections.insert(mid_node, vec![Connection {
                                in_node,
                                enabled: true,
                                weight: 0.2 * (rng.f32() - 0.5),
                                // TODO:
                                innov_id: 0,
                            }]);

                            self.connections.get_mut(&out_node).unwrap().push(Connection {
                                in_node: mid_node,
                                enabled: true,
                                weight: 0.2 * (rng.f32() - 0.5),
                                // TODO:
                                innov_id: 0,
                            });

                            self.update_topology();
                        }
                    }
                }
            },
            M::ChangeWeight => {
                if let Some((_, c)) = rng.choice(self.connections.iter_mut()) {
                    if let Some(c) = rng.choice(c.iter_mut()) {
                        if c.enabled {
                            c.weight += rng.f32() - 0.5;
                        }
                    }
                }
            },
        }
    }
}
