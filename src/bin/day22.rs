use std::collections::VecDeque;

#[derive(Clone, Copy, Debug)]
struct GameState {
    boss_hp: isize,
    boss_damage: usize,
    hard: bool,
    hp: isize,
    mana: usize,
    mana_spent: usize,
    shield_turns: usize,
    poison_turns: usize,
    recharge_turns: usize,
}

impl GameState {
    fn new(boss_hp: isize, boss_damage: usize, hard: bool) -> Self {
        assert!(boss_damage > 7); // so we don't have to complicate armor/shield too much
        Self {
            boss_hp,
            boss_damage,
            hard,
            hp: if hard { 49 } else { 50 },
            mana: 500,
            mana_spent: 0,
            shield_turns: 0,
            poison_turns: 0,
            recharge_turns: 0,
        }
    }

    fn mana_spent_to_win(&self) -> Option<usize> {
        if self.hp <= 0 {
            Some(usize::MAX)
        } else if self.boss_hp <= 0 {
            Some(self.mana_spent)
        } else {
            None
        }
    }

    fn apply_effects(&mut self) {
        if self.shield_turns > 0 {
            self.shield_turns -= 1;
        }
        if self.poison_turns > 0 {
            self.boss_hp -= 3;
            self.poison_turns -= 1;
        }
        if self.recharge_turns > 0 {
            self.mana += 101;
            self.recharge_turns -= 1;
        }
    }

    fn apply_post_cast_actions(&mut self) {
        if self.boss_hp > 0 {
            self.apply_effects();

            if self.boss_hp > 0 {
                let damage = self.boss_damage - if self.shield_turns > 0 { 7 } else { 0 };
                self.hp -= damage as isize;
                if self.hard {
                    self.hp -= 1;
                }
                self.apply_effects();
            }
        }
    }

    fn spend_mana(&mut self, mana: usize) -> bool {
        if self.mana >= mana {
            self.mana -= mana;
            self.mana_spent += mana;
            true
        } else {
            false
        }
    }

    fn cast_magic_missle(mut self) -> Option<Self> {
        if self.spend_mana(53) {
            self.boss_hp -= 4;
            self.apply_post_cast_actions();
            Some(self)
        } else {
            None
        }
    }

    fn cast_drain(mut self) -> Option<Self> {
        if self.spend_mana(73) {
            self.boss_hp -= 2;
            self.hp += 2;
            self.apply_post_cast_actions();
            Some(self)
        } else {
            None
        }
    }

    fn cast_shield(mut self) -> Option<Self> {
        if self.shield_turns == 0 && self.spend_mana(113) {
            self.shield_turns = 6;
            self.apply_post_cast_actions();
            Some(self)
        } else {
            None
        }
    }

    fn cast_poison(mut self) -> Option<Self> {
        if self.poison_turns == 0 && self.spend_mana(173) {
            self.poison_turns = 6;
            self.apply_post_cast_actions();
            Some(self)
        } else {
            None
        }
    }

    fn cast_recharge(mut self) -> Option<Self> {
        if self.recharge_turns == 0 && self.spend_mana(229) {
            self.recharge_turns = 5;
            self.apply_post_cast_actions();
            Some(self)
        } else {
            None
        }
    }

    fn after_possible_moves(self) -> impl Iterator<Item = Self> {
        [
            Self::cast_magic_missle,
            Self::cast_drain,
            Self::cast_shield,
            Self::cast_poison,
            Self::cast_recharge,
        ]
        .into_iter()
        .filter_map(move |action| action(self))
    }
}

fn find_least_mana_spent(boss_hp: isize, boss_damage: usize, hard: bool) -> usize {
    let mut least_mana_spent = usize::MAX;
    let mut queue = VecDeque::new();
    queue.push_back(GameState::new(boss_hp, boss_damage, hard));
    while let Some(state) = queue.pop_front() {
        for new_state in state.after_possible_moves() {
            if let Some(mana_spent) = new_state.mana_spent_to_win() {
                if mana_spent < least_mana_spent {
                    least_mana_spent = mana_spent;
                }
            } else if state.mana_spent < least_mana_spent {
                queue.push_back(new_state);
            }
        }
    }
    least_mana_spent
}

fn main() {
    println!("Part 1: {}", find_least_mana_spent(55, 8, false));
    println!("Part 2: {}", find_least_mana_spent(55, 8, true));
}
