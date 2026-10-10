//! One expedition: a generated trail and the automatic fight along it.
//! Everything here follows from the seed, the party and the chosen element.

use super::data::{Class, Data, Element, Rules};
use super::loot::{Gear, Horse, roll_gear};
use super::rng::Rng;

#[derive(Clone, Debug, PartialEq)]
pub struct Spawn {
    pub enemy: usize,
    pub element: Element,
}

/// The trail for one seed: ordinary encounters, then the final one.
#[derive(Clone, Debug, PartialEq)]
pub struct Plan {
    pub seed: u64,
    pub level: u32,
    pub encounters: Vec<Vec<Spawn>>,
    /// A fairy spring heals the party after this encounter (index), if any.
    pub bonus_after: Option<usize>,
}

impl Plan {
    pub fn generate(data: &Data, seed: u64, level: u32) -> Plan {
        let mut rng = Rng::new(seed);
        let ordinary = data.rules.ordinary_encounters;

        let mut order: Vec<usize> = (0..data.encounters.ordinary.len()).collect();
        rng.shuffle(&mut order);
        let mut pieces: Vec<&Vec<String>> = order
            .iter()
            .cycle()
            .take(ordinary)
            .map(|&i| &data.encounters.ordinary[i])
            .collect();
        pieces.push(rng.pick(&data.encounters.finals));

        let encounters = pieces
            .iter()
            .map(|piece| {
                piece
                    .iter()
                    .map(|id| Spawn {
                        enemy: data.enemy_index(id),
                        element: *rng.pick(&Element::ALL),
                    })
                    .collect()
            })
            .collect();

        let bonus_after = if rng.chance(data.rules.bonus_chance) {
            Some(rng.below(ordinary as u32) as usize)
        } else {
            None
        };

        Plan {
            seed,
            level,
            encounters,
            bonus_after,
        }
    }
}

#[derive(Clone, Debug)]
pub struct Fighter {
    pub name: String,
    pub class: Class,
    pub health: i32,
    pub max_health: i32,
    pub attack: i32,
    pub defense: i32,
    pub elemental: i32,
    attack_timer: f32,
    ability_timer: f32,
}

#[derive(Clone, Debug)]
pub struct Foe {
    pub name: String,
    pub element: Element,
    pub health: i32,
    pub max_health: i32,
    pub attack: i32,
    pub defense: i32,
    attack_interval: f32,
    attack_timer: f32,
    coins: (i32, i32),
}

/// An enemy brought down, for the presentation to show: which one, and what
/// it dropped.
#[derive(Clone, Debug, PartialEq)]
pub struct Kill {
    /// Its index among the encounter's enemies, which is where it stood.
    pub foe: usize,
    pub max_health: i32,
    pub coins: i32,
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub enum Phase {
    Travelling,
    Fighting,
    Finished,
}

#[derive(Clone, Debug, PartialEq)]
pub struct Outcome {
    pub won: bool,
    pub cleared: usize,
    pub total: usize,
    pub element: Element,
    pub strong_hits: u32,
    pub resisted_hits: u32,
    pub coins: i32,
    pub drop: Option<Gear>,
}

#[derive(Clone, Debug)]
pub struct Expedition {
    pub plan: Plan,
    pub element: Element,
    pub party: Vec<Fighter>,
    pub foes: Vec<Foe>,
    /// Index of the encounter being fought, or travelled towards.
    pub encounter: usize,
    pub phase: Phase,
    /// Seconds of the Mage's Ward left.
    pub ward: f32,
    pub coins: i32,
    pub log: Vec<String>,
    /// Kills not yet shown, oldest first. Whoever presents the run takes them.
    pub kills: Vec<Kill>,
    pub outcome: Option<Outcome>,
    travel: f32,
    strong_hits: u32,
    resisted_hits: u32,
    rng: Rng,
}

impl Expedition {
    pub fn new(data: &Data, plan: Plan, party: &[Horse], element: Element) -> Expedition {
        let rules = &data.rules;
        let party = party
            .iter()
            .map(|horse| {
                let stats = horse.stats();
                Fighter {
                    name: horse.name.clone(),
                    class: horse.class,
                    health: stats.health,
                    max_health: stats.health,
                    attack: stats.attack,
                    defense: stats.defense,
                    elemental: stats.elemental,
                    attack_timer: rules.attack_interval,
                    ability_timer: match horse.class {
                        Class::Ranger => rules.focused_shot_cooldown,
                        Class::Wizard => rules.burst_cooldown,
                        // The first Ward comes early, so it is seen in every fight.
                        Class::Mage => rules.ward_cooldown * 0.25,
                    },
                }
            })
            .collect();

        Expedition {
            // Kept apart from the plan's rolls, so the trail does not change
            // with how the fight happens to go.
            rng: Rng::new(plan.seed ^ 0xC0FF_EE00_F00D_5EED),
            plan,
            element,
            party,
            foes: Vec::new(),
            encounter: 0,
            phase: Phase::Travelling,
            ward: 0.0,
            coins: 0,
            log: vec![format!(
                "The party sets out, attuned to {}.",
                element.name()
            )],
            kills: Vec::new(),
            outcome: None,
            travel: rules.travel_secs,
            strong_hits: 0,
            resisted_hits: 0,
        }
    }

    pub fn tick(&mut self, data: &Data, dt: f32) {
        match self.phase {
            Phase::Finished => {}
            Phase::Travelling => {
                self.travel -= dt;
                if self.travel <= 0.0 {
                    self.start_encounter(data);
                }
            }
            Phase::Fighting => self.fight(data, dt),
        }
    }

    fn start_encounter(&mut self, data: &Data) {
        let scale = 1.0 + data.rules.level_scaling * self.plan.level as f32;
        self.foes = self.plan.encounters[self.encounter]
            .iter()
            .map(|spawn| {
                let def = &data.enemies[spawn.enemy];
                let health = (def.health as f32 * scale).round() as i32;
                Foe {
                    name: def.name.clone(),
                    element: spawn.element,
                    health,
                    max_health: health,
                    attack: (def.attack as f32 * scale).round() as i32,
                    defense: def.defense,
                    attack_interval: def.attack_interval,
                    attack_timer: def.attack_interval,
                    coins: def.coins,
                }
            })
            .collect();
        let names: Vec<String> = self
            .foes
            .iter()
            .map(|f| format!("{} ({})", f.name, f.element.name()))
            .collect();
        self.log.push(format!(
            "Encounter {}/{}: {}",
            self.encounter + 1,
            self.plan.encounters.len(),
            names.join(", ")
        ));
        self.phase = Phase::Fighting;
    }

    fn fight(&mut self, data: &Data, dt: f32) {
        let rules = &data.rules;
        self.ward = (self.ward - dt).max(0.0);

        for i in 0..self.party.len() {
            if self.party[i].health <= 0 {
                continue;
            }
            self.party[i].attack_timer -= dt;
            self.party[i].ability_timer -= dt;

            if self.party[i].attack_timer <= 0.0 {
                self.party[i].attack_timer += rules.attack_interval;
                if let Some(target) = self.target() {
                    let (attack, elemental) = self.power(i);
                    self.strike(rules, i, target, attack, elemental, "hits");
                }
            }

            if self.party[i].ability_timer <= 0.0 {
                let (attack, elemental) = self.power(i);
                match self.party[i].class {
                    Class::Ranger => {
                        self.party[i].ability_timer += rules.focused_shot_cooldown;
                        if let Some(target) = self.target() {
                            let attack = attack * rules.focused_shot_multiplier;
                            self.strike(rules, i, target, attack, elemental, "Focused Shot on");
                        }
                    }
                    Class::Wizard => {
                        self.party[i].ability_timer += rules.burst_cooldown;
                        let burst = elemental * rules.burst_multiplier;
                        for target in 0..self.foes.len() {
                            if self.foes[target].health > 0 {
                                self.strike(rules, i, target, 0.0, burst, "Elemental Burst on");
                            }
                        }
                    }
                    Class::Mage => {
                        self.party[i].ability_timer += rules.ward_cooldown;
                        self.ward = rules.ward_duration;
                        self.log
                            .push(format!("{} raises a Ward.", self.party[i].name));
                    }
                }
            }
        }

        for j in 0..self.foes.len() {
            if self.foes[j].health <= 0 {
                continue;
            }
            self.foes[j].attack_timer -= dt;
            if self.foes[j].attack_timer > 0.0 {
                continue;
            }
            self.foes[j].attack_timer += self.foes[j].attack_interval;

            let standing: Vec<usize> = (0..self.party.len())
                .filter(|&i| self.party[i].health > 0)
                .collect();
            if standing.is_empty() {
                break;
            }
            let victim = &mut self.party[*self.rng.pick(&standing)];
            let mut damage = (self.foes[j].attack - victim.defense).max(1) as f32;
            let warded = self.ward > 0.0;
            if warded {
                damage *= 1.0 - rules.ward_reduction;
            }
            let damage = damage.round().max(1.0) as i32;
            victim.health = (victim.health - damage).max(0);
            self.log.push(format!(
                "{} hits {} for {}{}",
                self.foes[j].name,
                victim.name,
                damage,
                if warded { " (Warded)" } else { "" }
            ));
            if victim.health == 0 {
                self.log.push(format!("{} falls!", victim.name));
            }
        }

        if self.party.iter().all(|f| f.health <= 0) {
            self.log.push("The party is driven back.".into());
            self.finish(data, false);
        } else if self.target().is_none() {
            self.encounter_cleared(data);
        }
    }

    fn encounter_cleared(&mut self, data: &Data) {
        let rules = &data.rules;
        let cleared = self.encounter;
        self.foes.clear();
        if cleared + 1 == self.plan.encounters.len() {
            self.log.push("The corruption lifts. Victory!".into());
            self.finish(data, true);
            return;
        }
        if self.plan.bonus_after == Some(cleared) {
            for fighter in self.party.iter_mut().filter(|f| f.health > 0) {
                let heal = (fighter.max_health as f32 * rules.bonus_heal).round() as i32;
                fighter.health = (fighter.health + heal).min(fighter.max_health);
            }
            self.log.push("A fairy spring restores the party.".into());
        }
        self.encounter += 1;
        self.travel = rules.travel_secs;
        self.phase = Phase::Travelling;
    }

    fn finish(&mut self, data: &Data, won: bool) {
        let cleared = if won {
            self.plan.encounters.len()
        } else {
            self.encounter
        };
        let drop = (cleared > 0).then(|| {
            let mut rng = Rng::new(self.plan.seed ^ 0x100F_100F_100F_100F);
            roll_gear(data, &mut rng, won, self.plan.level, self.party.len())
        });
        self.phase = Phase::Finished;
        self.outcome = Some(Outcome {
            won,
            cleared,
            total: self.plan.encounters.len(),
            element: self.element,
            strong_hits: self.strong_hits,
            resisted_hits: self.resisted_hits,
            coins: self.coins,
            drop,
        });
    }

    /// The first enemy still standing.
    fn target(&self) -> Option<usize> {
        self.foes.iter().position(|f| f.health > 0)
    }

    fn power(&self, horse: usize) -> (f32, f32) {
        let fighter = &self.party[horse];
        (fighter.attack as f32, fighter.elemental as f32)
    }

    /// Deal a hit: the physical part is reduced by defense, the elemental part
    /// is scaled by the triangle.
    fn strike(
        &mut self,
        rules: &Rules,
        horse: usize,
        target: usize,
        physical: f32,
        elemental: f32,
        verb: &str,
    ) {
        let foe = &mut self.foes[target];
        let multiplier = rules.multiplier(self.element, foe.element);
        let damage = (physical - foe.defense as f32).max(0.0) + elemental * multiplier;
        let damage = damage.round().max(1.0) as i32;
        foe.health = (foe.health - damage).max(0);

        let mut note = "";
        if elemental > 0.0 && multiplier > 1.0 {
            self.strong_hits += 1;
            note = "  Strong!";
        } else if elemental > 0.0 && multiplier < 1.0 {
            self.resisted_hits += 1;
            note = "  Resisted";
        }
        self.log.push(format!(
            "{} {} {} for {}{}",
            self.party[horse].name, verb, foe.name, damage, note
        ));

        if foe.health == 0 {
            let coins = self.rng.range(foe.coins.0, foe.coins.1);
            self.coins += coins;
            self.log
                .push(format!("{} is defeated (+{} coins)", foe.name, coins));
            self.kills.push(Kill {
                foe: target,
                max_health: foe.max_health,
                coins,
            });
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const TICK: f32 = 0.1;

    fn party(data: &Data) -> Vec<Horse> {
        data.horses.iter().map(Horse::new).collect()
    }

    fn run(data: &Data, seed: u64, level: u32, element: Element) -> Expedition {
        let plan = Plan::generate(data, seed, level);
        let mut expedition = Expedition::new(data, plan, &party(data), element);
        // Ten minutes of game time is far past any real run.
        for _ in 0..6000 {
            expedition.tick(data, TICK);
            if expedition.phase == Phase::Finished {
                return expedition;
            }
        }
        panic!("seed {seed} did not finish");
    }

    #[test]
    fn triangle_multipliers() {
        let rules = Data::load().rules;
        use Element::*;
        for (attack, strong, weak) in [(Fire, Plant, Ice), (Plant, Ice, Fire), (Ice, Fire, Plant)] {
            assert_eq!(rules.multiplier(attack, strong), 1.5);
            assert_eq!(rules.multiplier(attack, weak), 0.75);
            assert_eq!(rules.multiplier(attack, attack), 1.0);
        }
    }

    #[test]
    fn trail_shape() {
        let data = Data::load();
        for seed in 0..500 {
            let plan = Plan::generate(&data, seed, 0);
            assert_eq!(plan.encounters.len(), data.rules.ordinary_encounters + 1);
            for encounter in &plan.encounters {
                assert!((1..=3).contains(&encounter.len()));
            }
        }
    }

    #[test]
    fn same_seed_same_run() {
        let data = Data::load();
        for seed in [1, 42, 9001] {
            let a = run(&data, seed, 0, Element::Fire);
            let b = run(&data, seed, 0, Element::Fire);
            assert_eq!(a.plan, b.plan);
            assert_eq!(a.outcome, b.outcome);
            assert_eq!(a.log, b.log);
        }
        assert_ne!(
            Plan::generate(&data, 1, 0).encounters,
            Plan::generate(&data, 2, 0).encounters
        );
    }

    /// The first trail should be winnable but not a formality, and the
    /// element choice should matter. Run with `--nocapture` to see the rates.
    #[test]
    fn first_trail_balance() {
        let data = Data::load();
        let seeds = 300;
        let (mut wins, mut best_wins, mut worst_wins, mut ticks) = (0, 0, 0, 0usize);
        for seed in 0..seeds {
            let won: Vec<bool> = Element::ALL
                .iter()
                .map(|&element| {
                    let run = run(&data, seed, 0, element);
                    ticks += run.log.len();
                    run.outcome.unwrap().won
                })
                .collect();
            wins += won.iter().filter(|&&w| w).count();
            best_wins += won.iter().any(|&w| w) as usize;
            worst_wins += won.iter().all(|&w| w) as usize;
        }
        let rate = wins as f32 / (seeds * 3) as f32;
        let best = best_wins as f32 / seeds as f32;
        let worst = worst_wins as f32 / seeds as f32;
        println!("win rate: any element {rate:.2}, best {best:.2}, worst {worst:.2}");
        println!("log lines per run: {}", ticks / (seeds as usize * 3));
        assert!((0.25..=0.75).contains(&rate), "win rate {rate}");
        assert!(best - worst >= 0.2, "element choice barely matters");
    }

    /// Every enemy brought down is reported once, with the coins it dropped.
    #[test]
    fn every_kill_is_reported_with_its_coins() {
        let data = Data::load();
        for seed in 0..100 {
            let run = run(&data, seed, 0, Element::Plant);
            let outcome = run.outcome.clone().unwrap();
            let defeated = run.log.iter().filter(|line| line.contains("is defeated")).count();
            assert_eq!(run.kills.len(), defeated);
            assert_eq!(run.kills.iter().map(|k| k.coins).sum::<i32>(), outcome.coins);
            let fought: usize = run.plan.encounters[..outcome.cleared].iter().map(Vec::len).sum();
            assert!(run.kills.len() >= fought);
            for kill in &run.kills {
                assert!(kill.foe < 3 && kill.max_health > 0, "{kill:?}");
            }
        }
    }

    #[test]
    fn drops_follow_the_seed() {
        let data = Data::load();
        let mut seen = 0;
        for seed in 0..200 {
            let a = run(&data, seed, 0, Element::Ice).outcome.unwrap();
            let b = run(&data, seed, 0, Element::Ice).outcome.unwrap();
            assert_eq!(a.drop, b.drop);
            assert_eq!(a.drop.is_some(), a.cleared > 0);
            if let Some(gear) = a.drop {
                assert!(gear.fits < data.horses.len());
                assert!(super::super::loot::score(&data, gear.stats) >= 2);
                seen += 1;
            }
        }
        assert!(seen > 0);
    }
}
