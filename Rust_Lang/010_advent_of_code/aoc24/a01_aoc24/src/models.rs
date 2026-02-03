use std::cell::Cell;

#[derive(Clone, Debug)]
pub struct Combat {
    pub army1: Army,
    pub army2: Army,
}

#[derive(Clone, Debug)]
pub struct Plan<'g> {
    pub attacker: &'g Group,
    pub victim: &'g Group,
}

#[derive(Clone, Debug)]
pub struct Army {
    pub name: String,
    pub groups: Vec<Group>,
}

#[derive(Clone, Debug)]
pub struct Group {
    pub army: String,
    pub id: u64,
    pub units: Cell<u64>,
    pub unit_hp: u64,
    pub initiative: u64,
    pub attack: Attack,
    pub weaknesses: Vec<AttackKind>,
    pub immunities: Vec<AttackKind>,
}

#[derive(Clone, Debug)]
pub struct Attack {
    pub kind: AttackKind,
    pub damage: u64,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub enum AttackKind {
    Radiation,
    Cold,
    Fire,
    Slashing,
    Bludgeoning,
}
