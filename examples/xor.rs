use smolneat::network::{activation::{ident, relu}, *};
use fastrand::Rng;

fn main() {
    let mut rng = Rng::new();

    let mut agents = (0..1000)
        .map(|_| {
            let mut net = NeatNetwork::new_empty(2, 1);
            net.mutate_kind(mutate::MutationKind::AddConnection, &mut rng);
            net
        })
        .collect::<Vec<_>>();

    loop {
        let mut loss = (0..agents.len()).map(|i| (i, 0.0)).collect::<Vec<_>>();

        for (i, a) in agents.iter().enumerate() {
            const INPUT: [[f32; 2]; 4] = [
                [0.0, 0.0],
                [0.0, 1.0],
                [1.0, 0.0],
                [1.0, 1.0],
            ];
            const OUTPUT: [f32; 4] = [0.0, 1.0, 1.0, 0.0];

            for (j, input) in INPUT.into_iter().enumerate() {
                let res = evaluate(&*a, input);
                let exp = OUTPUT[j];
                loss[i].1 += (exp - res).powi(2);
            }
        }

        loss.sort_unstable_by(|i, j| i.1.partial_cmp(&j.1).unwrap_or(core::cmp::Ordering::Equal));

        println!("{}, {:?}", loss[0].1, agents[loss[0].0]);

        let mut new_agents = Vec::with_capacity(agents.len());
        let split = agents.len() * 3 / 10;

        for (i, _) in loss.iter().take(split) {
            new_agents.push(agents[*i].clone());
        }

        for _ in split..agents.len() {
            let mut old = rng.choice(agents.iter()).unwrap().clone();
            old.mutate(&mut rng);
            new_agents.push(old);
        }
    }
}

fn evaluate(agent: &NeatNetwork, input: [f32; 2]) -> f32 {
    let mut output = [0.0];
    agent.evaluate(&input, &mut output, ident, relu, relu);
    output[0]
}
