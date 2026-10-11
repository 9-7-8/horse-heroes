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
    fountain: String,
    death: String,
}

/// An enemy brought down, for the presentation to show: which one, and what
/// it dropped.
#[derive(Clone, Debug, PartialEq)]
pub struct Kill {
    /// Its index among the encounter's enemies, which is where it stood.
    pub foe: usize,
    pub element: Element,
    pub max_health: i32,
    pub coins: i32,
    /// The kind of kill: its enemy's `fountain` in `enemies.ron`.
    pub fountain: String,
    /// How it died: its enemy's `death` in `enemies.ron`.
    pub death: String,
    /// The gear it dropped. Only the enemy whose fall wins the trail drops
    /// any: the run's piece, the same one the outcome carries.
    pub gear: Option<Gear>,
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
                    fountain: def.fountain.clone(),
                    death: def.death.clone(),
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
            roll_gear(data, &mut rng, won, self.plan.level)
        });
        // A trail is won in the tick its last enemy falls, so that kill is
        // still here, not yet taken.
        if won && let Some(kill) = self.kills.last_mut() {
            kill.gear = drop.clone();
        }
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
                element: foe.element,
                max_health: foe.max_health,
                coins,
                fountain: foe.fountain.clone(),
                death: foe.death.clone(),
                gear: None,
            });
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::sim::data::{Slot, Stats};

    const TICK: f32 = 0.1;

    fn party(data: &Data) -> Vec<Horse> {
        data.horses.iter().map(Horse::new).collect()
    }

    fn run(data: &Data, seed: u64, level: u32, element: Element) -> Expedition {
        run_with(data, seed, level, element, &party(data))
    }

    fn run_with(data: &Data, seed: u64, level: u32, element: Element, party: &[Horse]) -> Expedition {
        let plan = Plan::generate(data, seed, level);
        let mut expedition = Expedition::new(data, plan, party, element);
        // Ten minutes of game time is far past any real run.
        for _ in 0..6000 {
            expedition.tick(data, TICK);
            if expedition.phase == Phase::Finished {
                return expedition;
            }
        }
        panic!("seed {seed} did not finish");
    }

    /// A hand-made trail, attuned to Fire, standing in its first encounter.
    fn fighting(data: &Data, level: u32, encounters: &[&[(&str, Element)]]) -> Expedition {
        let plan = Plan {
            seed: 7,
            level,
            encounters: encounters
                .iter()
                .map(|piece| {
                    piece
                        .iter()
                        .map(|&(id, element)| Spawn {
                            enemy: data.enemy_index(id),
                            element,
                        })
                        .collect()
                })
                .collect(),
            bonus_after: None,
        };
        let mut expedition = Expedition::new(data, plan, &party(data), Element::Fire);
        expedition.start_encounter(data);
        expedition
    }

    /// Keep the party from acting, so only the enemies do anything.
    fn hold_the_party(expedition: &mut Expedition) {
        for fighter in &mut expedition.party {
            fighter.attack_timer = f32::MAX;
            fighter.ability_timer = f32::MAX;
        }
    }

    fn health_lost(expedition: &Expedition) -> i32 {
        expedition.party.iter().map(|f| f.max_health - f.health).sum()
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

    /// A kill says how its enemy dies and what element it was, so that the
    /// body can be shown going: each is one its trail's enemies have, and the
    /// kill that wins a trail dies the way one of the finals does.
    #[test]
    fn every_kill_says_how_its_enemy_dies() {
        let data = Data::load();
        let deaths_of = |ids: &[String]| -> Vec<&str> {
            ids.iter().map(|id| data.enemies[data.enemy_index(id)].death.as_str()).collect()
        };
        let finals: Vec<&str> = data.encounters.finals.iter().flat_map(|e| deaths_of(e)).collect();
        let mut won = 0;
        for seed in 0..100 {
            let run = run(&data, seed, 0, Element::Fire);
            let spawns: Vec<&Spawn> = run.plan.encounters.iter().flatten().collect();
            for kill in &run.kills {
                let known = spawns.iter().any(|spawn| {
                    spawn.element == kill.element && data.enemies[spawn.enemy].death == kill.death
                });
                assert!(known, "{kill:?}");
            }
            if run.outcome.as_ref().unwrap().won {
                assert!(finals.contains(&run.kills.last().unwrap().death.as_str()));
                won += 1;
            }
        }
        assert!(won > 0);
    }

    /// The run's gear comes out of the enemy whose fall won the trail, and
    /// out of no other. A lost trail's piece has no body to come out of.
    #[test]
    fn the_winning_kill_drops_the_gear() {
        let data = Data::load();
        let (mut won, mut lost) = (0, 0);
        for seed in 0..100 {
            let run = run(&data, seed, 0, Element::Fire);
            let outcome = run.outcome.clone().unwrap();
            let Some((last, earlier)) = run.kills.split_last() else {
                assert!(!outcome.won);
                continue;
            };
            assert!(earlier.iter().all(|kill| kill.gear.is_none()));
            if outcome.won {
                assert!(last.gear.is_some());
                assert_eq!(last.gear, outcome.drop);
                won += 1;
            } else {
                assert!(last.gear.is_none());
                lost += 1;
            }
        }
        assert!(won > 0 && lost > 0);
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
                assert!(super::super::loot::score(&data, gear.stats) >= 2);
                seen += 1;
            }
        }
        assert!(seen > 0);
    }

    /// A hit is its physical part less the target's defense, plus its
    /// elemental part scaled by the triangle, and never less than 1.
    #[test]
    fn a_hit_is_physical_less_defense_plus_elemental_by_the_triangle() {
        let data = Data::load();
        let rules = &data.rules;
        use Element::*;
        let mut run = fighting(
            &data,
            0,
            &[&[("corrupted_stag", Plant), ("corrupted_stag", Fire), ("corrupted_stag", Ice)]],
        );
        let defense = run.foes[0].defense as f32;
        assert!(defense > 0.0, "the stag is the enemy with defense");
        let (physical, elemental) = (defense + 5.0, 4.0);

        let mut dealt = Vec::new();
        for (target, multiplier) in [rules.strong_multiplier, 1.0, rules.weak_multiplier]
            .into_iter()
            .enumerate()
        {
            run.strike(rules, 0, target, physical, elemental, "hits");
            let foe = &run.foes[target];
            dealt.push(foe.max_health - foe.health);
            assert_eq!(dealt[target], (5.0 + elemental * multiplier).round() as i32);
        }
        assert!(dealt[0] > dealt[1] && dealt[1] > dealt[2], "{dealt:?}");
        assert_eq!((run.strong_hits, run.resisted_hits), (1, 1));

        // Defense never turns a hit into healing, and a hit with nothing
        // elemental in it is neither strong nor resisted.
        let before = run.foes[0].health;
        run.strike(rules, 0, 0, defense * 0.5, 0.0, "hits");
        assert_eq!(run.foes[0].health, before - 1);
        assert_eq!((run.strong_hits, run.resisted_hits), (1, 1));
    }

    /// Enemies gain health and attack with the trail level, and nothing else.
    #[test]
    fn enemies_grow_with_the_trail_level() {
        let data = Data::load();
        let trail: &[&[(&str, Element)]] =
            &[&[("mire_toad", Element::Ice), ("corrupted_stag", Element::Ice)]];
        let base = fighting(&data, 0, trail);
        let level = 4;
        let grown = fighting(&data, level, trail);
        let scale = 1.0 + data.rules.level_scaling * level as f32;
        for (foe, at_zero) in grown.foes.iter().zip(&base.foes) {
            assert_eq!(foe.max_health, (at_zero.max_health as f32 * scale).round() as i32);
            assert_eq!(foe.attack, (at_zero.attack as f32 * scale).round() as i32);
            assert!(foe.max_health > at_zero.max_health && foe.attack > at_zero.attack);
            assert_eq!(foe.health, foe.max_health);
            assert_eq!(foe.defense, at_zero.defense);
        }
    }

    /// Under the Ward a hit lands for less, by `ward_reduction`; the Mage
    /// raises it without being told, and it runs out.
    #[test]
    fn the_ward_reduces_damage_taken_and_runs_out() {
        let data = Data::load();
        let rules = &data.rules;
        let toad: &[&[(&str, Element)]] = &[&[("mire_toad", Element::Ice)]];
        let mut plain = fighting(&data, 0, toad);
        hold_the_party(&mut plain);
        let mut warded = plain.clone();
        warded.ward = f32::MAX;

        // Long enough for the toad's first hit, too short for its second.
        let dt = plain.foes[0].attack_interval * 1.5;
        plain.tick(&data, dt);
        warded.tick(&data, dt);
        let (full, less) = (health_lost(&plain), health_lost(&warded));
        assert!(full > 1, "the toad's hit should be worth reducing");
        assert_eq!(less, ((full as f32 * (1.0 - rules.ward_reduction)).round() as i32).max(1));
        assert!(less < full);
        assert!(warded.log.last().unwrap().ends_with("(Warded)"));
        assert!(!plain.log.last().unwrap().ends_with("(Warded)"));

        let mut run = fighting(&data, 0, toad);
        assert_eq!(run.ward, 0.0);
        let mut raised = false;
        for _ in 0..(rules.ward_cooldown / TICK) as usize + 1 {
            run.tick(&data, TICK);
            assert!(run.ward <= rules.ward_duration);
            raised |= run.ward > 0.0;
        }
        assert!(raised, "the Mage never raised a Ward");

        let mut fading = fighting(&data, 0, toad);
        hold_the_party(&mut fading);
        fading.ward = rules.ward_duration;
        fading.tick(&data, rules.ward_duration * 0.5);
        assert!(fading.ward > 0.0 && fading.ward < rules.ward_duration);
        fading.tick(&data, rules.ward_duration);
        assert_eq!(fading.ward, 0.0);
    }

    /// An enemy's hit is its attack less the horse's defense, never less
    /// than 1, and it lands only on a horse still standing.
    #[test]
    fn an_enemy_hit_is_its_attack_less_defense_on_a_standing_horse() {
        let data = Data::load();
        let toad: &[&[(&str, Element)]] = &[&[("mire_toad", Element::Ice)]];
        let hit_with = |defense: i32| {
            let mut run = fighting(&data, 0, toad);
            hold_the_party(&mut run);
            for fighter in &mut run.party {
                fighter.defense = defense;
            }
            // Long enough for the toad's first hit, too short for its second.
            let dt = run.foes[0].attack_interval * 1.5;
            run.tick(&data, dt);
            (health_lost(&run), run.foes[0].attack)
        };
        let (lost, attack) = hit_with(2);
        assert!(attack > 3, "the toad's hit should be worth blocking");
        assert_eq!(lost, attack - 2);
        assert_eq!(hit_with(attack).0, 1);
        assert_eq!(hit_with(attack + 50).0, 1);

        let mut run = fighting(&data, 0, toad);
        hold_the_party(&mut run);
        run.party[0].health = 0;
        run.party[1].health = 0;
        run.party[2].health = 10_000;
        run.party[2].max_health = 10_000;
        let dt = run.foes[0].attack_interval;
        for _ in 0..20 {
            run.tick(&data, dt);
        }
        let hits = lines_with(&run, "Mire Toad hits") as i32;
        assert!(hits >= 10, "{hits} hits");
        let each = (run.foes[0].attack - run.party[2].defense).max(1);
        assert_eq!(run.party[2].max_health - run.party[2].health, hits * each);
        assert_eq!((run.party[0].health, run.party[1].health), (0, 0));
        assert_eq!(run.phase, Phase::Fighting);
    }

    /// A fallen horse does nothing more: no hit and no ability.
    #[test]
    fn a_fallen_horse_does_nothing() {
        let data = Data::load();
        let mut run = fighting(&data, 0, &[&[("corrupted_stag", Element::Fire)]]);
        run.foes[0].health = i32::MAX;
        run.foes[0].attack_timer = f32::MAX;
        run.party[0].health = 0;
        run.party[1].health = 0;
        let lines = run.log.len();
        for _ in 0..200 {
            run.tick(&data, TICK);
        }
        let acted = |horse: usize| {
            let name = &run.party[horse].name;
            run.log[lines..].iter().filter(|line| line.starts_with(name.as_str())).count()
        };
        assert_eq!((acted(0), acted(1)), (0, 0));
        assert!(acted(2) > 0);
        assert_eq!(run.log.len() - lines, acted(2));
    }

    /// A trail is lost when the last horse falls. Lost in its first
    /// encounter it brings nothing back; lost further on it brings a piece,
    /// which no enemy drops.
    #[test]
    fn a_lost_trail_brings_back_a_piece_only_past_its_first_encounter() {
        let data = Data::load();
        let trail: &[&[(&str, Element)]] = &[
            &[("mire_toad", Element::Ice)],
            &[("mire_toad", Element::Ice)],
            &[("hollow_wolf", Element::Ice)],
        ];
        let lose = |cleared: usize| {
            let mut run = fighting(&data, 0, trail);
            for _ in 0..cleared {
                run.foes.clear();
                run.encounter_cleared(&data);
            }
            hold_the_party(&mut run);
            for fighter in &mut run.party {
                fighter.health = 1;
            }
            for _ in 0..600 {
                run.tick(&data, TICK);
            }
            assert_eq!(run.phase, Phase::Finished);
            assert!(run.party.iter().all(|f| f.health == 0));
            assert_eq!(lines_with(&run, "driven back"), 1);
            assert!(run.kills.is_empty());
            let outcome = run.outcome.unwrap();
            assert_eq!((outcome.won, outcome.cleared, outcome.total), (false, cleared, 3));
            outcome.drop
        };
        assert_eq!(lose(0), None);
        assert!(lose(1).is_some());
        assert!(lose(2).is_some());
    }

    fn horse_of(expedition: &Expedition, class: Class) -> usize {
        expedition.party.iter().position(|f| f.class == class).unwrap()
    }

    /// Let one horse use its ability on the next tick, and nobody do
    /// anything else.
    fn only_the_ability_of(expedition: &mut Expedition, horse: usize) {
        hold_the_party(expedition);
        for foe in &mut expedition.foes {
            foe.attack_timer = f32::MAX;
        }
        expedition.party[horse].ability_timer = TICK * 0.5;
    }

    fn lines_with(expedition: &Expedition, text: &str) -> usize {
        expedition.log.iter().filter(|line| line.contains(text)).count()
    }

    /// Focused Shot is one hit on the current target and no other, with the
    /// Ranger's attack times `focused_shot_multiplier` and the elemental
    /// part left as it is.
    #[test]
    fn focused_shot_multiplies_the_attack_on_the_current_target() {
        let data = Data::load();
        let rules = &data.rules;
        let mut run = fighting(
            &data,
            0,
            &[&[("corrupted_stag", Element::Plant), ("mire_toad", Element::Plant)]],
        );
        let ranger = horse_of(&run, Class::Ranger);
        only_the_ability_of(&mut run, ranger);
        run.tick(&data, TICK);

        let (attack, elemental) = run.power(ranger);
        let physical = attack * rules.focused_shot_multiplier - run.foes[0].defense as f32;
        let expected = (physical + elemental * rules.strong_multiplier).round() as i32;
        assert_eq!(run.foes[0].max_health - run.foes[0].health, expected);
        assert_eq!(run.foes[1].health, run.foes[1].max_health);
        assert_eq!(lines_with(&run, "Focused Shot on"), 1);
        assert_eq!(run.strong_hits, 1);

        // It is worth more than an ordinary hit, by the attack alone.
        let before = run.foes[0].health;
        run.strike(rules, ranger, 0, attack, elemental, "hits");
        let ordinary = before - run.foes[0].health;
        assert_eq!(
            expected - ordinary,
            (attack * (rules.focused_shot_multiplier - 1.0)).round() as i32
        );
    }

    /// Elemental Burst hits every enemy still standing with the Wizard's
    /// elemental times `burst_multiplier`, by the triangle, past defense
    /// and with no attack in it.
    #[test]
    fn elemental_burst_hits_every_standing_enemy_with_elemental_alone() {
        let data = Data::load();
        let rules = &data.rules;
        use Element::*;
        let mut run = fighting(
            &data,
            0,
            &[&[
                ("corrupted_stag", Plant),
                ("mire_toad", Fire),
                ("thornling", Fire),
                ("mire_toad", Ice),
            ]],
        );
        assert!(run.foes[0].defense > 0, "the stag is the enemy with defense");
        // The thornling fell earlier in this fight.
        run.foes[2].health = 0;
        let wizard = horse_of(&run, Class::Wizard);
        only_the_ability_of(&mut run, wizard);
        run.tick(&data, TICK);

        let burst = run.power(wizard).1 * rules.burst_multiplier;
        for (target, multiplier) in [(0, rules.strong_multiplier), (1, 1.0), (3, rules.weak_multiplier)] {
            let foe = &run.foes[target];
            assert_eq!(foe.max_health - foe.health, (burst * multiplier).round() as i32, "{}", foe.name);
        }
        assert_eq!(lines_with(&run, "Elemental Burst on"), 3);
        assert_eq!((run.strong_hits, run.resisted_hits), (1, 1));
        assert!(run.kills.is_empty(), "a fallen enemy is not hit again");
    }

    /// Each ability comes round once per cooldown, the first time after one
    /// whole cooldown.
    #[test]
    fn the_shot_and_the_burst_keep_their_cooldowns() {
        let data = Data::load();
        let rules = &data.rules;
        for (class, cooldown, line) in [
            (Class::Ranger, rules.focused_shot_cooldown, "Focused Shot on"),
            (Class::Wizard, rules.burst_cooldown, "Elemental Burst on"),
        ] {
            let mut run = fighting(&data, 0, &[&[("corrupted_stag", Element::Fire)]]);
            let horse = horse_of(&run, class);
            let first = run.party[horse].ability_timer;
            assert_eq!(first, cooldown);
            only_the_ability_of(&mut run, horse);
            run.party[horse].ability_timer = first;
            run.foes[0].health = i32::MAX;

            // Half a cooldown short of the first use, then three and a half
            // cooldowns in all: three uses.
            for _ in 0..(cooldown * 0.5 / TICK).round() as usize {
                run.tick(&data, TICK);
            }
            assert_eq!(lines_with(&run, line), 0);
            for _ in 0..(cooldown * 3.0 / TICK).round() as usize {
                run.tick(&data, TICK);
            }
            assert_eq!(lines_with(&run, line), 3, "{line}");
        }
    }

    /// The fairy spring heals the horses still standing, by `bonus_heal` of
    /// their health and not past it, after its own encounter and no other.
    #[test]
    fn the_fairy_spring_heals_the_standing_and_no_further() {
        let data = Data::load();
        let trail: &[&[(&str, Element)]] = &[
            &[("mire_toad", Element::Ice)],
            &[("mire_toad", Element::Ice)],
            &[("hollow_wolf", Element::Ice)],
        ];
        let hurt = |spring: Option<usize>| {
            let mut run = fighting(&data, 0, trail);
            run.plan.bonus_after = spring;
            run.party[0].health = 1;
            run.party[1].health = 0;
            run.foes.clear();
            run.encounter_cleared(&data);
            run
        };

        let healed = hurt(Some(0));
        let first = &healed.party[0];
        let heal = (first.max_health as f32 * data.rules.bonus_heal).round() as i32;
        assert!(heal > 0 && 1 + heal < first.max_health);
        assert_eq!(first.health, 1 + heal);
        assert_eq!(healed.party[1].health, 0, "a fallen horse is not raised");
        assert_eq!(healed.party[2].health, healed.party[2].max_health);
        assert!(healed.log.iter().any(|line| line.contains("fairy spring")));
        assert_eq!((healed.encounter, healed.phase), (1, Phase::Travelling));

        for spring in [None, Some(1)] {
            let dry = hurt(spring);
            assert_eq!(dry.party[0].health, 1);
            assert!(!dry.log.iter().any(|line| line.contains("fairy spring")));
            assert_eq!((dry.encounter, dry.phase), (1, Phase::Travelling));
        }
    }

    /// A spring is on about `bonus_chance` of trails, and always after an
    /// ordinary encounter, where there is still a fight ahead to heal for.
    #[test]
    fn springs_are_rolled_at_their_chance_and_before_the_final() {
        let data = Data::load();
        let seeds = 4000;
        let mut springs = 0;
        for seed in 0..seeds {
            if let Some(after) = Plan::generate(&data, seed, 0).bonus_after {
                assert!(after < data.rules.ordinary_encounters, "seed {seed}");
                springs += 1;
            }
        }
        let rate = springs as f32 / seeds as f32;
        assert!((rate - data.rules.bonus_chance).abs() < 0.03, "spring rate {rate}");
    }

    /// The trail is the seed's alone: the level and the fight do not move it.
    #[test]
    fn the_trail_does_not_change_with_the_level_or_the_fight() {
        let data = Data::load();
        for seed in 0..50 {
            let plan = Plan::generate(&data, seed, 0);
            let harder = Plan::generate(&data, seed, 5);
            assert_eq!(plan.encounters, harder.encounters);
            assert_eq!(plan.bonus_after, harder.bonus_after);
            for element in Element::ALL {
                assert_eq!(run(&data, seed, 0, element).plan, plan);
            }
        }
    }

    /// What a horse wears goes onto the trail with it, and a party in good
    /// gear wins more often than a bare one.
    #[test]
    fn worn_gear_is_fought_in_and_wins_more_trails() {
        let data = Data::load();
        let mut geared = party(&data);
        for horse in geared.iter_mut() {
            horse.gear[Slot::Body.index()] = Some(Gear {
                name: "Test Blanket".into(),
                slot: Slot::Body,
                rarity: 0,
                stats: Stats {
                    health: 20,
                    attack: 3,
                    defense: 2,
                    elemental: 3,
                },
            });
        }

        let plan = Plan::generate(&data, 1, 0);
        let expedition = Expedition::new(&data, plan, &geared, Element::Fire);
        for (fighter, horse) in expedition.party.iter().zip(&geared) {
            let stats = horse.stats();
            assert_ne!(stats, horse.base);
            assert_eq!((fighter.health, fighter.max_health), (stats.health, stats.health));
            assert_eq!(
                (fighter.attack, fighter.defense, fighter.elemental),
                (stats.attack, stats.defense, stats.elemental)
            );
        }

        let wins = |party: &[Horse]| {
            (0..100)
                .filter(|&seed| run_with(&data, seed, 0, Element::Ice, party).outcome.unwrap().won)
                .count()
        };
        let (bare, worn) = (wins(&party(&data)), wins(&geared));
        assert!(worn > bare, "geared {worn} wins against bare {bare}");
    }

    /// Tick by tick, for a presenter that takes the kills as they come:
    /// health stays between nothing and full, a fallen horse stays down, the
    /// coins only climb and match the kills, and the winning kill still
    /// carries the gear.
    #[test]
    fn a_run_holds_together_tick_by_tick() {
        let data = Data::load();
        let mut won = 0;
        for seed in 0..60 {
            let plan = Plan::generate(&data, seed, 1);
            let mut run = Expedition::new(&data, plan, &party(&data), Element::Plant);
            let mut fallen = vec![false; run.party.len()];
            let mut taken: Vec<Kill> = Vec::new();
            let mut coins = 0;
            for _ in 0..6000 {
                run.tick(&data, TICK);
                for (fighter, down) in run.party.iter().zip(fallen.iter_mut()) {
                    assert!((0..=fighter.max_health).contains(&fighter.health), "seed {seed}");
                    assert!(!*down || fighter.health == 0, "seed {seed}: {} got up", fighter.name);
                    *down = fighter.health == 0;
                }
                for foe in &run.foes {
                    assert!((0..=foe.max_health).contains(&foe.health), "seed {seed}");
                }
                assert!(run.coins >= coins, "seed {seed}");
                coins = run.coins;
                taken.append(&mut run.kills);
                assert_eq!(taken.iter().map(|k| k.coins).sum::<i32>(), run.coins);
                if run.phase == Phase::Finished {
                    break;
                }
            }
            let outcome = run.outcome.clone().expect("the run finished");
            assert_eq!(outcome.total, run.plan.encounters.len());
            assert_eq!(outcome.coins, run.coins);
            assert_eq!(outcome.won, outcome.cleared == outcome.total);
            assert_eq!(outcome.won, fallen.iter().any(|&down| !down));
            if outcome.won {
                assert_eq!(taken.last().unwrap().gear, outcome.drop);
                assert!(outcome.drop.is_some());
                won += 1;
            }

            // A finished run stays as it ended.
            let log = run.log.len();
            run.tick(&data, 60.0);
            assert_eq!((run.log.len(), run.outcome.as_ref()), (log, Some(&outcome)));
        }
        assert!(won > 0);
    }

    /// A trail is ordinary pieces, none of them twice while there are enough
    /// to go round, then one final. Over many seeds every piece comes up in
    /// every place on the trail, and every final and every element comes up.
    #[test]
    fn a_trail_is_ordinary_pieces_in_a_rolled_order_then_a_final() {
        let data = Data::load();
        let (pieces, finals) = (&data.encounters.ordinary, &data.encounters.finals);
        let ordinary = data.rules.ordinary_encounters;
        let place = |among: &[Vec<String>], encounter: &[Spawn]| {
            let ids: Vec<&str> = encounter.iter().map(|s| data.enemies[s.enemy].id.as_str()).collect();
            among.iter().position(|piece| *piece == ids)
        };
        let mut met = vec![vec![false; pieces.len()]; ordinary];
        let mut ended = vec![false; finals.len()];
        let mut elements = [false; 3];
        for seed in 0..500 {
            let plan = Plan::generate(&data, seed, 0);
            let (last, road) = plan.encounters.split_last().unwrap();
            let drawn: Vec<usize> = road
                .iter()
                .map(|encounter| place(pieces, encounter).expect("an ordinary piece"))
                .collect();
            for (at, &piece) in drawn.iter().enumerate() {
                met[at][piece] = true;
                if ordinary <= pieces.len() {
                    assert!(!drawn[..at].contains(&piece), "seed {seed}: a piece twice");
                }
            }
            ended[place(finals, last).expect("a final")] = true;
            for spawn in plan.encounters.iter().flatten() {
                elements[spawn.element as usize] = true;
            }
        }
        assert!(met.iter().flatten().all(|&seen| seen), "{met:?}");
        assert!(ended.iter().all(|&seen| seen), "{ended:?}");
        assert!(elements.iter().all(|&seen| seen), "{elements:?}");
    }

    /// The road to each encounter takes `travel_secs`, and no timer runs on
    /// it: a hit or an ability is no nearer at the end of the road.
    #[test]
    fn the_road_takes_its_time_and_no_timer_runs_on_it() {
        let data = Data::load();
        let rules = &data.rules;
        let plan = Plan::generate(&data, 3, 0);
        let mut run = Expedition::new(&data, plan, &party(&data), Element::Fire);
        let timers = |run: &Expedition| -> Vec<(f32, f32)> {
            run.party.iter().map(|f| (f.attack_timer, f.ability_timer)).collect()
        };
        let walk = |run: &mut Expedition| {
            let mut ticks = 0;
            while run.phase == Phase::Travelling {
                run.tick(&data, TICK);
                ticks += 1;
                assert!(ticks < 6000, "the road never ended");
            }
            ticks as f32 * TICK
        };

        assert_eq!((run.phase, run.encounter), (Phase::Travelling, 0));
        assert!(run.foes.is_empty());
        let mage = horse_of(&run, Class::Mage);
        assert_eq!(run.party[mage].ability_timer, rules.ward_cooldown * 0.25);
        assert!(run.party.iter().all(|f| f.attack_timer == rules.attack_interval));

        let before = timers(&run);
        assert!((walk(&mut run) - rules.travel_secs).abs() <= TICK);
        assert_eq!(run.phase, Phase::Fighting);
        assert_eq!(run.foes.len(), run.plan.encounters[0].len());
        assert_eq!(lines_with(&run, "Encounter 1/"), 1);
        assert_eq!(timers(&run), before);

        run.foes.clear();
        run.encounter_cleared(&data);
        assert_eq!((run.phase, run.encounter), (Phase::Travelling, 1));
        assert!((walk(&mut run) - rules.travel_secs).abs() <= TICK);
        assert_eq!(run.phase, Phase::Fighting);
        assert_eq!(lines_with(&run, "Encounter 2/"), 1);
        assert_eq!(timers(&run), before);
    }

    /// An ordinary hit comes once every `attack_interval` from each horse,
    /// and lands on the first enemy still standing and no other.
    #[test]
    fn each_horse_hits_the_first_standing_enemy_once_an_interval() {
        let data = Data::load();
        let rules = &data.rules;
        use Element::Fire;
        let mut run = fighting(
            &data,
            0,
            &[&[("thornling", Fire), ("mire_toad", Fire), ("gloom_sprite", Fire)]],
        );
        for fighter in &mut run.party {
            fighter.ability_timer = f32::MAX;
        }
        for foe in &mut run.foes {
            foe.attack_timer = f32::MAX;
            foe.health = i32::MAX;
            foe.max_health = i32::MAX;
        }
        // The thornling fell earlier in this fight.
        run.foes[0].health = 0;
        let hits = |run: &Expedition, foe: usize| {
            lines_with(run, &format!(" hits {} for", run.foes[foe].name))
        };
        let horses = run.party.len();

        // Three and a half intervals: three hits from each horse.
        for _ in 0..(rules.attack_interval * 3.5 / TICK).round() as usize {
            run.tick(&data, TICK);
        }
        assert_eq!((hits(&run, 0), hits(&run, 1), hits(&run, 2)), (0, 3 * horses, 0));
        assert_eq!(run.foes[2].health, run.foes[2].max_health);

        // The toad falls, and the next round of hits goes to the sprite.
        run.foes[1].health = 0;
        for _ in 0..(rules.attack_interval / TICK).round() as usize {
            run.tick(&data, TICK);
        }
        assert_eq!((hits(&run, 0), hits(&run, 1), hits(&run, 2)), (0, 3 * horses, horses));
        assert_eq!(run.phase, Phase::Fighting);
    }

    /// A kill's coins are within its enemy's range in `enemies.ron`, and over
    /// many kills they reach both ends of it.
    #[test]
    fn a_kill_drops_coins_within_its_enemys_range() {
        let data = Data::load();
        for enemy in &data.enemies {
            let (lo, hi) = enemy.coins;
            let (mut fewest, mut most) = (i32::MAX, i32::MIN);
            for seed in 0..200 {
                let mut run = fighting(&data, 0, &[&[(enemy.id.as_str(), Element::Fire)]]);
                run.rng = Rng::new(seed);
                run.foes[0].health = 1;
                run.strike(&data.rules, 0, 0, 1.0, 0.0, "hits");
                let kill = run.kills.pop().expect("the enemy fell");
                assert!((lo..=hi).contains(&kill.coins), "{}: {}", enemy.id, kill.coins);
                assert_eq!(run.coins, kill.coins);
                assert!(run.kills.is_empty());
                fewest = fewest.min(kill.coins);
                most = most.max(kill.coins);
            }
            assert_eq!((fewest, most), (lo, hi), "{}", enemy.id);
        }
    }
}
