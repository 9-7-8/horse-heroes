//! One expedition: a generated trail and the automatic fight along it.
//! Everything here follows from the seed and the party: its horses, their
//! gear and the ability each carries (#357).

use super::data::{AbilityKind, Data, Element, Rules};
use super::lessons::Event;
use super::loot::{Gear, Horse, Odds, fixed_piece, roll_gear, roll_piece};
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
    /// The trail's own piece, if the plan gives it (#294); otherwise it is
    /// rolled at the trail's odds when the trail ends.
    pub piece: Option<Gear>,
}

impl Plan {
    /// A new game's first trail (#294): the one plan `encounters.ron` gives
    /// it, the same for every seed, with no spring and its own piece. The
    /// seed still rolls what the fight rolls: whom an enemy strikes, each
    /// swing's timing, coins and the kills' pieces.
    pub fn first(data: &Data, seed: u64) -> Plan {
        let first = &data.encounters.first;
        Plan {
            seed,
            level: 0,
            encounters: first
                .encounters
                .iter()
                .map(|piece| {
                    piece
                        .iter()
                        .map(|(id, element)| Spawn {
                            enemy: data.enemy_index(id),
                            element: *element,
                        })
                        .collect()
                })
                .collect(),
            bonus_after: None,
            piece: Some(fixed_piece(data, &first.piece)),
        }
    }

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
            piece: None,
        }
    }
}

/// What one horse did on a trail (#311), for its view. Damage is the
/// health it took off or lost, so a trail's sums are its enemies' health and
/// the party's.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub struct Deeds {
    pub dealt: i32,
    pub kills: u32,
    pub strong_hits: u32,
    pub resisted_hits: u32,
    pub taken: i32,
    pub wards: u32,
    pub fell: bool,
}

#[derive(Clone, Debug)]
pub struct Fighter {
    pub name: String,
    /// Its one ability, by its place in `abilities.ron` (#357).
    pub ability: usize,
    pub health: i32,
    pub max_health: i32,
    pub attack: i32,
    pub defense: i32,
    pub elemental: i32,
    /// Its speed, gear's included (#355), which its interval comes from.
    pub speed: i32,
    pub deeds: Deeds,
    /// Seconds between its ordinary hits, before each swing's jitter (#354).
    attack_interval: f32,
    attack_timer: f32,
    ability_timer: f32,
}

#[derive(Clone, Debug)]
pub struct Foe {
    /// Its id in `enemies.ron`, which the presentation looks it up by.
    pub id: String,
    pub name: String,
    pub element: Element,
    pub health: i32,
    pub max_health: i32,
    pub attack: i32,
    pub defense: i32,
    attack_interval: f32,
    attack_timer: f32,
    coins: (i32, i32),
    gear: (u32, u32),
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
    /// The gear it dropped: a piece or more, as its enemy's `gear` has it,
    /// and for the enemy whose fall wins the trail the trail's own piece
    /// last (#352). Every piece is in the outcome's `drops` too.
    pub gear: Vec<Gear>,
}

/// A hit that landed, for the presentation to show its number (#249): who
/// took it, how much, and what kind of hit it was.
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Hit {
    pub struck: Struck,
    /// Who dealt it, named as `struck` names who took it.
    pub by: Struck,
    /// The damage the hit did, as the fight log says it.
    pub damage: i32,
    /// The element of its elemental part, if it had one.
    pub element: Option<Element>,
    pub kind: HitKind,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Struck {
    /// An enemy: where it stands among the encounter's enemies, and its full
    /// health, which together say where it is drawn.
    Foe { slot: usize, max_health: i32 },
    /// A horse of the party, by its place in the party.
    Horse(usize),
}

/// One use of an ability: which horse of the party used it, and the
/// ability by its place in `abilities.ron`.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct Used {
    pub horse: usize,
    pub ability: usize,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum HitKind {
    Plain,
    /// Its element was strong against the enemy's.
    Strong,
    /// Its element was resisted by the enemy's.
    Resisted,
    /// An enemy's hit, softened by the Ward.
    Warded,
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
    pub strong_hits: u32,
    pub resisted_hits: u32,
    pub coins: i32,
    /// Every piece the trail brought back, in the order they dropped: the
    /// kills' pieces, then the trail's own if it cleared an encounter.
    pub drops: Vec<Gear>,
    /// What each horse did, in the party's order.
    pub deeds: Vec<Deeds>,
}

impl Outcome {
    /// The best piece the trail brought back: of the highest tier, and of
    /// those the last to drop. What its report names (#231).
    pub fn best(&self) -> Option<&Gear> {
        self.drops.iter().max_by_key(|gear| gear.rarity)
    }

    /// The trail's own piece, the plan's or rolled at the trail's odds: the
    /// last to drop, when an encounter was cleared.
    pub fn trail_piece(&self) -> Option<&Gear> {
        self.drops.last().filter(|_| self.cleared > 0)
    }
}

#[derive(Clone, Debug)]
pub struct Expedition {
    pub plan: Plan,
    pub party: Vec<Fighter>,
    pub foes: Vec<Foe>,
    /// Index of the encounter being fought, or travelled towards.
    pub encounter: usize,
    pub phase: Phase,
    /// Seconds left of the final enemy's approach (#19): its fight is on,
    /// but nothing is struck or cast until this is spent.
    pub approach: f32,
    /// Seconds of a Ward left, and the share it takes off enemy hits.
    pub ward: f32,
    ward_reduction: f32,
    pub coins: i32,
    pub log: Vec<String>,
    /// Kills not yet shown, oldest first. Whoever presents the run takes them.
    pub kills: Vec<Kill>,
    /// Hits not yet shown, oldest first (#249). Taken as the kills are.
    pub hits: Vec<Hit>,
    /// Abilities used and not yet heard or shown, oldest first (#358, #18).
    /// Taken as the hits are; a cast's hits, if it has any, are in `hits`
    /// from the same tick.
    pub casts: Vec<Used>,
    /// Every piece dropped so far, in order.
    pub drops: Vec<Gear>,
    /// What has happened that a lesson may wait for (#290), oldest first.
    /// Whoever presents the run takes them, as it takes the kills.
    pub events: Vec<Event>,
    pub outcome: Option<Outcome>,
    travel: f32,
    /// Seconds the road between two encounters takes this party, at its
    /// average speed (#355).
    road: f32,
    strong_hits: u32,
    resisted_hits: u32,
    rng: Rng,
    /// The kills' pieces are rolled apart from the fight, so the fight goes
    /// as it would with no gear thrown.
    loot: Rng,
    /// The order a fight opens in and each swing's jitter (#354).
    cadence: Rng,
}

/// Seconds to the end of the step of `dt` in which a timer of `timer` runs
/// out, as `fight` counts it down; None for one too far off to count.
fn steps_to(mut timer: f32, dt: f32) -> Option<f32> {
    if dt <= 0.0 || timer > 1000.0 * dt {
        return None;
    }
    let mut steps = 0;
    while timer > 0.0 {
        timer -= dt;
        steps += 1;
    }
    Some(steps as f32 * dt)
}

impl Expedition {
    pub fn new(data: &Data, plan: Plan, party: &[Horse]) -> Expedition {
        let rules = &data.rules;
        let road = rules.travel_secs(party.iter().map(|horse| horse.stats().speed));
        let party = party
            .iter()
            .map(|horse| {
                let stats = horse.stats();
                Fighter {
                    name: horse.name.clone(),
                    ability: horse.ability,
                    health: stats.health,
                    max_health: stats.health,
                    attack: stats.attack,
                    defense: stats.defense,
                    elemental: stats.elemental,
                    speed: stats.speed,
                    deeds: Deeds::default(),
                    attack_interval: rules.attack_interval(stats.speed),
                    // Set when a fight opens.
                    attack_timer: f32::MAX,
                    ability_timer: {
                        let ability = &data.abilities[horse.ability];
                        ability.first.unwrap_or(ability.cooldown)
                    },
                }
            })
            .collect();

        Expedition {
            // Kept apart from the plan's rolls, so the trail does not change
            // with how the fight happens to go.
            rng: Rng::new(plan.seed ^ 0xC0FF_EE00_F00D_5EED),
            loot: Rng::new(plan.seed ^ 0x4B11_4B11_4B11_4B11),
            // The horses' cadence has its own rolls too, so the enemies'
            // choices of whom to strike are not moved by it.
            cadence: Rng::new(plan.seed ^ 0x0CAD_E4CE_0CAD_E4CE),
            plan,
            party,
            foes: Vec::new(),
            encounter: 0,
            phase: Phase::Travelling,
            approach: 0.0,
            ward: 0.0,
            ward_reduction: 0.0,
            coins: 0,
            log: vec!["The party sets out.".into()],
            kills: Vec::new(),
            hits: Vec::new(),
            casts: Vec::new(),
            drops: Vec::new(),
            events: Vec::new(),
            outcome: None,
            travel: road,
            road,
            strong_hits: 0,
            resisted_hits: 0,
        }
    }

    /// Seconds the road between two encounters takes this party (#355).
    pub fn road_secs(&self) -> f32 {
        self.road
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
                    id: def.id.clone(),
                    name: def.name.clone(),
                    element: spawn.element,
                    health,
                    max_health: health,
                    attack: (def.attack as f32 * scale).round() as i32,
                    defense: def.defense,
                    attack_interval: def.attack_interval,
                    attack_timer: def.attack_interval,
                    coins: def.coins,
                    gear: def.gear,
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
        self.open_fight(&data.rules);
        self.events.push(Event::Fight);
        if self.encounter + 1 == self.plan.encounters.len() {
            self.events.push(Event::Boss);
            self.approach = data.rules.final_approach;
        }
        self.phase = Phase::Fighting;
    }

    /// Each fight opens out of step (#354): the horses' first hits come one
    /// after another, `opening_stagger` apart, in an order rolled for the
    /// fight. Only the ordinary hits start again; the abilities' timers run
    /// on from the last fight.
    fn open_fight(&mut self, rules: &Rules) {
        let mut order: Vec<usize> = (0..self.party.len()).collect();
        self.cadence.shuffle(&mut order);
        for (place, &horse) in order.iter().enumerate() {
            self.party[horse].attack_timer = rules.opening + rules.opening_stagger * place as f32;
        }
    }

    /// When an ordinary hit by `who` lands, for the presentation to wind it
    /// up ahead of it (#15): seconds to the end of the step of `dt` it lands
    /// in, counted as `tick` counts them. None outside a fight and for a
    /// body that is down. The hit is already rolled, so this only reads.
    pub fn swing_in(&self, who: Struck, dt: f32) -> Option<f32> {
        if self.phase != Phase::Fighting {
            return None;
        }
        let timer = match who {
            Struck::Horse(horse) => self.party.get(horse).filter(|f| f.health > 0)?.attack_timer,
            Struck::Foe { slot, .. } => self.foes.get(slot).filter(|f| f.health > 0)?.attack_timer,
        };
        Some(steps_to(self.approach, dt)? + steps_to(timer, dt)?)
    }

    /// When a horse's ability is next used, for the presentation to wind it
    /// up ahead of it (#18), counted as `swing_in` counts. None outside a
    /// fight and for a horse that is down.
    pub fn cast_in(&self, horse: usize, dt: f32) -> Option<f32> {
        if self.phase != Phase::Fighting {
            return None;
        }
        let timer = self.party.get(horse).filter(|f| f.health > 0)?.ability_timer;
        Some(steps_to(self.approach, dt)? + steps_to(timer, dt)?)
    }

    /// The time to a horse's next ordinary hit: its own interval, a little
    /// sooner or later each swing (#354).
    fn next_swing(&mut self, rules: &Rules, horse: usize) -> f32 {
        let jitter = rules.attack_jitter * (self.cadence.unit() * 2.0 - 1.0);
        self.party[horse].attack_interval * (1.0 + jitter)
    }

    fn fight(&mut self, data: &Data, dt: f32) {
        // The final enemy is still coming up (#19): the step that spends
        // the approach is spent on it whole.
        if self.approach > 0.0 {
            self.approach -= dt;
            return;
        }
        let rules = &data.rules;
        self.ward = (self.ward - dt).max(0.0);

        for i in 0..self.party.len() {
            if self.party[i].health <= 0 {
                continue;
            }
            self.party[i].attack_timer -= dt;
            self.party[i].ability_timer -= dt;

            if self.party[i].attack_timer <= 0.0 {
                self.party[i].attack_timer += self.next_swing(rules, i);
                // A plain hit has no element (vision.md, "Combat rules
                // settled"): its elemental part is neutral.
                if let Some(target) = self.target() {
                    let (attack, elemental) = self.power(i);
                    self.strike(data, i, target, attack, elemental, None, "hits");
                }
            }

            if self.party[i].ability_timer <= 0.0 {
                self.events.push(Event::Ability);
                self.casts.push(Used { horse: i, ability: self.party[i].ability });
                let (attack, elemental) = self.power(i);
                let ability = &data.abilities[self.party[i].ability];
                let (element, verb) = (ability.element, format!("{} on", ability.named()));
                self.party[i].ability_timer += ability.cooldown;
                match ability.kind {
                    AbilityKind::Shot(multiplier) => {
                        if let Some(target) = self.target() {
                            self.strike(data, i, target, attack * multiplier, elemental, element, &verb);
                        }
                    }
                    AbilityKind::Burst(multiplier) => {
                        for target in 0..self.foes.len() {
                            if self.foes[target].health > 0 {
                                self.strike(data, i, target, 0.0, elemental * multiplier, element, &verb);
                            }
                        }
                    }
                    AbilityKind::Ward(secs, reduction) => {
                        self.ward = secs;
                        self.ward_reduction = reduction;
                        self.party[i].deeds.wards += 1;
                        self.log
                            .push(format!("{} raises a {}.", self.party[i].name, ability.named()));
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
            let struck = *self.rng.pick(&standing);
            let victim = &mut self.party[struck];
            let mut damage = (self.foes[j].attack - victim.defense).max(1) as f32;
            let warded = self.ward > 0.0;
            if warded {
                damage *= 1.0 - self.ward_reduction;
            }
            let damage = damage.round().max(1.0) as i32;
            victim.deeds.taken += damage.min(victim.health);
            victim.health = (victim.health - damage).max(0);
            self.log.push(format!(
                "{} hits {} for {}{}",
                self.foes[j].name,
                victim.name,
                damage,
                if warded { " (Warded)" } else { "" }
            ));
            self.hits.push(Hit {
                struck: Struck::Horse(struck),
                by: Struck::Foe {
                    slot: j,
                    max_health: self.foes[j].max_health,
                },
                damage,
                element: None,
                kind: if warded { HitKind::Warded } else { HitKind::Plain },
            });
            if victim.health == 0 {
                victim.deeds.fell = true;
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
        self.travel = self.road;
        self.phase = Phase::Travelling;
    }

    fn finish(&mut self, data: &Data, won: bool) {
        let cleared = if won {
            self.plan.encounters.len()
        } else {
            self.encounter
        };
        // The trail's own piece, the plan's or at the trail's odds: the only
        // one that can be of the higher tiers.
        let drop = (cleared > 0).then(|| {
            self.plan.piece.clone().unwrap_or_else(|| {
                let mut rng = Rng::new(self.plan.seed ^ 0x100F_100F_100F_100F);
                roll_gear(data, &mut rng, won, self.plan.level)
            })
        });
        // A trail is won in the tick its last enemy falls, so that kill is
        // still here, not yet taken.
        if won
            && let Some(kill) = self.kills.last_mut()
            && let Some(drop) = &drop
        {
            kill.gear.push(drop.clone());
        }
        self.drops.extend(drop);
        if won {
            self.events.push(Event::Win);
        }
        self.phase = Phase::Finished;
        self.outcome = Some(Outcome {
            won,
            cleared,
            total: self.plan.encounters.len(),
            strong_hits: self.strong_hits,
            resisted_hits: self.resisted_hits,
            coins: self.coins,
            drops: self.drops.clone(),
            deeds: self.party.iter().map(|fighter| fighter.deeds).collect(),
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
    /// is scaled by the triangle, by the hit's element: its ability's, or
    /// none for a plain hit, which is neutral.
    #[allow(clippy::too_many_arguments)]
    fn strike(
        &mut self,
        data: &Data,
        horse: usize,
        target: usize,
        physical: f32,
        elemental: f32,
        element: Option<Element>,
        verb: &str,
    ) {
        let rules = &data.rules;
        let foe = &mut self.foes[target];
        let multiplier = rules.multiplier(element, foe.element);
        let damage = (physical - foe.defense as f32).max(0.0) + elemental * multiplier;
        let damage = damage.round().max(1.0) as i32;
        let deeds = &mut self.party[horse].deeds;
        deeds.dealt += damage.min(foe.health);
        foe.health = (foe.health - damage).max(0);

        let mut note = "";
        let mut kind = HitKind::Plain;
        if elemental > 0.0 && multiplier > 1.0 {
            self.strong_hits += 1;
            deeds.strong_hits += 1;
            note = "  Strong!";
            kind = HitKind::Strong;
        } else if elemental > 0.0 && multiplier < 1.0 {
            self.resisted_hits += 1;
            deeds.resisted_hits += 1;
            self.events.push(Event::Resisted);
            note = "  Resisted";
            kind = HitKind::Resisted;
        }
        self.hits.push(Hit {
            struck: Struck::Foe {
                slot: target,
                max_health: foe.max_health,
            },
            by: Struck::Horse(horse),
            damage,
            element: element.filter(|_| elemental > 0.0),
            kind,
        });
        self.log.push(format!(
            "{} {} {} for {}{}",
            self.party[horse].name, verb, foe.name, damage, note
        ));

        if foe.health == 0 {
            self.party[horse].deeds.kills += 1;
            let coins = self.rng.range(foe.coins.0, foe.coins.1);
            self.coins += coins;
            self.log
                .push(format!("{} is defeated (+{} coins)", foe.name, coins));
            self.events.push(Event::Kill);
            let pieces = self.loot.range(foe.gear.0 as i32, foe.gear.1 as i32);
            let gear: Vec<Gear> = (0..pieces)
                .map(|_| roll_piece(data, &mut self.loot, Odds::Kill, self.plan.level))
                .collect();
            self.drops.extend(gear.iter().cloned());
            self.kills.push(Kill {
                foe: target,
                element: foe.element,
                max_health: foe.max_health,
                coins,
                fountain: foe.fountain.clone(),
                death: foe.death.clone(),
                gear,
            });
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::sim::data::{AbilityDef, Slot, Stats};
    use crate::sim::loot::score;

    const TICK: f32 = 0.1;

    fn party(data: &Data) -> Vec<Horse> {
        Horse::party(&data)
    }

    fn run(data: &Data, seed: u64, level: u32) -> Expedition {
        run_with(data, seed, level, &party(data))
    }

    fn run_with(data: &Data, seed: u64, level: u32, party: &[Horse]) -> Expedition {
        walk(data, Plan::generate(data, seed, level), party)
    }

    /// A trail walked to its end.
    fn walk(data: &Data, plan: Plan, party: &[Horse]) -> Expedition {
        let seed = plan.seed;
        let mut expedition = Expedition::new(data, plan, party);
        // Ten minutes of game time is far past any real run.
        for _ in 0..6000 {
            expedition.tick(data, TICK);
            if expedition.phase == Phase::Finished {
                return expedition;
            }
        }
        panic!("seed {seed} did not finish");
    }

    /// A hand-made trail, with the starting party, standing in its first
    /// encounter.
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
            piece: None,
        };
        let mut expedition = Expedition::new(data, plan, &party(data));
        expedition.start_encounter(data);
        // Its last encounter is a final one; these are tests of the fight,
        // so it starts at the first blow. The approach has its own test.
        expedition.approach = 0.0;
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
            assert_eq!(rules.multiplier(Some(attack), strong), 1.5);
            assert_eq!(rules.multiplier(Some(attack), weak), 0.75);
            assert_eq!(rules.multiplier(Some(attack), attack), 1.0);
            // A hit with no element, a plain one, is neutral (#357).
            assert_eq!(rules.multiplier(None, attack), 1.0);
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
            let a = run(&data, seed, 0);
            let b = run(&data, seed, 0);
            assert_eq!(a.plan, b.plan);
            assert_eq!(a.outcome, b.outcome);
            assert_eq!(a.log, b.log);
        }
        assert_ne!(
            Plan::generate(&data, 1, 0).encounters,
            Plan::generate(&data, 2, 0).encounters
        );
    }

    /// The first trail should be winnable but not a formality, and which
    /// abilities the horses carry should matter (#357). The choices are the
    /// parties whose first two horses carry any of the abilities that deal
    /// damage and whose third keeps the Ward: the best of them for a seed
    /// against the worst. Run with `--nocapture` to see the rates.
    #[test]
    fn first_trail_balance() {
        let data = Data::load();
        let damage: Vec<usize> = (0..data.abilities.len())
            .filter(|&a| !matches!(data.abilities[a].kind, AbilityKind::Ward(..)))
            .collect();
        let ward = ability(&data, "ward");
        let choices: Vec<[usize; 3]> =
            damage.iter().flat_map(|&a| damage.iter().map(move |&b| [a, b, ward])).collect();
        let seeds = 300;
        let (mut wins, mut best_wins, mut worst_wins, mut starting, mut ticks) = (0, 0, 0, 0, 0usize);
        for seed in 0..seeds {
            let won: Vec<bool> = choices
                .iter()
                .map(|abilities| {
                    let mut horses = party(&data);
                    for (horse, &ability) in horses.iter_mut().zip(abilities) {
                        horse.ability = ability;
                    }
                    let run = run_with(&data, seed, 0, &horses);
                    ticks += run.log.len();
                    run.outcome.unwrap().won
                })
                .collect();
            wins += won.iter().filter(|&&w| w).count();
            best_wins += won.iter().any(|&w| w) as usize;
            worst_wins += won.iter().all(|&w| w) as usize;
            starting += run(&data, seed, 0).outcome.unwrap().won as usize;
        }
        let runs = seeds as usize * choices.len();
        let rate = wins as f32 / runs as f32;
        let best = best_wins as f32 / seeds as f32;
        let worst = worst_wins as f32 / seeds as f32;
        let starting = starting as f32 / seeds as f32;
        println!("win rate: any abilities {rate:.2}, best {best:.2}, worst {worst:.2}, the starting party {starting:.2}");
        println!("log lines per run: {}", ticks / runs);
        assert!((0.25..=0.75).contains(&rate), "win rate {rate}");
        assert!((0.25..=0.75).contains(&starting), "the starting party wins {starting}");
        // 0.75 when the abilities came in (#357): with the party element it was 0.5.
        assert!(best - worst >= 0.5, "the abilities' elements barely matter");
    }

    /// Every set of abilities the party may carry: each horse any ability.
    fn every_set_of_abilities(data: &Data) -> Vec<Vec<Horse>> {
        let n = data.abilities.len();
        (0..n * n * n)
            .map(|i| {
                let mut horses = party(data);
                for (k, horse) in horses.iter_mut().enumerate() {
                    horse.ability = i / n.pow(k as u32) % n;
                }
                horses
            })
            .collect()
    }

    /// A new game's first trail is one plan (#294): the same fights,
    /// elements and piece for every seed, no spring, and level 0.
    #[test]
    fn the_first_trail_is_the_same_whatever_the_seed() {
        let data = Data::load();
        let one = Plan::first(&data, 0);
        assert_eq!(one.level, 0);
        assert_eq!(one.bonus_after, None);
        assert!(one.piece.is_some());
        for seed in 1..200 {
            let other = Plan::first(&data, seed);
            assert_eq!((&other.encounters, &other.piece), (&one.encounters, &one.piece), "seed {seed}");
            assert_eq!(other.bonus_after, None);
        }
        // The trail after it is rolled, as every later one is.
        let second = Plan::generate(&data, 1, 1);
        assert_eq!(second.piece, None);
        assert_ne!(second.encounters, one.encounters);
    }

    /// The first trail cannot be lost, by tuning alone (#289, question 3):
    /// under the ordinary rules, with every set of abilities the party may
    /// carry and over many seeds, every run is won. That no horse falls
    /// (#293, call 2) is held for the starting party, below (the owner,
    /// 2026-10-19, #371): with some sets a horse reaches the boss with
    /// almost nothing left, and a rare fall there is accepted. Run with
    /// `--nocapture` to see how often one falls.
    #[test]
    fn the_first_trail_is_won_with_every_set_of_abilities() {
        let data = Data::load();
        let sets = every_set_of_abilities(&data);
        let seeds = 100;
        let mut falls = 0;
        for horses in &sets {
            for seed in 0..seeds {
                let run = walk(&data, Plan::first(&data, seed), horses);
                let abilities: Vec<&str> = horses.iter().map(|h| data.abilities[h.ability].id.as_str()).collect();
                assert!(run.outcome.as_ref().unwrap().won, "seed {seed}, {abilities:?}: lost");
                falls += run.party.iter().any(|fighter| fighter.health <= 0) as usize;
            }
        }
        println!("first trail: a horse fell in {falls} of {} runs", sets.len() * seeds as usize);
    }

    /// The starting party, on the first trail: no horse falls (#293, call
    /// 2), each of its three abilities fires, Ember Shot is resisted by the
    /// Toad (#293, call 4), and the
    /// trail's piece is the plan's, an upgrade for a bare horse (#293,
    /// call 3), dropped last by the boss.
    #[test]
    fn the_first_trail_shows_every_ability_and_a_resist() {
        let data = Data::load();
        let horses = party(&data);
        let piece = Plan::first(&data, 0).piece.unwrap();
        assert_eq!(data.gear.rarities[piece.rarity].name, "Rare");
        // Above any tier a kill throws, so it stands out by its look alone.
        let thrown = data.gear.rarities.iter().rposition(|r| r.kill_weight > 0).unwrap();
        assert!(piece.rarity > thrown);
        // Worn, it is more than a bare horse has.
        let mut worn = horses[0].clone();
        worn.gear[piece.slot.index()] = Some(piece.clone());
        assert!(score(&data, worn.stats()) > score(&data, horses[0].stats()));

        let mut least = vec![i32::MAX; horses.len()];
        for seed in 0..1000 {
            let run = walk(&data, Plan::first(&data, seed), &horses);
            for (horse, least) in least.iter_mut().enumerate() {
                let fighter = &run.party[horse];
                assert!(run.casts.iter().any(|used| used.horse == horse), "seed {seed}: {} never cast", fighter.name);
                assert!(fighter.health > 0, "seed {seed}: {} fell", fighter.name);
                *least = (*least).min(fighter.health);
            }
            assert!(run.events.contains(&Event::Resisted), "seed {seed}: nothing resisted");
            let outcome = run.outcome.as_ref().unwrap();
            assert!(outcome.won, "seed {seed}");
            assert_eq!(outcome.trail_piece(), Some(&piece), "seed {seed}");
            assert_eq!(run.kills.last().unwrap().gear.last(), Some(&piece), "seed {seed}");
        }
        println!("the starting party's least health left over 1000 seeds: {least:?}");
    }

    /// Every enemy brought down is reported once, with the coins it dropped.
    #[test]
    fn every_kill_is_reported_with_its_coins() {
        let data = Data::load();
        for seed in 0..100 {
            let run = run(&data, seed, 0);
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
            let run = run(&data, seed, 0);
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

    /// Every kill throws a piece or more of the chaff (#352). The trail's
    /// own piece comes out of the enemy whose fall won the trail, after its
    /// chaff; a lost trail's has no body to come out of. The outcome holds
    /// every piece, in the order they dropped.
    #[test]
    fn every_kill_drops_gear_and_the_winning_kill_the_trails_piece() {
        let data = Data::load();
        let chaff = |gear: &[Gear]| {
            !gear.is_empty() && gear.iter().all(|gear| data.gear.rarities[gear.rarity].kill_weight > 0)
        };
        let (mut won, mut lost) = (0, 0);
        for seed in 0..100 {
            let run = run(&data, seed, 0);
            let outcome = run.outcome.clone().unwrap();
            let mut thrown: Vec<Gear> = run.kills.iter().flat_map(|kill| kill.gear.clone()).collect();
            let Some((last, earlier)) = run.kills.split_last() else {
                assert!(!outcome.won);
                continue;
            };
            assert!(earlier.iter().all(|kill| chaff(&kill.gear)), "seed {seed}");
            if outcome.won {
                let (piece, rest) = last.gear.split_last().unwrap();
                assert!(chaff(rest), "seed {seed}");
                assert_eq!(Some(piece), outcome.trail_piece());
                won += 1;
            } else {
                assert!(chaff(&last.gear), "seed {seed}");
                thrown.extend(outcome.trail_piece().cloned());
                lost += 1;
            }
            assert_eq!(thrown, outcome.drops, "seed {seed}");
        }
        assert!(won > 0 && lost > 0);
    }

    #[test]
    fn drops_follow_the_seed() {
        let data = Data::load();
        let mut seen = 0;
        for seed in 0..200 {
            let a = run(&data, seed, 0).outcome.unwrap();
            let b = run(&data, seed, 0).outcome.unwrap();
            assert_eq!(a.drops, b.drops);
            assert_eq!(a.trail_piece().is_some(), a.cleared > 0);
            for gear in &a.drops {
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
            run.strike(&data, 0, target, physical, elemental, Some(Fire), "hits");
            let foe = &run.foes[target];
            dealt.push(foe.max_health - foe.health);
            assert_eq!(dealt[target], (5.0 + elemental * multiplier).round() as i32);
        }
        assert!(dealt[0] > dealt[1] && dealt[1] > dealt[2], "{dealt:?}");
        assert_eq!((run.strong_hits, run.resisted_hits), (1, 1));

        // Defense never turns a hit into healing, and a hit with nothing
        // elemental in it is neither strong nor resisted.
        let before = run.foes[0].health;
        run.strike(&data, 0, 0, defense * 0.5, 0.0, Some(Fire), "hits");
        assert_eq!(run.foes[0].health, before - 1);
        assert_eq!((run.strong_hits, run.resisted_hits), (1, 1));

        // A hit with no element, a plain one, is neutral against every
        // enemy (#357): its elemental part is as it is.
        for target in 0..3 {
            let before = run.foes[target].health;
            run.strike(&data, 0, target, physical, elemental, None, "hits");
            assert_eq!(before - run.foes[target].health, (5.0 + elemental).round() as i32);
            assert_eq!(run.hits.last().unwrap().element, None);
        }
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

    /// Under the Ward a hit lands for less, by its share in `abilities.ron`;
    /// the horse that carries it raises it without being told, and it runs
    /// out.
    #[test]
    fn the_ward_reduces_damage_taken_and_runs_out() {
        let data = Data::load();
        let (duration, reduction) = ward(&data);
        let cooldown = def(&data, "ward").cooldown;
        let toad: &[&[(&str, Element)]] = &[&[("mire_toad", Element::Ice)]];
        let mut plain = fighting(&data, 0, toad);
        hold_the_party(&mut plain);
        let mut warded = plain.clone();
        warded.ward = f32::MAX;
        warded.ward_reduction = reduction;

        // Long enough for the toad's first hit, too short for its second.
        let dt = plain.foes[0].attack_interval * 1.5;
        plain.tick(&data, dt);
        warded.tick(&data, dt);
        let (full, less) = (health_lost(&plain), health_lost(&warded));
        assert!(full > 1, "the toad's hit should be worth reducing");
        assert_eq!(less, ((full as f32 * (1.0 - reduction)).round() as i32).max(1));
        assert!(less < full);
        assert!(warded.log.last().unwrap().ends_with("(Warded)"));
        assert!(!plain.log.last().unwrap().ends_with("(Warded)"));

        let mut run = fighting(&data, 0, toad);
        assert_eq!(run.ward, 0.0);
        let mut raised = false;
        for _ in 0..(cooldown / TICK) as usize + 1 {
            run.tick(&data, TICK);
            assert!(run.ward <= duration);
            raised |= run.ward > 0.0;
        }
        assert!(raised, "the Ward was never raised");
        assert_eq!(run.ward_reduction, reduction);

        let mut fading = fighting(&data, 0, toad);
        hold_the_party(&mut fading);
        fading.ward = duration;
        fading.tick(&data, duration * 0.5);
        assert!(fading.ward > 0.0 && fading.ward < duration);
        fading.tick(&data, duration);
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
    /// encounter it brings back no piece of its own, only what its kills
    /// threw; lost further on it brings its piece, which no enemy drops.
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
            assert_eq!(outcome.trail_piece(), outcome.drops.last().filter(|_| cleared > 0));
            outcome.drops.len()
        };
        assert_eq!(lose(0), 0);
        assert_eq!(lose(1), 1);
        assert_eq!(lose(2), 1);
    }

    /// An ability's place in `abilities.ron`, by its id.
    fn ability(data: &Data, id: &str) -> usize {
        data.ability_index(id)
    }

    /// The horse of the starting party that carries this ability.
    fn horse_with(expedition: &Expedition, data: &Data, id: &str) -> usize {
        let ability = ability(data, id);
        expedition.party.iter().position(|f| f.ability == ability).unwrap()
    }

    fn def<'a>(data: &'a Data, id: &str) -> &'a AbilityDef {
        &data.abilities[ability(data, id)]
    }

    /// A Ward's seconds and the share it takes off, from `abilities.ron`.
    fn ward(data: &Data) -> (f32, f32) {
        match def(data, "ward").kind {
            AbilityKind::Ward(secs, reduction) => (secs, reduction),
            kind => panic!("the ward is {kind:?}"),
        }
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

    /// A Shot is one hit on the current target and no other, with the
    /// horse's attack times the Shot's multiplier and its elemental in the
    /// ability's element: Ember Shot's Fire, strong against Plant.
    #[test]
    fn a_shot_multiplies_the_attack_on_the_current_target_in_its_element() {
        let data = Data::load();
        let rules = &data.rules;
        let AbilityKind::Shot(multiplier) = def(&data, "ember_shot").kind else {
            panic!("Ember Shot is a Shot");
        };
        let mut run = fighting(
            &data,
            0,
            &[&[("corrupted_stag", Element::Plant), ("mire_toad", Element::Plant)]],
        );
        let shooter = horse_with(&run, &data, "ember_shot");
        only_the_ability_of(&mut run, shooter);
        run.tick(&data, TICK);

        let (attack, elemental) = run.power(shooter);
        let physical = attack * multiplier - run.foes[0].defense as f32;
        let expected = (physical + elemental * rules.strong_multiplier).round() as i32;
        assert_eq!(run.foes[0].max_health - run.foes[0].health, expected);
        assert_eq!(run.foes[1].health, run.foes[1].max_health);
        assert_eq!(lines_with(&run, "Ember Shot (Fire) on"), 1);
        assert_eq!(run.strong_hits, 1);
        assert_eq!(run.hits.last().unwrap().element, Some(Element::Fire));

        // It is worth more than a hit of the same element, by the attack alone.
        let before = run.foes[0].health;
        run.strike(&data, shooter, 0, attack, elemental, Some(Element::Fire), "hits");
        let ordinary = before - run.foes[0].health;
        assert_eq!(expected - ordinary, (attack * (multiplier - 1.0)).round() as i32);
    }

    /// A Burst hits every enemy still standing with the horse's elemental
    /// times its multiplier, in the ability's element by the triangle, past
    /// defense and with no attack in it: Frost Burst's Ice.
    #[test]
    fn a_burst_hits_every_standing_enemy_with_elemental_alone() {
        let data = Data::load();
        let rules = &data.rules;
        let AbilityKind::Burst(multiplier) = def(&data, "frost_burst").kind else {
            panic!("Frost Burst is a Burst");
        };
        use Element::*;
        let mut run = fighting(
            &data,
            0,
            &[&[
                ("corrupted_stag", Fire),
                ("mire_toad", Ice),
                ("thornling", Fire),
                ("mire_toad", Plant),
            ]],
        );
        assert!(run.foes[0].defense > 0, "the stag is the enemy with defense");
        // The thornling fell earlier in this fight.
        run.foes[2].health = 0;
        let burster = horse_with(&run, &data, "frost_burst");
        only_the_ability_of(&mut run, burster);
        run.tick(&data, TICK);

        let burst = run.power(burster).1 * multiplier;
        for (target, multiplier) in [(0, rules.strong_multiplier), (1, 1.0), (3, rules.weak_multiplier)] {
            let foe = &run.foes[target];
            assert_eq!(foe.max_health - foe.health, (burst * multiplier).round() as i32, "{}", foe.name);
        }
        assert_eq!(lines_with(&run, "Frost Burst (Ice) on"), 3);
        assert!(run.hits.iter().all(|hit| hit.element == Some(Ice)));
        assert_eq!((run.strong_hits, run.resisted_hits), (1, 1));
        assert!(run.kills.is_empty(), "a fallen enemy is not hit again");
    }

    /// Each ability comes round once per cooldown, the first time after one
    /// whole cooldown unless it says otherwise.
    #[test]
    fn the_shot_and_the_burst_keep_their_cooldowns() {
        let data = Data::load();
        for id in ["ember_shot", "frost_burst"] {
            let ability = def(&data, id);
            let (cooldown, line) = (ability.cooldown, format!("{} on", ability.named()));
            let line = line.as_str();
            let mut run = fighting(&data, 0, &[&[("corrupted_stag", Element::Fire)]]);
            let horse = horse_with(&run, &data, id);
            let first = run.party[horse].ability_timer;
            assert_eq!(first, ability.first.unwrap_or(cooldown));
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

    /// Every use of an ability is told once, by which ability it was, so its
    /// sound can be played (#358); a plain hit is no use of one.
    #[test]
    fn every_use_of_an_ability_is_told_once() {
        let data = Data::load();
        for (index, ability) in data.abilities.iter().enumerate() {
            let mut run = fighting(&data, 0, &[&[("corrupted_stag", Element::Fire)]]);
            run.party[0].ability = index;
            only_the_ability_of(&mut run, 0);
            run.foes[0].health = i32::MAX;
            run.tick(&data, TICK);
            assert_eq!(run.casts, vec![Used { horse: 0, ability: index }], "{}", ability.id);

            run.casts.clear();
            run.party[0].ability_timer = f32::MAX;
            run.party[0].attack_timer = TICK * 0.5;
            run.tick(&data, TICK);
            assert!(!run.hits.is_empty() && run.casts.is_empty(), "{}", ability.id);
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
            for ability in 0..data.abilities.len() {
                let mut horses = party(&data);
                horses[0].ability = ability;
                assert_eq!(run_with(&data, seed, 0, &horses).plan, plan);
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
                look: Default::default(),
                rarity: 0,
                stats: Stats {
                    health: 20,
                    attack: 3,
                    defense: 2,
                    elemental: 3,
                    speed: 0,
                },
            });
        }

        let plan = Plan::generate(&data, 1, 0);
        let expedition = Expedition::new(&data, plan, &geared);
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
                .filter(|&seed| run_with(&data, seed, 0, party).outcome.unwrap().won)
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
            let mut run = Expedition::new(&data, plan, &party(&data));
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
                assert_eq!(taken.last().unwrap().gear.last(), outcome.trail_piece());
                let thrown: Vec<&Gear> = taken.iter().flat_map(|kill| &kill.gear).collect();
                assert_eq!(thrown, outcome.drops.iter().collect::<Vec<_>>());
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

    /// The road to each encounter takes the party's road time (#355), and
    /// no ability's timer runs on it: an ability is no nearer at the end of
    /// the road. The ordinary hits start again at each fight's opening (#354).
    #[test]
    fn the_road_takes_its_time_and_no_timer_runs_on_it() {
        let data = Data::load();
        let rules = &data.rules;
        let plan = Plan::generate(&data, 3, 0);
        let mut run = Expedition::new(&data, plan, &party(&data));
        let timers = |run: &Expedition| -> Vec<(f32, f32)> {
            run.party.iter().map(|f| (f.attack_timer, f.ability_timer)).collect()
        };
        let abilities = |timers: &[(f32, f32)]| -> Vec<f32> {
            timers.iter().map(|&(_, ability)| ability).collect()
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
        let warder = horse_with(&run, &data, "ward");
        assert_eq!(Some(run.party[warder].ability_timer), def(&data, "ward").first);
        // The first of the opening's hits, and the last.
        let opened = |run: &Expedition| {
            let last = rules.opening + rules.opening_stagger * (run.party.len() - 1) as f32;
            run.party.iter().all(|f| (rules.opening..=last).contains(&f.attack_timer))
        };

        let before = timers(&run);
        let road = run.road_secs();
        assert!((walk(&mut run) - road).abs() <= TICK);
        assert_eq!(run.phase, Phase::Fighting);
        assert_eq!(run.foes.len(), run.plan.encounters[0].len());
        assert_eq!(lines_with(&run, "Encounter 1/"), 1);
        assert_eq!(abilities(&timers(&run)), abilities(&before));
        assert!(opened(&run), "{:?}", timers(&run));

        // Halfway to the next hits, the fight is won.
        for fighter in &mut run.party {
            fighter.attack_timer = 0.5;
        }
        run.foes.clear();
        run.encounter_cleared(&data);
        assert_eq!((run.phase, run.encounter), (Phase::Travelling, 1));
        assert!((walk(&mut run) - road).abs() <= TICK);
        assert_eq!(run.phase, Phase::Fighting);
        assert_eq!(lines_with(&run, "Encounter 2/"), 1);
        assert_eq!(abilities(&timers(&run)), abilities(&before));
        assert!(opened(&run), "{:?}", timers(&run));
    }

    /// An ordinary hit lands on the first enemy still standing and no other.
    /// How often each horse hits is `each_horse_hits_at_its_own_cadence`.
    #[test]
    fn each_horse_hits_the_first_standing_enemy() {
        let data = Data::load();
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

        let all = |run: &Expedition| (0..3).map(|foe| hits(run, foe)).sum::<usize>();
        let until = |run: &mut Expedition, total: usize| {
            for _ in 0..1000 {
                if all(run) >= total {
                    return;
                }
                run.tick(&data, TICK);
            }
            panic!("the hits never came");
        };

        until(&mut run, 3 * horses);
        assert_eq!((hits(&run, 0), hits(&run, 2)), (0, 0));
        assert_eq!(run.foes[2].health, run.foes[2].max_health);

        // The toad falls, and the next hits go to the sprite.
        run.foes[1].health = 0;
        let on_toad = hits(&run, 1);
        until(&mut run, on_toad + horses);
        assert_eq!((hits(&run, 0), hits(&run, 1)), (0, on_toad));
        assert!(hits(&run, 2) >= horses);
        assert_eq!(run.phase, Phase::Fighting);
    }

    /// The ticks on which each horse made an ordinary hit, horse by horse,
    /// over a long fight nobody can win or lose, opened with `seed`.
    fn swings(data: &Data, seed: u64) -> Vec<Vec<usize>> {
        let mut run = fighting(data, 0, &[&[("mire_toad", Element::Fire)]]);
        run.cadence = Rng::new(seed);
        run.open_fight(&data.rules);
        for fighter in &mut run.party {
            fighter.ability_timer = f32::MAX;
        }
        let foe = &mut run.foes[0];
        foe.attack_timer = f32::MAX;
        foe.health = i32::MAX;
        foe.max_health = i32::MAX;
        let mut swings = vec![Vec::new(); run.party.len()];
        for tick in 0..2000 {
            let before: Vec<f32> = run.party.iter().map(|f| f.attack_timer).collect();
            run.tick(data, TICK);
            for (horse, fighter) in run.party.iter().enumerate() {
                if fighter.attack_timer > before[horse] {
                    swings[horse].push(tick);
                }
            }
        }
        swings
    }

    /// `swing_in` says when each ordinary hit lands, horse's and enemy's,
    /// before it is dealt (#15): a hit by a body lands on the step it said,
    /// and on no other. Only reading it changes nothing.
    #[test]
    fn swing_in_says_the_step_each_hit_lands_on() {
        let data = Data::load();
        for seed in 0..20 {
            let mut run = fighting(&data, 0, &[&[("mire_toad", Element::Fire), ("hollow_wolf", Element::Ice)]]);
            run.cadence = Rng::new(seed);
            run.open_fight(&data.rules);
            for fighter in &mut run.party {
                fighter.ability_timer = f32::MAX;
                fighter.health = i32::MAX / 2;
            }
            for foe in &mut run.foes {
                foe.health = i32::MAX / 2;
                foe.max_health = i32::MAX / 2;
            }
            let bodies: Vec<Struck> = (0..run.party.len())
                .map(Struck::Horse)
                .chain((0..run.foes.len()).map(|slot| Struck::Foe { slot, max_health: i32::MAX / 2 }))
                .collect();
            let mut seen = 0;
            for tick in 0..600 {
                let due: Vec<f32> = bodies.iter().map(|&who| run.swing_in(who, TICK).unwrap()).collect();
                run.tick(&data, TICK);
                for (&who, due) in bodies.iter().zip(due) {
                    let dealt = run.hits.iter().any(|hit| hit.by == who);
                    let now = (due - TICK).abs() < 1e-4;
                    assert_eq!(dealt, now, "seed {seed}, tick {tick}, {who:?}: due in {due}");
                    seen += dealt as usize;
                }
                run.hits.clear();
            }
            assert!(seen > 600 / 10, "seed {seed}: {seen} hits");
        }
        let run = fighting(&data, 0, &[&[("mire_toad", Element::Fire)]]);
        assert_eq!(run.swing_in(Struck::Horse(9), TICK), None);
        let mut done = run.clone();
        done.phase = Phase::Finished;
        assert_eq!(done.swing_in(Struck::Horse(0), TICK), None);
    }

    /// `cast_in` says when each horse's ability is used, before it is (#18):
    /// on the step it said and on no other, for every horse and so every
    /// kind of ability; and never for a horse that is down.
    #[test]
    fn cast_in_says_the_step_each_ability_is_used_on() {
        let data = Data::load();
        let mut run = fighting(&data, 0, &[&[("mire_toad", Element::Fire)]]);
        for fighter in &mut run.party {
            fighter.health = i32::MAX / 2;
        }
        run.foes[0].health = i32::MAX / 2;
        let mut seen = vec![0; run.party.len()];
        for tick in 0..1200 {
            let due: Vec<f32> = (0..run.party.len()).map(|horse| run.cast_in(horse, TICK).unwrap()).collect();
            run.tick(&data, TICK);
            for (horse, due) in due.into_iter().enumerate() {
                let used = run.casts.iter().any(|cast| cast.horse == horse);
                assert_eq!(used, (due - TICK).abs() < 1e-4, "tick {tick}, horse {horse}: due in {due}");
                seen[horse] += used as usize;
            }
            run.casts.clear();
            run.hits.clear();
        }
        assert!(seen.iter().all(|&n| n >= 5), "{seen:?}");
        run.party[1].health = 0;
        assert_eq!(run.cast_in(1, TICK), None);
        assert_eq!(run.cast_in(9, TICK), None);
    }

    /// The final enemy comes up before its fight's first blow (#19): its
    /// fight opens with `final_approach` to spend, and an ordinary one with
    /// none; while it lasts nothing is struck or cast; and `swing_in` and
    /// `cast_in` count it, so the first blow after it lands on the step
    /// they said as it began.
    #[test]
    fn the_final_enemy_comes_up_before_the_first_blow() {
        let data = Data::load();
        let approach = data.rules.final_approach;
        let mut finals = 0;
        for seed in 0..30 {
            let plan = Plan::generate(&data, seed, 1);
            let mut run = Expedition::new(&data, plan, &party(&data));
            let mut was = Phase::Travelling;
            // The step the first blow was said to land on, and the first
            // ability, as the approach began.
            let mut first: Option<(usize, usize)> = None;
            for tick in 0..20_000 {
                run.tick(&data, TICK);
                if run.phase == Phase::Finished {
                    break;
                }
                let last = run.encounter + 1 == run.plan.encounters.len();
                if was == Phase::Travelling && run.phase == Phase::Fighting {
                    let want = if last { approach } else { 0.0 };
                    assert_eq!(run.approach, want, "seed {seed}, encounter {}", run.encounter);
                    if last {
                        finals += 1;
                        let horses = 0..run.party.len();
                        let blow = horses.clone().filter_map(|h| run.swing_in(Struck::Horse(h), TICK));
                        let blow = blow.fold(f32::MAX, f32::min);
                        let cast = horses.filter_map(|h| run.cast_in(h, TICK)).fold(f32::MAX, f32::min);
                        assert!(blow > approach && cast > approach, "seed {seed}: {blow} {cast}");
                        let at = |secs: f32| tick + (secs / TICK).round() as usize;
                        first = Some((at(blow), at(cast)));
                    }
                }
                if run.approach > 0.0 {
                    assert!(run.hits.is_empty() && run.casts.is_empty(), "seed {seed}");
                }
                if let Some((blow, cast)) = first {
                    // An ordinary hit has no element; a shot's has.
                    let struck = run.hits.iter().any(|hit| matches!(hit.by, Struck::Horse(_)) && hit.element.is_none());
                    if tick <= blow {
                        assert_eq!(struck, tick == blow, "seed {seed}, tick {tick}, due {blow}");
                    }
                    if tick <= cast {
                        assert_eq!(!run.casts.is_empty(), tick == cast, "seed {seed}, tick {tick}, cast due {cast}");
                    }
                }
                was = run.phase;
                run.hits.clear();
                run.casts.clear();
                run.kills.clear();
            }
        }
        assert!(finals > 10, "{finals} final fights met");
    }

    /// Each horse hits at its own cadence (#354): a fight opens with the
    /// horses' first hits one after another, in an order that changes from
    /// fight to fight; each next hit comes the horse's own interval later,
    /// give or take `attack_jitter`; and the same seed plays the same fight.
    #[test]
    fn each_horse_hits_at_its_own_cadence() {
        let data = Data::load();
        let rules = &data.rules;
        let mut orders = std::collections::HashSet::new();
        for seed in 0..200 {
            let swings = swings(&data, seed);
            let mut first: Vec<(usize, usize)> =
                swings.iter().enumerate().map(|(horse, ticks)| (ticks[0], horse)).collect();
            first.sort();
            for pair in first.windows(2) {
                assert!(pair[0].0 < pair[1].0, "seed {seed}: the fight opens in step, {first:?}");
            }
            orders.insert(first.iter().map(|&(_, horse)| horse).collect::<Vec<_>>());

            for (horse, ticks) in swings.iter().enumerate() {
                let interval = rules.attack_interval(data.horses[horse].stats.speed);
                let (lo, hi) = (interval * (1.0 - rules.attack_jitter), interval * (1.0 + rules.attack_jitter));
                for gap in ticks.windows(2).map(|pair| (pair[1] - pair[0]) as f32 * TICK) {
                    assert!(lo - TICK - 1e-3 <= gap && gap <= hi + TICK + 1e-3, "seed {seed}, horse {horse}: {gap}");
                }
                let mean = (ticks[ticks.len() - 1] - ticks[0]) as f32 * TICK / (ticks.len() - 1) as f32;
                assert!((mean / interval - 1.0).abs() < 0.03, "seed {seed}, horse {horse}: {mean}");
            }
        }
        assert_eq!(orders.len(), 6, "{orders:?}");
        assert_eq!(swings(&data, 5), swings(&data, 5));
        assert_ne!(swings(&data, 5), swings(&data, 6));
    }

    /// Speed (#355): a faster horse hits more often, by `attack_tempo`, and
    /// its gear's speed counts as its own does. The party walks between
    /// encounters at its average speed, by `travel_distance`: one speed
    /// sets both.
    #[test]
    fn speed_sets_how_often_a_horse_hits_and_how_fast_the_party_walks() {
        let data = Data::load();
        let rules = &data.rules;
        let mut horses = party(&data);
        let slow = rules.attack_interval(horses[0].stats().speed);
        horses[0].gear[0] = Some(Gear {
            name: "Swift Shoes".into(),
            slot: Slot::ALL[0],
            look: Default::default(),
            rarity: 0,
            stats: Stats { speed: 10, ..Stats::default() },
        });
        let speed = horses[0].stats().speed;
        assert_eq!(speed, horses[0].base.speed + 10);
        let fast = rules.attack_interval(speed);
        assert!(fast < slow, "{fast} {slow}");
        assert!((fast * speed as f32 - rules.attack_tempo).abs() < 1e-4);

        let plan = Plan::generate(&data, 3, 0);
        let run = Expedition::new(&data, plan, &horses);
        assert_eq!(run.party[0].attack_interval, fast);
        assert_eq!(run.party[1].attack_interval, rules.attack_interval(horses[1].stats().speed));
        let speeds: Vec<i32> = horses.iter().map(|horse| horse.stats().speed).collect();
        let average = speeds.iter().sum::<i32>() as f32 / speeds.len() as f32;
        assert!((run.road_secs() - rules.travel_distance / average).abs() < 1e-4);
        let before = Expedition::new(&data, Plan::generate(&data, 3, 0), &party(&data));
        assert!(run.road_secs() < before.road_secs(), "a faster horse speeds the party");

        // Over a long fight the faster horse makes more hits.
        let mut slow_run = fighting(&data, 0, &[&[("mire_toad", Element::Fire)]]);
        let mut fast_run = slow_run.clone();
        fast_run.party[0].attack_interval = fast;
        let count = |run: &mut Expedition| {
            for fighter in &mut run.party {
                fighter.ability_timer = f32::MAX;
            }
            let foe = &mut run.foes[0];
            foe.attack_timer = f32::MAX;
            foe.health = i32::MAX;
            foe.max_health = i32::MAX;
            let mut hits = 0;
            for _ in 0..2000 {
                let before = run.party[0].attack_timer;
                run.tick(&data, TICK);
                hits += (run.party[0].attack_timer > before) as u32;
            }
            hits
        };
        let (slow_hits, fast_hits) = (count(&mut slow_run), count(&mut fast_run));
        assert!(fast_hits > slow_hits, "{fast_hits} {slow_hits}");
    }

    /// Over a whole trail the horses rarely hit on the same tick (#354): the
    /// share of the ticks with a hit that have two or more. Run with
    /// `--nocapture` to see it. In step, as before #354, it was all of them.
    #[test]
    fn over_a_trail_the_horses_rarely_hit_together() {
        let data = Data::load();
        let (mut with_a_hit, mut together) = (0, 0);
        for seed in 0..200 {
            let plan = Plan::generate(&data, seed, 0);
            let mut run = Expedition::new(&data, plan, &party(&data));
            while run.phase != Phase::Finished {
                let fighting = run.phase == Phase::Fighting;
                let before: Vec<f32> = run.party.iter().map(|f| f.attack_timer).collect();
                run.tick(&data, TICK);
                let hit = run.party.iter().zip(&before).filter(|(f, b)| f.attack_timer > **b).count();
                if fighting && hit > 0 {
                    with_a_hit += 1;
                    together += (hit > 1) as usize;
                }
            }
        }
        let share = together as f32 / with_a_hit as f32;
        println!("ticks with a hit: {with_a_hit}, with two or more: {together} ({share:.3})");
        assert!(share < 0.1, "{share}");
    }

    /// A kill's coins are within its enemy's range in `enemies.ron`, and over
    /// many kills they reach both ends of it.
    /// What each horse did (#311) adds up to what happened on the trail.
    #[test]
    fn the_horses_deeds_add_up_to_the_trail() {
        let data = Data::load();
        let (mut won, mut lost) = (0, 0);
        for seed in 0..200 {
            for level in [0, 6] {
                let run = run(&data, seed, level);
                let outcome = run.outcome.as_ref().unwrap();
                let deeds = &outcome.deeds;
                assert_eq!(deeds.len(), run.party.len());
                assert_eq!(deeds.iter().map(|d| d.kills as usize).sum::<usize>(), run.kills.len());
                assert_eq!(deeds.iter().map(|d| d.strong_hits).sum::<u32>(), outcome.strong_hits);
                assert_eq!(deeds.iter().map(|d| d.resisted_hits).sum::<u32>(), outcome.resisted_hits);
                let fallen: i32 = run.kills.iter().map(|kill| kill.max_health).sum();
                let dealt: i32 = deeds.iter().map(|d| d.dealt).sum();
                if outcome.won {
                    assert_eq!(dealt, fallen, "seed {seed}");
                    won += 1;
                } else {
                    assert!(dealt >= fallen, "seed {seed}");
                    lost += 1;
                }
                for (fighter, deeds) in run.party.iter().zip(deeds) {
                    assert_eq!(deeds.fell, fighter.health == 0, "seed {seed}");
                    // The spring heals, so a horse may have lost more than it is short of.
                    assert!(deeds.taken >= fighter.max_health - fighter.health, "seed {seed}");
                    let kind = data.abilities[fighter.ability].kind;
                    assert!(deeds.wards == 0 || matches!(kind, AbilityKind::Ward(..)));
                }
            }
        }
        assert!(won > 0 && lost > 0, "{won} won, {lost} lost");
    }

    /// Every hit that lands is told for its number (#249), in the order the
    /// fight log has it and for the damage it says; a horse's hit names the
    /// enemy it struck, an enemy's the horse, each names who dealt it, and
    /// the Strong and Resisted ones are the outcome's.
    #[test]
    fn every_hit_is_told_as_the_log_says_it() {
        let data = Data::load();
        let (mut foes, mut horses, mut bursts) = (0, 0, 0);
        for seed in 0..100 {
            let run = run(&data, seed, 3);
            let outcome = run.outcome.as_ref().unwrap();
            let logged: Vec<i32> = run
                .log
                .iter()
                .filter_map(|line| line.split(" for ").nth(1))
                .map(|rest| rest.split(|c: char| !c.is_ascii_digit()).next().unwrap().parse().unwrap())
                .collect();
            let told: Vec<i32> = run.hits.iter().map(|hit| hit.damage).collect();
            assert_eq!(told, logged, "seed {seed}");
            let lines = run.log.iter().filter(|line| line.contains(" for "));
            for (hit, line) in run.hits.iter().zip(lines) {
                assert!(hit.damage >= 1);
                match hit.struck {
                    Struck::Foe { slot, max_health } => {
                        foes += 1;
                        assert!(slot < 3 && max_health > 0);
                        assert!(!line.contains("Warded") && hit.kind != HitKind::Warded);
                        assert_eq!(hit.kind == HitKind::Strong, line.ends_with("Strong!"));
                        assert_eq!(hit.kind == HitKind::Resisted, line.ends_with("Resisted"));
                        // A plain hit has no element, and only an ability's
                        // hit is ever strong or resisted (#357).
                        let plain = line.contains(" hits ");
                        assert_eq!(plain, hit.element.is_none(), "{line}");
                        assert!(hit.kind == HitKind::Plain || !plain, "{line}");
                        bursts += line.contains("Frost Burst (Ice) on") as u32;
                        // The horse that dealt it is the one the line opens with.
                        let Struck::Horse(by) = hit.by else { panic!("{line}: by {:?}", hit.by) };
                        assert!(line.starts_with(&format!("{} ", run.party[by].name)), "{line}");
                    }
                    Struck::Horse(horse) => {
                        horses += 1;
                        assert!(line.contains(&format!(" hits {} for", run.party[horse].name)));
                        // An enemy dealt it, so the line opens with no horse's name.
                        let Struck::Foe { slot, max_health } = hit.by else { panic!("{line}: by {:?}", hit.by) };
                        assert!(slot < 3 && max_health > 0);
                        assert!(run.party.iter().all(|f| !line.starts_with(&format!("{} ", f.name))), "{line}");
                        assert_eq!(hit.kind == HitKind::Warded, line.ends_with("(Warded)"));
                        assert_eq!(hit.element, None);
                    }
                }
            }
            let count = |kind| run.hits.iter().filter(|hit| hit.kind == kind).count() as u32;
            assert_eq!((count(HitKind::Strong), count(HitKind::Resisted)), (outcome.strong_hits, outcome.resisted_hits));
        }
        assert!(foes > 0 && horses > 0 && bursts > 0, "{foes} {horses} {bursts}");
    }

    /// A kill's coins and its count of gear are each within its enemy's
    /// range, and over many seeds each comes up at both ends. The count is
    /// the same at a high trail level as at the first (#352: the early
    /// trails are no less generous).
    #[test]
    fn a_kill_drops_coins_and_gear_within_its_enemys_range() {
        let data = Data::load();
        let kill = |enemy: &str, seed: u64, level: u32| {
            let mut run = fighting(&data, level, &[&[(enemy, Element::Fire)]]);
            run.rng = Rng::new(seed);
            run.loot = Rng::new(seed);
            run.foes[0].health = 1;
            run.strike(&data, 0, 0, 1.0, 0.0, None, "hits");
            let kill = run.kills.pop().expect("the enemy fell");
            assert_eq!(run.coins, kill.coins);
            assert_eq!(run.drops, kill.gear);
            assert!(run.kills.is_empty());
            kill
        };
        for enemy in &data.enemies {
            let (lo, hi) = enemy.coins;
            let (glo, ghi) = (enemy.gear.0 as usize, enemy.gear.1 as usize);
            let (mut fewest, mut most) = (i32::MAX, i32::MIN);
            let (mut fewest_gear, mut most_gear) = (usize::MAX, 0);
            for seed in 0..200 {
                let low = kill(&enemy.id, seed, 0);
                assert!((lo..=hi).contains(&low.coins), "{}: {}", enemy.id, low.coins);
                let pieces = low.gear.len();
                assert!((glo..=ghi).contains(&pieces), "{}: {pieces} pieces", enemy.id);
                assert_eq!(kill(&enemy.id, seed, 6).gear.len(), pieces, "{}", enemy.id);
                fewest = fewest.min(low.coins);
                most = most.max(low.coins);
                fewest_gear = fewest_gear.min(pieces);
                most_gear = most_gear.max(pieces);
            }
            assert_eq!((fewest, most), (lo, hi), "{}", enemy.id);
            assert_eq!((fewest_gear, most_gear), (glo, ghi), "{}", enemy.id);
        }
    }

    /// The rates of #352, rolled over many trails from fixed seeds. Every
    /// trail brings back many pieces, nearly all chaff; the higher tiers
    /// come no more often per trail than when a trail brought back its own
    /// piece alone, within a tenth; and the chaff's tiers follow
    /// `kill_weight`.
    #[test]
    fn a_trail_brings_back_many_pieces_and_the_higher_tiers_stay_rare() {
        let data = Data::load();
        let tiers = &data.gear.rarities;
        let share = |weight: &dyn Fn(&crate::sim::data::RarityDef) -> u32, from: usize| {
            let total: u32 = tiers.iter().map(weight).sum();
            tiers[from..].iter().map(weight).sum::<u32>() as f64 / total as f64
        };
        // Rare and up.
        let higher = 2;
        const TRAILS: u64 = 2000;
        let (mut pieces, mut kills, mut found, mut before) = (0, 0, 0, 0.0);
        let mut chaff = vec![0u32; tiers.len()];
        for seed in 0..TRAILS {
            let run = run(&data, seed, 0);
            let outcome = run.outcome.as_ref().unwrap();
            let killed: u32 = outcome.deeds.iter().map(|deeds| deeds.kills).sum();
            assert!(outcome.drops.len() as u32 >= killed, "seed {seed}");
            pieces += outcome.drops.len();
            kills += killed as usize;
            found += outcome.drops.iter().filter(|gear| gear.rarity >= higher).count();
            if outcome.cleared > 0 {
                before += if outcome.won {
                    share(&|r| r.win_weight, higher)
                } else {
                    share(&|r| r.weight, higher)
                };
            }
            let trail_piece = outcome.trail_piece().map(|_| 1).unwrap_or(0);
            for gear in &outcome.drops[..outcome.drops.len() - trail_piece] {
                chaff[gear.rarity] += 1;
            }
        }
        let per_trail = pieces as f64 / TRAILS as f64;
        assert!((8.0..=25.0).contains(&per_trail), "{per_trail} pieces a trail");
        assert!(pieces >= kills);
        let (found, before) = (found as f64 / TRAILS as f64, before / TRAILS as f64);
        assert!(found <= before * 1.1, "{found} of the higher tiers a trail, {before} before");
        let total: u32 = chaff.iter().sum();
        for (tier, &count) in tiers.iter().zip(&chaff) {
            let expected =
                tier.kill_weight as f64 / tiers.iter().map(|r| r.kill_weight).sum::<u32>() as f64;
            let got = count as f64 / total as f64;
            assert!((got - expected).abs() < 0.03, "{}: {got} against {expected}", tier.name);
        }
    }
}
