use crate::models::{Army, Attack, AttackKind, Combat, Group, Plan};

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

    fn winner(&self) -> Option<&Army> {
        assert!(self.army1.is_alive() || self.army2.is_alive());
        if !self.army1.is_alive() {
            Some(&self.army2)
        } else if !self.army2.is_alive() {
            Some(&self.army1)
        } else {
            None
        }
    }

    fn target_selection(&self) -> Vec<Plan<'_>> {
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

impl Army {
    pub fn is_alive(&self) -> bool {
        self.groups.iter().any(|g| g.is_alive())
    }

    pub fn total_live_units(&self) -> u64 {
        self.groups.iter().map(|g| g.units.get()).sum()
    }

    pub fn boost(&mut self, amount: u64) {
        for g in self.groups.iter_mut() {
            g.attack.damage += amount;
        }
    }

    fn target_selection<'a>(&'a self, enemy: &'a Army) -> Vec<Plan<'a>> {
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

    fn target_selection_order(&self) -> Vec<&Group> {
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

    fn alive_groups(&self) -> Vec<&Group> {
        self.groups.iter().filter(|g| g.is_alive()).collect()
    }
}

impl Group {
    pub fn is_alive(&self) -> bool {
        self.units.get() > 0
    }

    pub fn effective_power(&self) -> u64 {
        self.units.get() * self.attack.damage
    }

    pub fn absorb(&self, damage: u64) -> u64 {
        let units_lost = damage / self.unit_hp;
        let old = self.units.get();
        self.units.set(old.saturating_sub(units_lost));
        old - self.units.get()
    }

    fn choose_victim(&self, candidates: &[&Group]) -> Option<usize> {
        let mut choice = None;
        for (i, &candidate) in candidates.iter().enumerate() {
            let damage = self.attack_damage(candidate);
            if damage == 0 {
                continue;
            }
            if choice.is_none() {
                choice = Some(i);
                continue;
            }
            let cur = choice.unwrap();
            let cur_damage = self.attack_damage(candidates[cur]);
            if damage < cur_damage {
                continue;
            } else if damage > cur_damage {
                choice = Some(i);
                continue;
            }

            let epower = candidate.effective_power();
            let cur_epower = candidates[cur].effective_power();
            if epower < cur_epower {
                continue;
            } else if epower > cur_epower {
                choice = Some(i);
                continue;
            }

            let init = candidate.initiative;
            let cur_init = candidates[cur].initiative;
            assert!(init != cur_init);
            if init > cur_init {
                choice = Some(i);
            }
        }
        choice
    }

    fn attack_damage(&self, group: &Group) -> u64 {
        if group.is_immune(&self.attack.kind) {
            return 0;
        }
        let mut damage = self.effective_power();
        if group.is_weak(&self.attack.kind) {
            damage *= 2;
        }
        damage
    }

    fn is_immune(&self, kind: &AttackKind) -> bool {
        self.immunities.contains(kind)
    }

    fn is_weak(&self, kind: &AttackKind) -> bool {
        self.weaknesses.contains(kind)
    }
}

impl Attack {
    pub fn new(kind: AttackKind, damage: u64) -> Attack {
        Attack { kind, damage }
    }
}
