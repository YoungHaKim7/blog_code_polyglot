use crate::models::{Army, Attack, AttackKind, Group, Plan};

impl Attack {
    pub fn new(kind: AttackKind, damage: u64) -> Attack {
        Attack { kind, damage }
    }
}

impl Army {
    pub fn is_alive(&self) -> bool {
        self.groups.iter().any(|g| g.is_alive())
    }

    pub fn total_live_units(&self) -> u64 {
        self.groups.iter().map(|g| g.units.get()).sum()
    }

    pub fn target_selection<'a>(&'a self, enemy: &'a Army) -> Vec<Plan<'a>> {
        let mut plans = vec![];
        let mut candidates: Vec<&Group> = enemy.alive_groups();
        for g in self.target_selection_order() {
            if let Some(i) = g.choose_victim(&candidates) {
                plans.push(Plan {
                    attacker: g,
                    victim: candidates[i],
                });
                candidates.swap_remove(i);
            }
        }
        plans
    }

    pub fn target_selection_order(&self) -> Vec<&Group> {
        let mut groups = self.alive_groups();
        groups.sort_by(|g1, g2| {
            let power1 = g1.effective_power();
            let power2 = g2.effective_power();
            if power1 != power2 {
                power1.cmp(&power2).reverse()
            } else {
                g1.initiative.cmp(&g2.initiative).reverse()
            }
        });
        groups
    }

    pub fn alive_groups(&self) -> Vec<&Group> {
        self.groups.iter().filter(|g| g.is_alive()).collect()
    }

    pub fn boost(&mut self, amount: u64) {
        for g in self.groups.iter_mut() {
            g.attack.damage += amount;
        }
    }
}
