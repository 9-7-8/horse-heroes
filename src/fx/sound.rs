//! Sound for the fountain: a launch sound as a burst leaves, a sound for
//! each piece of gear thrown that tells its rarity, landings, and a pickup
//! tick whose pitch climbs across a stream of pickups. And for the death the
//! fountain comes out of: a sound as the body breaks.
//!
//! There are no sound files. Every sound is a recipe in
//! `assets/data/sounds.ron`, made into samples when the game starts
//! (`render`). Which sound plays when is data too: a rarity names its own in
//! `gear.ron`, a fountain its own in `loot_shower.ron`, a death its own in
//! `deaths.ron`, and the rest are in `sound` in `loot_shower.ron`.
//!
//! Making the samples and deciding what is heard (`Run`, `Gate`) are plain
//! arithmetic, so they are unit-tested; the system below them only plays.

use std::collections::BTreeMap;
use std::f32::consts::TAU;
use std::num::NonZero;
use std::sync::Arc;
use std::time::Duration;

use bevy::audio::{AddAudioSource, ChannelCount, Decodable, Sample, SampleRate, Source, Volume};
use bevy::prelude::*;
use serde::Deserialize;

use super::death::Deaths;
use super::drops::{Kind, Shower};
use crate::game::Game;
use crate::sim::data::GearTable;
use crate::sim::rng::Rng;

/// Samples a second of every sound made here.
pub const RATE: u32 = 44_100;
/// Every voice is brought to nothing over its last few thousandths of a
/// second, so no sound ends on a click.
const TAIL_SECS: f32 = 0.005;
/// A voice has died away by this many nepers at its end: a thousandth of its
/// level, which is 60 decibels down.
const DECAY: f32 = 6.9;

#[derive(Clone, Copy, Debug, Default, PartialEq, Deserialize)]
pub enum Wave {
    #[default]
    Sine,
    Triangle,
    Square,
    /// A hiss, with nothing in it much above `freq`.
    Noise,
}

/// One tone of a sound. The fields are described in `sounds.ron`.
#[derive(Clone, Debug, Deserialize)]
pub struct Voice {
    #[serde(default)]
    pub wave: Wave,
    pub freq: f32,
    #[serde(default)]
    pub slide: f32,
    #[serde(default)]
    pub at: f32,
    pub secs: f32,
    #[serde(default = "attack")]
    pub attack: f32,
    pub level: f32,
}

fn attack() -> f32 {
    0.004
}

/// `assets/data/sounds.ron`, compiled into the binary: each sound by name.
#[derive(Clone, Debug, Deserialize)]
pub struct SoundBook {
    pub sounds: BTreeMap<String, Vec<Voice>>,
}

impl SoundBook {
    pub fn load() -> SoundBook {
        ron::from_str(include_str!("../../assets/data/sounds.ron"))
            .unwrap_or_else(|e| panic!("assets/data/sounds.ron: {e}"))
    }
}

/// The `sound` numbers in `loot_shower.ron`.
#[derive(Clone, Debug, Deserialize)]
pub struct SoundFeel {
    /// Multiplies every sound. 0 turns sound off.
    pub volume: f32,
    pub land_coin: String,
    pub land_gear: String,
    pub land_gap_secs: f32,
    pub land_fade: f32,
    pub land_vary: f32,
    pub pickup_coin: String,
    pub pickup_gear: String,
    pub pickup_scale: Vec<f32>,
    pub pickup_most_steps: usize,
    pub pickup_run_secs: f32,
}

/// How long a sound lasts: until its last voice ends.
pub fn length_secs(voices: &[Voice]) -> f32 {
    voices
        .iter()
        .map(|voice| voice.at + voice.secs)
        .fold(0.0, f32::max)
}

/// A sound's samples, one channel at `RATE`. Each voice comes up over its
/// attack and dies away over its length; the voices are added together.
pub fn render(voices: &[Voice]) -> Vec<f32> {
    let rate = RATE as f32;
    let mut samples = vec![0.0; (length_secs(voices) * rate).ceil() as usize];
    for (number, voice) in voices.iter().enumerate() {
        let start = (voice.at * rate) as usize;
        let count = (voice.secs * rate) as usize;
        // The hiss is the same every time: its seed is the voice's place.
        let mut hiss = Rng::new(number as u64 + 1);
        let (mut phase, mut low) = (0.0_f32, 0.0_f32);
        for i in 0..count {
            let Some(sample) = samples.get_mut(start + i) else {
                break;
            };
            let age = i as f32 / rate;
            let through = age / voice.secs;
            let freq = voice.freq * semitones(voice.slide * through);
            phase = (phase + freq / rate).fract();
            let wave = match voice.wave {
                Wave::Sine => (TAU * phase).sin(),
                Wave::Triangle => 4.0 * (phase - 0.5).abs() - 1.0,
                Wave::Square => 1.0_f32.copysign(0.5 - phase),
                Wave::Noise => {
                    low += (TAU * freq / rate).min(1.0) * (2.0 * hiss.unit() - 1.0 - low);
                    low
                }
            };
            let rise = (age / voice.attack.max(1.0 / rate)).min(1.0);
            let tail = ((voice.secs - age) / TAIL_SECS).min(1.0);
            *sample += voice.level * wave * rise * tail * (-DECAY * through).exp();
        }
    }
    samples
}

/// How much faster a sound is played to raise it by this many semitones.
pub fn semitones(up: f32) -> f32 {
    2.0_f32.powf(up / 12.0)
}

/// A stream of pickups: each one is a step up the scale from the last, so
/// many of them are heard as a rising run and not as one noise repeated.
#[derive(Default)]
pub struct Run {
    step: usize,
    /// Seconds since the last pickup.
    quiet: f32,
}

impl Run {
    pub fn wait(&mut self, dt: f32) {
        self.quiet += dt;
    }

    /// The next pickup's step. A pickup that comes after a silence starts a
    /// new run at the bottom, and a long run stops climbing at the top.
    pub fn next(&mut self, feel: &SoundFeel) -> usize {
        if self.quiet > feel.pickup_run_secs {
            self.step = 0;
        }
        self.quiet = 0.0;
        let step = self.step.min(feel.pickup_most_steps);
        self.step += 1;
        step
    }
}

/// How far above the first a run's `step` is, in semitones: up
/// `pickup_scale`, and past its end again an octave higher.
pub fn run_semitones(feel: &SoundFeel, step: usize) -> f32 {
    let notes = feel.pickup_scale.len().max(1);
    let note = feel.pickup_scale.get(step % notes).copied().unwrap_or(0.0);
    12.0 * (step / notes) as f32 + note
}

/// Landings: a burst's pieces touch the ground dozens at a time, and the
/// ones that come within `land_gap_secs` of another are heard as one.
#[derive(Default)]
pub struct Gate {
    /// Seconds until the next landing is heard.
    shut: f32,
}

impl Gate {
    pub fn wait(&mut self, dt: f32) {
        self.shut = (self.shut - dt).max(0.0);
    }

    /// True if this landing is heard.
    pub fn pass(&mut self, feel: &SoundFeel) -> bool {
        if self.shut > 0.0 {
            return false;
        }
        self.shut = feel.land_gap_secs;
        true
    }
}

/// How loud a piece's landing is on its `touch`: the first is 0, and each
/// bounce after it is quieter.
pub fn landing_level(feel: &SoundFeel, touch: u32) -> f32 {
    feel.land_fade.powi(touch as i32)
}

/// Every sound the data names, with where it is named: each must be in
/// `sounds.ron`. A rarity may name none.
pub fn named<'a>(
    shower: &'a Shower,
    gear: &'a GearTable,
    deaths: &'a Deaths,
) -> Vec<(String, &'a str)> {
    let feel = &shower.sound;
    let mut names = vec![
        ("loot_shower.ron: land_coin".to_string(), feel.land_coin.as_str()),
        ("loot_shower.ron: land_gear".to_string(), feel.land_gear.as_str()),
        ("loot_shower.ron: pickup_coin".to_string(), feel.pickup_coin.as_str()),
        ("loot_shower.ron: pickup_gear".to_string(), feel.pickup_gear.as_str()),
    ];
    for (name, fountain) in &shower.fountains {
        names.push((format!("loot_shower.ron: fountain `{name}`"), &fountain.sound));
    }
    for rarity in gear.rarities.iter().filter(|r| !r.look.sound.is_empty()) {
        names.push((format!("gear.ron: {}", rarity.name), &rarity.look.sound));
    }
    for (name, death) in &deaths.deaths {
        names.push((format!("deaths.ron: death `{name}`"), &death.sound));
    }
    names
}

/// Something the fountain did that can be heard.
#[derive(Message, Clone, Debug)]
pub enum Heard {
    /// A burst has been thrown: its first piece leaves `in_secs` from now,
    /// with this sound.
    Burst { in_secs: f32, sound: String },
    /// A piece of gear of this rarity tier has left the body.
    Thrown(usize),
    /// A piece has touched the ground, for the `touch`th time, from 0.
    Landed { kind: Kind, touch: u32 },
    /// A piece has reached the party.
    Collected(Kind),
    /// An enemy's body has broken, with this sound.
    Died(String),
}

/// A sound made from its recipe: one channel of samples at `RATE`.
#[derive(Asset, TypePath, Clone, Debug)]
pub struct Clip {
    samples: Arc<[f32]>,
}

/// A `Clip` being played.
pub struct ClipPlaying {
    samples: Arc<[f32]>,
    at: usize,
}

impl Iterator for ClipPlaying {
    type Item = Sample;

    fn next(&mut self) -> Option<Sample> {
        let sample = self.samples.get(self.at).copied();
        self.at += 1;
        sample
    }
}

impl Source for ClipPlaying {
    fn current_span_len(&self) -> Option<usize> {
        // None is "until the sound ends"; nothing changes on the way.
        None
    }

    fn channels(&self) -> ChannelCount {
        NonZero::<u16>::MIN
    }

    fn sample_rate(&self) -> SampleRate {
        NonZero::new(RATE).unwrap()
    }

    fn total_duration(&self) -> Option<Duration> {
        Some(Duration::from_secs_f32(self.samples.len() as f32 / RATE as f32))
    }
}

impl Decodable for Clip {
    type Decoder = ClipPlaying;

    fn decoder(&self) -> ClipPlaying {
        ClipPlaying {
            samples: self.samples.clone(),
            at: 0,
        }
    }
}

pub struct SoundPlugin {
    /// Play nothing (`--mute`).
    pub mute: bool,
}

impl Plugin for SoundPlugin {
    fn build(&self, app: &mut App) {
        app.add_audio_source::<Clip>()
            .add_message::<Heard>()
            .insert_resource(Hearing {
                mute: self.mute,
                ..default()
            })
            .add_systems(Startup, setup)
            // After Update, so what the drops did this frame is heard in it.
            .add_systems(PostUpdate, play);
    }
}

/// The sounds, made, by their names in `sounds.ron`.
#[derive(Resource)]
struct Clips(BTreeMap<String, Handle<Clip>>);

#[derive(Resource, Default)]
struct Hearing {
    mute: bool,
    /// Bursts thrown whose first piece has not left yet.
    waiting: Vec<(f32, String)>,
    run: Run,
    gate: Gate,
    /// Landings that were let through, which is what their pitch varies by.
    landings: u64,
}

fn setup(
    mut commands: Commands,
    game: Res<Game>,
    shower: Res<Shower>,
    deaths: Res<Deaths>,
    mut clips: ResMut<Assets<Clip>>,
) {
    let book = SoundBook::load();
    for (place, name) in named(&shower, &game.data.gear, &deaths) {
        assert!(book.sounds.contains_key(name), "{place}: no sound named `{name}` in sounds.ron");
    }
    let made = book
        .sounds
        .iter()
        .map(|(name, voices)| {
            let samples = render(voices).into();
            (name.clone(), clips.add(Clip { samples }))
        })
        .collect();
    commands.insert_resource(Clips(made));
}

fn play(
    mut commands: Commands,
    time: Res<Time<Real>>,
    game: Res<Game>,
    shower: Res<Shower>,
    clips: Res<Clips>,
    mut heard: MessageReader<Heard>,
    mut hearing: ResMut<Hearing>,
) {
    let dt = time.delta_secs();
    let feel = &shower.sound;
    let hearing = &mut *hearing;
    hearing.run.wait(dt);
    hearing.gate.wait(dt);

    // What to play this frame: a sound, how loud, and how many semitones up.
    let mut due: Vec<(&str, f32, f32)> = Vec::new();
    // Launches and breaks, which bring their sound's name with them.
    let mut launches = Vec::new();
    hearing.waiting.retain_mut(|(in_secs, sound)| {
        *in_secs -= dt;
        if *in_secs <= 0.0 {
            launches.push(std::mem::take(sound));
        }
        *in_secs > 0.0
    });
    for event in heard.read() {
        match event {
            Heard::Burst { in_secs, sound } if *in_secs > 0.0 => {
                hearing.waiting.push((*in_secs, sound.clone()));
            }
            Heard::Burst { sound, .. } => launches.push(sound.clone()),
            Heard::Thrown(tier) => {
                let sound = &game.data.gear.rarities[*tier].look.sound;
                if !sound.is_empty() {
                    due.push((sound, 1.0, 0.0));
                }
            }
            Heard::Landed { kind, touch } => {
                if hearing.gate.pass(feel) {
                    let sound = match kind {
                        Kind::Coin => &feel.land_coin,
                        Kind::Gear(_) => &feel.land_gear,
                    };
                    let vary = 2.0 * Rng::new(hearing.landings).unit() - 1.0;
                    hearing.landings += 1;
                    due.push((sound, landing_level(feel, *touch), feel.land_vary * vary));
                }
            }
            Heard::Collected(Kind::Coin) => {
                let step = hearing.run.next(feel);
                due.push((&feel.pickup_coin, 1.0, run_semitones(feel, step)));
            }
            Heard::Collected(Kind::Gear(_)) => due.push((&feel.pickup_gear, 1.0, 0.0)),
            Heard::Died(sound) => launches.push(sound.clone()),
        }
    }
    due.extend(launches.iter().map(|sound| (sound.as_str(), 1.0, 0.0)));

    if hearing.mute || feel.volume <= 0.0 {
        return;
    }
    for (sound, level, up) in due {
        let Some(clip) = clips.0.get(sound) else {
            continue;
        };
        debug!("sound: {sound} at {level:.2}, {up:+.1} semitones");
        commands.spawn((
            AudioPlayer::<Clip>(clip.clone()),
            PlaybackSettings::DESPAWN
                .with_volume(Volume::Linear(feel.volume * level))
                .with_speed(semitones(up)),
        ));
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::sim::data::Data;

    fn feel() -> SoundFeel {
        Shower::load().sound
    }

    /// The loudest a sound gets.
    fn peak(samples: &[f32]) -> f32 {
        samples.iter().fold(0.0, |peak, sample| peak.max(sample.abs()))
    }

    /// The times a sound's voices start at, in order: its notes.
    fn onsets(voices: &[Voice]) -> Vec<u32> {
        let mut onsets: Vec<u32> = voices.iter().map(|v| (v.at * 1000.0) as u32).collect();
        onsets.sort();
        onsets.dedup();
        onsets
    }

    /// Every recipe makes a sound that can be played as it is: something to
    /// hear, never past full scale, short, and silent at both ends so it
    /// neither starts nor stops on a click.
    #[test]
    fn every_sound_can_be_played() {
        let book = SoundBook::load();
        assert!(!book.sounds.is_empty());
        for (name, voices) in &book.sounds {
            assert!(!voices.is_empty(), "{name}");
            for voice in voices {
                assert!(voice.freq > 0.0 && voice.freq < RATE as f32 / 2.0, "{name}");
                assert!(voice.secs > TAIL_SECS && voice.at >= 0.0, "{name}");
                assert!(voice.attack >= 0.0 && voice.attack < voice.secs, "{name}");
                assert!(voice.level > 0.0 && voice.level <= 1.0, "{name}");
            }
            let samples = render(voices);
            let secs = samples.len() as f32 / RATE as f32;
            assert!((secs - length_secs(voices)).abs() < 0.001, "{name}: {secs} s");
            assert!(secs <= 1.5, "{name} lasts {secs} s");
            assert!(samples.iter().all(|sample| sample.is_finite()), "{name}");
            let peak = peak(&samples);
            assert!(peak > 0.05, "{name} can hardly be heard: {peak}");
            assert!(peak <= 1.0, "{name} is past full scale: {peak}");
            assert!(samples[0].abs() < 0.01, "{name} starts on a click");
            assert!(samples[samples.len() - 1].abs() < 0.01, "{name} ends on a click");
        }
    }

    /// A voice is the pitch its recipe says, and a slide moves it: counted
    /// by how often the samples cross zero going up.
    #[test]
    fn a_voice_is_the_pitch_it_says() {
        let crossings = |samples: &[f32]| {
            samples.windows(2).filter(|pair| pair[0] < 0.0 && pair[1] >= 0.0).count() as f32
        };
        let mut voice = Voice {
            wave: Wave::Sine,
            freq: 440.0,
            slide: 0.0,
            at: 0.0,
            secs: 0.5,
            attack: 0.004,
            level: 0.5,
        };
        for wave in [Wave::Sine, Wave::Triangle, Wave::Square] {
            voice.wave = wave;
            let heard = crossings(&render(std::slice::from_ref(&voice))) / voice.secs;
            assert!((heard - 440.0).abs() < 6.0, "{wave:?}: {heard} Hz");
        }
        // An octave up by the end: more cycles than staying put, fewer than
        // starting there.
        voice.wave = Wave::Sine;
        voice.slide = 12.0;
        let heard = crossings(&render(std::slice::from_ref(&voice))) / voice.secs;
        assert!(heard > 500.0 && heard < 820.0, "{heard} Hz");
        assert_eq!(semitones(12.0), 2.0);
        assert_eq!(semitones(0.0), 1.0);
    }

    /// The same recipe makes the same samples, hiss and all.
    #[test]
    fn a_sound_is_the_same_every_time() {
        for (name, voices) in &SoundBook::load().sounds {
            assert_eq!(render(voices), render(voices), "{name}");
        }
    }

    /// Every sound the data names is in `sounds.ron`, and every fountain and
    /// every rarity names one.
    #[test]
    fn every_sound_named_is_there() {
        let (shower, data, book) = (Shower::load(), Data::load(), SoundBook::load());
        for (place, name) in named(&shower, &data.gear, &Deaths::load()) {
            assert!(book.sounds.contains_key(name), "{place}: no sound `{name}`");
        }
        for rarity in &data.gear.rarities {
            assert!(!rarity.look.sound.is_empty(), "{} has no sound", rarity.name);
        }
    }

    /// A rarity can be told by ear: no two tiers share a sound or its notes,
    /// no tier's sound is used for anything else, and a better tier's is
    /// never the shorter. A Rare has more notes than any tier below it.
    #[test]
    fn a_rarity_is_told_by_its_sound() {
        let (shower, data, book) = (Shower::load(), Data::load(), SoundBook::load());
        let rarities = &data.gear.rarities;
        let of = |tier: usize| &book.sounds[&rarities[tier].look.sound];
        let notes = |voices: &[Voice]| -> Vec<(u32, u32)> {
            let mut notes: Vec<(u32, u32)> = voices
                .iter()
                .map(|v| ((v.at * 1000.0) as u32, v.freq as u32))
                .collect();
            notes.sort();
            notes
        };
        for tier in 0..rarities.len() {
            for other in 0..tier {
                assert_ne!(rarities[tier].look.sound, rarities[other].look.sound);
                assert_ne!(notes(of(tier)), notes(of(other)), "{}", rarities[tier].name);
            }
            if tier > 0 {
                assert!(
                    length_secs(of(tier)) >= length_secs(of(tier - 1)),
                    "{} is shorter than the tier below it",
                    rarities[tier].name
                );
            }
        }
        let drop_sounds: Vec<&str> = rarities.iter().map(|r| r.look.sound.as_str()).collect();
        for (place, name) in named(&shower, &data.gear, &Deaths::load()) {
            let uses = drop_sounds.iter().filter(|&&sound| sound == name).count();
            assert!(uses == 0 || place.starts_with("gear.ron"), "{place} uses {name}");
        }
        let rare = rarities.iter().position(|r| r.name == "Rare").unwrap();
        for lesser in 0..rare {
            assert!(
                onsets(of(rare)).len() > onsets(of(lesser)).len(),
                "a Rare has no more notes than a {}",
                rarities[lesser].name
            );
        }
    }

    /// A death can be told by ear: an ordinary enemy's and a final one's do
    /// not share a sound, the final's is at least twice as long and reaches
    /// an octave or more lower, and no death's sound is used for anything
    /// else.
    #[test]
    fn a_final_death_is_told_from_an_ordinary_one_by_its_sound() {
        let (shower, data, book) = (Shower::load(), Data::load(), SoundBook::load());
        let deaths = Deaths::load();
        let of = |ids: &Vec<Vec<String>>| -> Vec<&str> {
            ids.iter()
                .flatten()
                .map(|id| {
                    let enemy = &data.enemies[data.enemy_index(id)];
                    deaths.death(&enemy.death).sound.as_str()
                })
                .collect()
        };
        let lowest = |voices: &[Voice]| voices.iter().map(|v| v.freq).fold(f32::MAX, f32::min);
        for small in of(&data.encounters.ordinary) {
            for big in of(&data.encounters.finals) {
                assert_ne!(small, big);
                let (small, big) = (&book.sounds[small], &book.sounds[big]);
                assert!(length_secs(big) >= 2.0 * length_secs(small));
                assert!(lowest(big) <= lowest(small) / 2.0);
            }
        }
        let death_sounds: Vec<&str> = deaths.deaths.values().map(|d| d.sound.as_str()).collect();
        for (place, name) in named(&shower, &data.gear, &deaths) {
            assert!(
                !death_sounds.contains(&name) || place.starts_with("deaths.ron"),
                "{place} uses {name}"
            );
        }
    }

    /// A death lands with its fountain's launch: the two played together,
    /// when each is due, stay inside full scale. Whether one covers the
    /// other is for an ear to say.
    #[test]
    fn a_death_and_its_launch_fit_together() {
        let (shower, data, book) = (Shower::load(), Data::load(), SoundBook::load());
        let deaths = Deaths::load();
        for enemy in &data.enemies {
            let death = deaths.death(&enemy.death);
            let fountain = shower.fountain(&enemy.fountain);
            let broke = render(&book.sounds[&death.sound]);
            let launch = render(&book.sounds[&fountain.sound]);
            let at = |secs: f32| (secs * RATE as f32) as usize;
            let (broke_at, launch_at) = (at(death.fall_secs), at(fountain.delay_secs));
            let mut both = vec![0.0; (broke_at + broke.len()).max(launch_at + launch.len())];
            for (i, sample) in broke.iter().enumerate() {
                both[broke_at + i] += sample;
            }
            for (i, sample) in launch.iter().enumerate() {
                both[launch_at + i] += sample;
            }
            let loudest = shower.sound.volume * peak(&both);
            assert!(loudest <= 1.0, "{}: {loudest}", enemy.id);
        }
    }

    /// A stream of pickups is a rising run: every pickup is higher than the
    /// last until the run tops out, where it stays, and never so high that
    /// the tick is played more than eight times too fast.
    #[test]
    fn a_stream_of_pickups_rises() {
        let feel = feel();
        assert!(feel.pickup_most_steps >= 4, "too short a run to hear as one");
        assert!(feel.pickup_scale.first() == Some(&0.0));
        assert!(feel.pickup_scale.iter().all(|&note| (0.0..12.0).contains(&note)));
        let mut run = Run::default();
        let mut last = f32::MIN;
        for pickup in 0..feel.pickup_most_steps + 20 {
            run.wait(0.06);
            let up = run_semitones(&feel, run.next(&feel));
            if pickup == 0 {
                assert_eq!(up, 0.0);
            } else if pickup <= feel.pickup_most_steps {
                assert!(up > last, "pickup {pickup} is not higher: {up} after {last}");
            } else {
                assert_eq!(up, last, "the run went on climbing at pickup {pickup}");
            }
            assert!(semitones(up) <= 8.0, "pickup {pickup} is {up} semitones up");
            last = up;
        }
    }

    /// A pickup after a silence starts a new run at the bottom; one that
    /// comes sooner carries on.
    #[test]
    fn a_run_starts_again_after_a_silence() {
        let feel = feel();
        let mut run = Run::default();
        // The first of all is the bottom, however long the game has run.
        run.wait(100.0);
        assert_eq!(run.next(&feel), 0);
        run.wait(feel.pickup_run_secs * 0.9);
        assert_eq!(run.next(&feel), 1);
        run.wait(feel.pickup_run_secs * 0.9);
        assert_eq!(run.next(&feel), 2);
        run.wait(feel.pickup_run_secs * 1.1);
        assert_eq!(run.next(&feel), 0);
        // The gap between two pieces of one burst never ends a run.
        assert!(Shower::load().pickup_stagger_secs < feel.pickup_run_secs);
    }

    /// Dozens of pieces landing in one frame are one sound, the next is
    /// heard once the gap has passed, and a later bounce is quieter.
    #[test]
    fn landings_together_are_heard_as_one() {
        let feel = feel();
        assert!(feel.land_gap_secs > 0.0 && feel.land_vary >= 0.0);
        let mut gate = Gate::default();
        let heard = (0..40).filter(|_| gate.pass(&feel)).count();
        assert_eq!(heard, 1);
        gate.wait(feel.land_gap_secs / 2.0);
        assert!(!gate.pass(&feel));
        gate.wait(feel.land_gap_secs);
        assert!(gate.pass(&feel));

        assert_eq!(landing_level(&feel, 0), 1.0);
        assert!(feel.land_fade > 0.0 && feel.land_fade < 1.0);
        for touch in 0..Shower::load().bounces {
            assert!(landing_level(&feel, touch + 1) < landing_level(&feel, touch));
        }
    }

    /// Nothing is played louder than a recipe was written: the volume turns
    /// sound down, to nothing at 0, and never up.
    #[test]
    fn the_volume_only_turns_sound_down() {
        let feel = feel();
        assert!((0.0..=1.0).contains(&feel.volume), "{}", feel.volume);
    }
}
