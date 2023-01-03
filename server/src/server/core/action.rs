#[derive(Clone, Copy, Debug)]
pub struct Attack {
    pub target: u32,
    pub repeat: bool,
    pub last_attack_tick: u128
}