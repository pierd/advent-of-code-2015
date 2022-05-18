use std::mem;

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
struct Item {
    cost: usize,
    damage: usize,
    armor: usize,
}

const WEAPONS: [Item; 5] = [
    Item {
        cost: 8,
        damage: 4,
        armor: 0,
    },
    Item {
        cost: 10,
        damage: 5,
        armor: 0,
    },
    Item {
        cost: 25,
        damage: 6,
        armor: 0,
    },
    Item {
        cost: 40,
        damage: 7,
        armor: 0,
    },
    Item {
        cost: 74,
        damage: 8,
        armor: 0,
    },
];

const ARMORS: [Item; 6] = [
    // fake to make sure an armor is always used
    Item {
        cost: 0,
        damage: 0,
        armor: 0,
    },
    Item {
        cost: 13,
        damage: 0,
        armor: 1,
    },
    Item {
        cost: 31,
        damage: 0,
        armor: 2,
    },
    Item {
        cost: 53,
        damage: 0,
        armor: 3,
    },
    Item {
        cost: 75,
        damage: 0,
        armor: 4,
    },
    Item {
        cost: 102,
        damage: 0,
        armor: 5,
    },
];

const RINGS: [Item; 8] = [
    // fake so there are always 2 rings in use
    Item {
        cost: 0,
        damage: 0,
        armor: 0,
    },
    Item {
        cost: 0,
        damage: 0,
        armor: 0,
    },
    Item {
        cost: 25,
        damage: 1,
        armor: 0,
    },
    Item {
        cost: 50,
        damage: 2,
        armor: 0,
    },
    Item {
        cost: 100,
        damage: 3,
        armor: 0,
    },
    Item {
        cost: 20,
        damage: 0,
        armor: 1,
    },
    Item {
        cost: 40,
        damage: 0,
        armor: 2,
    },
    Item {
        cost: 80,
        damage: 0,
        armor: 3,
    },
];

#[derive(Clone, Copy, Debug)]
struct Character {
    is_player: bool,
    hp: usize,
    damage: usize,
    armor: usize,
}

impl Character {
    fn new_boss() -> Self {
        Self {
            is_player: false,
            hp: 104,
            damage: 8,
            armor: 1,
        }
    }

    fn new_player(items: &[Item]) -> Self {
        let damage = items.iter().map(|Item { damage, .. }| *damage).sum();
        let armor = items.iter().map(|Item { armor, .. }| *armor).sum();
        Self {
            is_player: true,
            hp: 100,
            damage,
            armor,
        }
    }

    fn survived_the_hit(&mut self, other: &Character) -> bool {
        let damage = if self.armor >= other.damage {
            1
        } else {
            other.damage - self.armor
        };
        if self.hp > damage {
            self.hp -= damage;
            true
        } else {
            self.hp = 0;
            false
        }
    }

    fn fight_the_boss(weapon: Item, armor: Item, ring1: Item, ring2: Item) -> (bool, usize) {
        let mut boss = Self::new_boss();
        let mut player = Self::new_player(&[weapon, armor, ring1, ring2]);
        Self::fight(&mut player, &mut boss);
        assert!(player.is_player);
        assert!(!boss.is_player);
        let win = player.hp > 0;
        let cost = [weapon, armor, ring1, ring2]
            .into_iter()
            .map(|Item { cost, .. }| cost)
            .sum();
        (win, cost)
    }

    fn fight<'a>(mut attacker: &'a mut Self, mut defender: &'a mut Self) {
        while defender.survived_the_hit(attacker) {
            mem::swap(&mut attacker, &mut defender);
        }
    }
}

fn check_all_items<F: FnMut((bool, usize))>(mut callback: F) {
    for weapon in WEAPONS {
        for armor in ARMORS {
            for (ring_idx, ring1) in RINGS[..RINGS.len() - 1].iter().enumerate() {
                for ring2 in RINGS[(ring_idx + 1)..RINGS.len()].iter() {
                    callback(Character::fight_the_boss(weapon, armor, *ring1, *ring2));
                }
            }
        }
    }
}

fn solve() -> (usize, usize) {
    let mut best_win = usize::MAX;
    let mut worst_lose = usize::MIN;
    check_all_items(|(win, cost)| {
        if win && cost < best_win {
            best_win = cost;
        } else if !win && cost > worst_lose {
            worst_lose = cost;
        }
    });
    (best_win, worst_lose)
}

fn main() {
    let (best, worst) = solve();
    println!("Part 1: {}", best);
    println!("Part 2: {}", worst);
}
