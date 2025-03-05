use smolneat::network::*;
use fastrand::Rng;

fn main() {
    let mut rng = Rng::new();

    let mut agents = (0..1000)
        .map(|_| {
            let mut net = NeatNetwork::new_empty(4, 1);
            net.mutate_kind(mutate::MutationKind::AddConnection, &mut rng);
            net
        })
        .collect::<Vec<_>>();

    // LIGHT:
    // inputs:
    //   - position
    //   - direction x
    //   - direction y
    //   - angular velocity
    // outputs:
    //   - cart speed
}

struct World {
    cart_pos: f32,
    pendulum_pos: (f32, f32),
    angular_vel: f32,
}

impl World {
    pub fn step(&mut self, dt: f32) {
    }
}
