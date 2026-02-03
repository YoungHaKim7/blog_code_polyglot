use crate::models::{Army, Combat, Plan};

impl Combat {
    pub fn fight_to_end(&self) -> &Army {
        loop {
            if let Some(winner) = self.fight() {
                return winner;
            }
        }
    }

    pub fn fight(&self) -> Option<&Army> {
        for plan in self.target_selection() {
            if !plan.attacker.is_alive() {
                continue;
            }

            let damage = plan.attacker.attack_damage(plan.victim);
            plan.victim.absorb(damage);
        }
        self.winner()
    }

    pub fn winner(&self) -> Option<&Army> {
        assert!(self.army1.is_alive() || self.army2.is_alive());
        if !self.army1.is_alive() {
            Some(&self.army2)
        } else if !self.army2.is_alive() {
            Some(&self.army1)
        } else {
            None
        }
    }

    pub fn target_selection(&self) -> Vec<Plan<'_>> {
        let mut plans = self.army1.target_selection(&self.army2);
        plans.extend(self.army2.target_selection(&self.army1));
        plans.sort_by(|plan1, plan2| {
            plan1
                .attacker
                .initiative
                .cmp(&plan2.attacker.initiative)
                .reverse()
        });
        plans
    }
}
