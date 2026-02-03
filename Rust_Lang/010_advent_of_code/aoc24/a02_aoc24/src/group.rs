use crate::models::{AttackKind, Group};

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

    pub fn choose_victim(&self, candidates: &[&Group]) -> Option<usize> {
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

    pub fn attack_damage(&self, group: &Group) -> u64 {
        if group.is_immune(&self.attack.kind) {
            return 0;
        }
        let mut damage = self.effective_power();
        if group.is_weak(&self.attack.kind) {
            damage *= 2;
        }
        damage
    }

    pub fn is_immune(&self, kind: &AttackKind) -> bool {
        self.immunities.contains(kind)
    }

    pub fn is_weak(&self, kind: &AttackKind) -> bool {
        self.weaknesses.contains(kind)
    }
}
