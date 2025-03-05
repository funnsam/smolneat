use super::NodeType;

impl NodeType {
    pub fn activate<ActIn, ActHid, ActOut>(&self, input: f32, act_in: ActIn, act_hid: ActHid, act_out: ActOut) -> f32 where
        ActIn: Fn(f32) -> f32,
        ActHid: Fn(f32) -> f32,
        ActOut: Fn(f32) -> f32,
    {
        match self {
            Self::Input => act_in(input),
            Self::Hidden => act_hid(input),
            Self::Output => act_out(input),
        }
    }
}

pub fn ident(x: f32) -> f32 { x }
pub fn relu(x: f32) -> f32 { x.max(0.0) }
