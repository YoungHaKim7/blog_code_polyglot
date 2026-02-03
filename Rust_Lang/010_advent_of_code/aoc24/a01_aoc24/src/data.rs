use std::cell::Cell;

use crate::models::{Attack, Army, Group};

impl Army {
    pub fn test1_immune() -> Army {
        use crate::models::AttackKind::*;

        Army {
            name: "immune".to_string(),
            groups: vec![
                Group {
                    army: "immune".to_string(),
                    id: 1,
                    units: Cell::new(17),
                    unit_hp: 5390,
                    initiative: 2,
                    attack: Attack::new(Fire, 4507),
                    weaknesses: vec![Radiation, Bludgeoning],
                    immunities: vec![],
                },
                Group {
                    army: "immune".to_string(),
                    id: 2,
                    units: Cell::new(989),
                    unit_hp: 1274,
                    initiative: 3,
                    attack: Attack::new(Slashing, 25),
                    weaknesses: vec![Bludgeoning, Slashing],
                    immunities: vec![Fire],
                },
            ],
        }
    }

    pub fn test1_infection() -> Army {
        use crate::models::AttackKind::*;

        Army {
            name: "infection".to_string(),
            groups: vec![
                Group {
                    army: "infection".to_string(),
                    id: 1,
                    units: Cell::new(801),
                    unit_hp: 4706,
                    initiative: 1,
                    attack: Attack::new(Bludgeoning, 116),
                    weaknesses: vec![Radiation],
                    immunities: vec![],
                },
                Group {
                    army: "infection".to_string(),
                    id: 2,
                    units: Cell::new(4485),
                    unit_hp: 2961,
                    initiative: 4,
                    attack: Attack::new(Slashing, 12),
                    weaknesses: vec![Fire, Cold],
                    immunities: vec![Radiation],
                },
            ],
        }
    }

    pub fn real_immune() -> Army {
        use crate::models::AttackKind::*;

        Army {
            name: "immune".to_string(),
            groups: vec![
                Group {
                    army: "immune".to_string(),
                    id: 1,
                    units: Cell::new(479),
                    unit_hp: 3393,
                    initiative: 8,
                    attack: Attack::new(Cold, 66),
                    weaknesses: vec![Radiation],
                    immunities: vec![],
                },
                Group {
                    army: "immune".to_string(),
                    id: 2,
                    units: Cell::new(2202),
                    unit_hp: 4950,
                    initiative: 2,
                    attack: Attack::new(Cold, 18),
                    weaknesses: vec![Fire],
                    immunities: vec![Slashing],
                },
                Group {
                    army: "immune".to_string(),
                    id: 3,
                    units: Cell::new(8132),
                    unit_hp: 9680,
                    initiative: 7,
                    attack: Attack::new(Radiation, 9),
                    weaknesses: vec![Bludgeoning, Fire],
                    immunities: vec![Slashing],
                },
                Group {
                    army: "immune".to_string(),
                    id: 4,
                    units: Cell::new(389),
                    unit_hp: 13983,
                    initiative: 13,
                    attack: Attack::new(Cold, 256),
                    weaknesses: vec![],
                    immunities: vec![Bludgeoning],
                },
                Group {
                    army: "immune".to_string(),
                    id: 5,
                    units: Cell::new(1827),
                    unit_hp: 5107,
                    initiative: 18,
                    attack: Attack::new(Slashing, 24),
                    weaknesses: vec![],
                    immunities: vec![],
                },
                Group {
                    army: "immune".to_string(),
                    id: 6,
                    units: Cell::new(7019),
                    unit_hp: 2261,
                    initiative: 16,
                    attack: Attack::new(Fire, 3),
                    weaknesses: vec![],
                    immunities: vec![Radiation, Slashing, Cold],
                },
                Group {
                    army: "immune".to_string(),
                    id: 7,
                    units: Cell::new(4736),
                    unit_hp: 8421,
                    initiative: 3,
                    attack: Attack::new(Slashing, 17),
                    weaknesses: vec![Cold],
                    immunities: vec![],
                },
                Group {
                    army: "immune".to_string(),
                    id: 8,
                    units: Cell::new(491),
                    unit_hp: 3518,
                    initiative: 1,
                    attack: Attack::new(Radiation, 65),
                    weaknesses: vec![Cold],
                    immunities: vec![Fire, Bludgeoning],
                },
                Group {
                    army: "immune".to_string(),
                    id: 9,
                    units: Cell::new(2309),
                    unit_hp: 7353,
                    initiative: 20,
                    attack: Attack::new(Bludgeoning, 31),
                    weaknesses: vec![],
                    immunities: vec![Radiation],
                },
                Group {
                    army: "immune".to_string(),
                    id: 10,
                    units: Cell::new(411),
                    unit_hp: 6375,
                    initiative: 14,
                    attack: Attack::new(Bludgeoning, 151),
                    weaknesses: vec![Cold, Fire],
                    immunities: vec![Slashing],
                },
            ],
        }
    }

    pub fn real_infection() -> Army {
        use crate::models::AttackKind::*;

        Army {
            name: "infection".to_string(),
            groups: vec![
                Group {
                    army: "infection".to_string(),
                    id: 1,
                    units: Cell::new(148),
                    unit_hp: 31914,
                    initiative: 4,
                    attack: Attack::new(Cold, 416),
                    weaknesses: vec![Bludgeoning],
                    immunities: vec![Radiation, Cold, Fire],
                },
                Group {
                    army: "infection".to_string(),
                    id: 2,
                    units: Cell::new(864),
                    unit_hp: 38189,
                    initiative: 6,
                    attack: Attack::new(Slashing, 72),
                    weaknesses: vec![],
                    immunities: vec![],
                },
                Group {
                    army: "infection".to_string(),
                    id: 3,
                    units: Cell::new(2981),
                    unit_hp: 7774,
                    initiative: 15,
                    attack: Attack::new(Fire, 4),
                    weaknesses: vec![],
                    immunities: vec![Bludgeoning, Cold],
                },
                Group {
                    army: "infection".to_string(),
                    id: 4,
                    units: Cell::new(5259),
                    unit_hp: 22892,
                    initiative: 5,
                    attack: Attack::new(Fire, 8),
                    weaknesses: vec![],
                    immunities: vec![],
                },
                Group {
                    army: "infection".to_string(),
                    id: 5,
                    units: Cell::new(318),
                    unit_hp: 16979,
                    initiative: 9,
                    attack: Attack::new(Bludgeoning, 106),
                    weaknesses: vec![Fire],
                    immunities: vec![],
                },
                Group {
                    army: "infection".to_string(),
                    id: 6,
                    units: Cell::new(5017),
                    unit_hp: 32175,
                    initiative: 17,
                    attack: Attack::new(Bludgeoning, 11),
                    weaknesses: vec![Slashing],
                    immunities: vec![Radiation],
                },
                Group {
                    army: "infection".to_string(),
                    id: 7,
                    units: Cell::new(4308),
                    unit_hp: 14994,
                    initiative: 10,
                    attack: Attack::new(Fire, 5),
                    weaknesses: vec![Slashing],
                    immunities: vec![Fire, Cold],
                },
                Group {
                    army: "infection".to_string(),
                    id: 8,
                    units: Cell::new(208),
                    unit_hp: 14322,
                    initiative: 19,
                    attack: Attack::new(Cold, 133),
                    weaknesses: vec![Radiation],
                    immunities: vec![],
                },
                Group {
                    army: "infection".to_string(),
                    id: 9,
                    units: Cell::new(3999),
                    unit_hp: 48994,
                    initiative: 11,
                    attack: Attack::new(Cold, 20),
                    weaknesses: vec![Cold, Slashing],
                    immunities: vec![],
                },
                Group {
                    army: "infection".to_string(),
                    id: 10,
                    units: Cell::new(1922),
                    unit_hp: 34406,
                    initiative: 12,
                    attack: Attack::new(Slashing, 35),
                    weaknesses: vec![Slashing],
                    immunities: vec![],
                },
            ],
        }
    }
}
