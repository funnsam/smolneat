use fastrand::Rng;

use crate::network::Connection;

use super::NeatNetwork;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum MutationKind {
    AddConnection,
    SplitConnection,
    ChangeWeight,
    None,
}

impl MutationKind {
    pub const ALL: &[Self] = &[
        Self::AddConnection,
        Self::SplitConnection,
        Self::ChangeWeight,
        Self::None,
    ];
}

impl NeatNetwork {
    pub fn mutate(&mut self, rng: &mut Rng) {
        use MutationKind as M;

        match rng.choice(M::ALL).unwrap() {
            M::AddConnection => {
                let in_t = rng.usize(..self.topology.len() - 1);
                let out_t = rng.usize(in_t + 1..self.topology.len());

                let in_node = self.topology[in_t];
                let out_node = self.topology[out_t];

                if !self.connections.iter().any(|c| c.in_node == in_node && c.out_node == out_node) {
                    self.connections.push(Connection {
                        in_node,
                        out_node,
                        enabled: true,
                        weight: rng.f32(),
                        // TODO:
                        innov_id: 0,
                    })
                }
            },
            M::SplitConnection => todo!(),
            M::ChangeWeight => {
                if let Some(c) = rng.choice(self.connections.iter_mut()) {
                    c.weight += rng.f32() - 0.5;
                }
            }
            M::None => {},
        }
    }
}
