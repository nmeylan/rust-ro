#[derive(Clone, Copy, Debug)]
pub struct Attack {
    pub target: u32,
    pub repeat: bool,
    pub last_attack_tick: u128,
    pub last_attack_motion: u32
}

#[derive(Debug, PartialEq)]
pub struct Damage {
    pub target_id: u32,
    pub attacker_id: u32,
    pub damage: u32,
    pub attacked_at: u128,
}