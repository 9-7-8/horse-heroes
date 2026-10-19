//! The menu shell: the bar of places, the status strip and the footer, which
//! never move, and between them the place that is open. Every later screen
//! hangs on this. How it works: docs/menu-shell.md.
//!
//! All the places are spawned once and only shown or hidden, so a place is
//! found exactly as it was left.

use bevy::{
    input_focus::{
        FocusCause, InputFocus, InputFocusVisible,
        directional_navigation::{AutoNavigationConfig, FocusableArea},
        navigator::find_best_candidate,
        tab_navigation::TabGroup,
    },
    math::CompassOctant,
    picking::{
        events::{PointerDragEnd, PointerDragStart, PointerPress},
        hover::Hovered,
        pointer::{PointerButton, PointerId, PointerLocation},
    },
    prelude::*,
    ui::{InteractionDisabled, ScrollPosition, Selected, UiGlobalTransform},
    ui_widgets::{
        Activate, ControlOrientation, ScrollArea, ScrollIntoView, Scrollbar, ScrollbarThumb,
    },
    window::PrimaryWindow,
};
use serde::Deserialize;

use super::card;
use super::held::{self, Held};
use super::teach;
use super::ink;
use super::picture::{self, picture, picture_of, reported_picture};
use super::parts::{
    ClosePanel, CounterValue, DismissHint, HintText, NotBuilt, Part, PartText, Pointed, PopupArrow, RowValue,
    SkipHints, Stop, Target, Unseen, button, cell, counter, dim, panel, popup, reason, side_panel, tab, tag,
    text, title, trail_mark,
};
use super::tokens::Tokens;
use crate::game::{
    Action, Game, Hand, Piece, Raise, SceneView, Screen, Uncollected, fight_log, report_side,
    report_text, stats_line,
};
use crate::game::scene::{HorseBox, enemy_centre, enemy_size, horse_feet, line_order};
use crate::sim::data::Slot;
use crate::sim::expedition::Phase;
use crate::sim::lessons::{EntryDef, Tick};

/// The map of the menus, from `assets/data/places.ron`.
#[derive(Resource, Clone, Debug, Deserialize)]
pub struct Places {
    pub places: Vec<Place>,
    /// At the far end of the bar: a page like a place, and not one of them.
    pub settings: Place,
    /// The keys that select the horses of the party, in order.
    #[serde(default)]
    pub horse_keys: Vec<String>,
    /// The keys that scroll what does not fit, up and then down (#304).
    pub scroll_keys: (String, String),
    /// The key that hides and shows the fight log over the road (#306).
    pub log_key: String,
    #[serde(default)]
    pub footer: Vec<Control>,
    /// What can be done with a piece of gear, on its card (#228): the same
    /// controls wherever the piece is.
    #[serde(default)]
    pub card: Vec<Control>,
    /// What puts a mark on a place (#232).
    #[serde(default)]
    pub marks: Vec<MarkRule>,
    /// Every control of the map, by its `id`.
    #[serde(skip)]
    list: Vec<Control>,
}

#[derive(Clone, Debug, Deserialize)]
pub struct Place {
    pub name: String,
    /// The key that opens it, printed on it.
    #[serde(default)]
    pub key: Option<String>,
    /// What is decided here, for a place with no tabs.
    #[serde(default)]
    pub job: String,
    /// What a place with no tabs shows.
    #[serde(default)]
    pub content: Content,
    /// Show the horses beside the tabs on one side and the cargo on the other.
    #[serde(default)]
    pub horse_columns: bool,
    #[serde(default)]
    pub tabs: Vec<Tab>,
    /// The controls of a place with no tabs.
    #[serde(default)]
    pub controls: Vec<Control>,
    /// The controls of the cargo column, in a place with `horse_columns`.
    #[serde(default)]
    pub cargo: Vec<Control>,
}

#[derive(Clone, Debug, Deserialize)]
pub struct Tab {
    pub name: String,
    /// The key that chooses it while its place is showing, printed on it.
    #[serde(default)]
    pub key: Option<String>,
    pub job: String,
    #[serde(default)]
    pub content: Content,
    #[serde(default)]
    pub controls: Vec<Control>,
}

/// A control planned for the menus. One with an `action`, a `shows` or a
/// `switch` works; one with none is not built yet, and a press on it says so.
#[derive(Clone, Debug, Deserialize)]
pub struct Control {
    pub name: String,
    /// What it does, or will do.
    pub does: String,
    /// The key that works it, printed on it.
    #[serde(default)]
    pub key: Option<String>,
    /// The issue that will make it work.
    #[serde(default)]
    pub issue: Option<u32>,
    /// The issue is a proposal the owner has not decided.
    #[serde(default)]
    pub proposed: bool,
    #[serde(default)]
    pub action: Option<Action>,
    /// What a press opens in the side panel of its page.
    #[serde(default)]
    pub shows: Option<Shows>,
    /// What a press turns on and off, of what is shown.
    #[serde(default)]
    pub switch: Option<Switch>,
    /// A line that stays under its group of controls, whether or not it
    /// can be used: what the player should know of it.
    #[serde(default)]
    pub note: Option<String>,
    /// Its number among all the controls of the map, set on loading.
    #[serde(skip)]
    pub id: usize,
}

impl Control {
    /// Whether a press does what the control says: it acts, it shows, or
    /// it switches.
    pub fn built(&self) -> bool {
        self.action.is_some() || self.shows.is_some() || self.switch.is_some()
    }
}

/// What raises a mark, and where: on a place, and on the tab of it the
/// thing is on when it is on one. Both are named as the map names them.
#[derive(Clone, Debug, Deserialize)]
pub struct MarkRule {
    pub raise: Raise,
    pub place: String,
    #[serde(default)]
    pub tab: Option<String>,
}

/// What a control shows in the side panel when it is pressed.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Deserialize)]
pub enum Shows {
    /// Every key, and what it works.
    Keys,
}

/// What a control turns on and off: a way of showing things, which changes
/// nothing in the game.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Deserialize)]
pub enum Switch {
    /// The comparison's table has only the rows that differ (#349).
    ChangedOnly,
    /// The hints are shown (#300). Off, none is shown by itself.
    Hints,
    /// Each hit's damage rises from what it struck (#249).
    DamageNumbers,
    /// The card's third layer, its formulas and assumptions (#228).
    Details,
    /// The list beside Equip of the other horses, to equip on (#356).
    OtherHorses,
}

/// What a tab shows, or a place without tabs. `Planned` is one whose system
/// is not built.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, Deserialize)]
pub enum Content {
    #[default]
    Planned,
    Gear,
    /// The selected horse's ability and what it does (#357).
    Abilities,
    Report,
    /// Every piece the party has, worn or carried, in a grid by type (#308).
    Inventory,
    /// Every hint that has been given, to show again (#300).
    Hints,
}

impl Places {
    pub fn load() -> Places {
        let mut places: Places = ron::from_str(include_str!("../../../assets/data/places.ron"))
            .unwrap_or_else(|e| panic!("assets/data/places.ron: {e}"));
        let mut list = Vec::new();
        for (id, control) in places.controls_mut().into_iter().enumerate() {
            control.id = id;
            list.push(control.clone());
        }
        places.list = list;
        places
    }

    /// Every page of the menus: the places, then settings.
    pub fn pages(&self) -> impl Iterator<Item = &Place> {
        self.places.iter().chain(std::iter::once(&self.settings))
    }

    /// Which page settings is: the one after the places.
    pub fn settings_page(&self) -> usize {
        self.places.len()
    }

    /// Every control of the map, in the order of their `id`.
    #[cfg(test)]
    pub fn controls(&self) -> &[Control] {
        &self.list
    }

    pub fn control(&self, id: usize) -> &Control {
        &self.list[id]
    }

    /// The page of this name: a place, or settings.
    pub fn page_named(&self, name: &str) -> Option<usize> {
        self.pages().position(|page| page.name == name)
    }

    /// The tab of this name on the page of that name.
    pub fn tab_named(&self, page: &str, tab: &str) -> Option<(usize, usize)> {
        let page = self.page_named(page)?;
        let tabs = &self.pages().nth(page)?.tabs;
        Some((page, tabs.iter().position(|other| other.name == tab)?))
    }

    /// The `id` of the control of this name: the first, where two places
    /// have one of the same name.
    pub fn control_named(&self, name: &str) -> Option<usize> {
        self.list.iter().position(|control| control.name == name)
    }

    /// The controls of the map in the groups they are drawn in: those of a
    /// tab, of a place without tabs, of a cargo column, of the footer, and
    /// of the item card.
    /// A group says under itself why any of its controls cannot be used.
    #[cfg(test)]
    pub fn groups(&self) -> Vec<&[Control]> {
        let mut groups = Vec::new();
        for place in self.pages() {
            groups.extend(place.tabs.iter().map(|tab| tab.controls.as_slice()));
            groups.push(&place.controls);
            groups.push(&place.cargo);
        }
        groups.push(&self.footer);
        groups.push(&self.card);
        groups.retain(|group| !group.is_empty());
        groups
    }

    /// The key that opens a page: a place, or settings.
    pub fn page_key(&self, page: usize) -> Option<&str> {
        self.pages().nth(page)?.key.as_deref()
    }

    pub fn tab_key(&self, page: usize, tab: usize) -> Option<&str> {
        self.pages().nth(page)?.tabs.get(tab)?.key.as_deref()
    }

    /// What a tab of a page shows, or the page itself when it has no tabs.
    pub fn content(&self, page: usize, tab: usize) -> Content {
        self.pages().nth(page).map_or(Content::Planned, |page| {
            page.tabs.get(tab).map_or(page.content, |tab| tab.content)
        })
    }

    pub fn horse_key(&self, horse: usize) -> Option<&str> {
        self.horse_keys.get(horse).map(String::as_str)
    }

    /// Where a mark's rule puts it: the place, and the tab if it names one.
    fn mark_at(&self, rule: &MarkRule) -> Option<(usize, Option<usize>)> {
        let place = self.places.iter().position(|place| place.name == rule.place)?;
        let tab = match &rule.tab {
            Some(name) => Some(self.places[place].tabs.iter().position(|tab| tab.name == *name)?),
            None => None,
        };
        Some((place, tab))
    }

    /// Whether a place has something the player has not seen (#232), or,
    /// with a tab, whether that tab of it has.
    pub fn wants(&self, game: &Game, place: usize, tab: Option<usize>) -> bool {
        self.marks.iter().any(|rule| {
            self.mark_at(rule)
                .is_some_and(|(p, t)| p == place && (tab.is_none() || t == tab))
                && game.wants(&rule.raise)
        })
    }

    fn controls_mut(&mut self) -> Vec<&mut Control> {
        let mut all = Vec::new();
        for place in self
            .places
            .iter_mut()
            .chain(std::iter::once(&mut self.settings))
        {
            for tab in &mut place.tabs {
                all.extend(tab.controls.iter_mut());
            }
            all.extend(place.controls.iter_mut());
            all.extend(place.cargo.iter_mut());
        }
        all.extend(self.footer.iter_mut());
        all.extend(self.card.iter_mut());
        all
    }
}

/// The key a control's `key` names: a letter, a digit, a square bracket,
/// Space, PageUp or PageDown.
pub fn key_code(name: &str) -> Option<KeyCode> {
    Some(match name {
        "PageUp" => KeyCode::PageUp,
        "PageDown" => KeyCode::PageDown,
        "Space" => KeyCode::Space,
        "[" => KeyCode::BracketLeft,
        "]" => KeyCode::BracketRight,
        "0" => KeyCode::Digit0,
        "1" => KeyCode::Digit1,
        "2" => KeyCode::Digit2,
        "3" => KeyCode::Digit3,
        "4" => KeyCode::Digit4,
        "5" => KeyCode::Digit5,
        "6" => KeyCode::Digit6,
        "7" => KeyCode::Digit7,
        "8" => KeyCode::Digit8,
        "9" => KeyCode::Digit9,
        "A" => KeyCode::KeyA,
        "B" => KeyCode::KeyB,
        "C" => KeyCode::KeyC,
        "D" => KeyCode::KeyD,
        "E" => KeyCode::KeyE,
        "F" => KeyCode::KeyF,
        "G" => KeyCode::KeyG,
        "H" => KeyCode::KeyH,
        "I" => KeyCode::KeyI,
        "J" => KeyCode::KeyJ,
        "K" => KeyCode::KeyK,
        "L" => KeyCode::KeyL,
        "M" => KeyCode::KeyM,
        "N" => KeyCode::KeyN,
        "O" => KeyCode::KeyO,
        "P" => KeyCode::KeyP,
        "Q" => KeyCode::KeyQ,
        "R" => KeyCode::KeyR,
        "S" => KeyCode::KeyS,
        "T" => KeyCode::KeyT,
        "U" => KeyCode::KeyU,
        "V" => KeyCode::KeyV,
        "W" => KeyCode::KeyW,
        "X" => KeyCode::KeyX,
        "Y" => KeyCode::KeyY,
        "Z" => KeyCode::KeyZ,
        _ => return None,
    })
}

impl Place {
    /// The pages of a place: one per tab, or the place itself when it has none.
    fn pages(&self) -> Vec<(String, &str, Content, &[Control])> {
        if self.tabs.is_empty() {
            return vec![(
                self.name.clone(),
                &self.job,
                self.content,
                &self.controls,
            )];
        }
        self.tabs
            .iter()
            .map(|tab| {
                (
                    format!("{}, {}", self.name, tab.name),
                    tab.job.as_str(),
                    tab.content,
                    tab.controls.as_slice(),
                )
            })
            .collect()
    }
}

/// Every piece the party has of one type, in the order the Inventory shows
/// them (#308): what each horse wears in that slot, in the party's order,
/// and then every piece in the cargo of that type, newest first (#352).
pub fn held(game: &Game, slot: usize) -> Vec<Piece> {
    let worn = (0..game.party.len()).map(|horse| Piece::Worn { horse, slot });
    worn.chain((0..game.cargo().len()).map(Piece::Carried))
        .filter(|piece| game.piece(*piece).is_some_and(|gear| gear.slot.index() == slot))
        .collect()
}

/// What a side panel is about.
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum Item {
    /// A gear slot of a horse, and the piece worn in it.
    Worn { horse: usize, slot: usize },
    /// A piece carried in the cargo, counted from the newest.
    Carried(usize),
    /// A control, by its `id`: what it shows, or, for one that is not
    /// built, what it will do.
    Control(usize),
    /// A horse of the party: everything about it (#311).
    Horse(usize),
}

impl Item {
    /// The piece of gear this is, when it is one: its card is what shows.
    pub fn piece(self) -> Option<Piece> {
        match self {
            Item::Worn { horse, slot } => Some(Piece::Worn { horse, slot }),
            Item::Carried(at) => Some(Piece::Carried(at)),
            Item::Control(_) | Item::Horse(_) => None,
        }
    }
}

impl From<Piece> for Item {
    fn from(piece: Piece) -> Item {
        match piece {
            Piece::Worn { horse, slot } => Item::Worn { horse, slot },
            Piece::Carried(at) => Item::Carried(at),
        }
    }
}

/// Detail about one thing, open in a place's side panel.
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Detail {
    /// The tab it was opened from; it shows on that tab only.
    pub tab: usize,
    pub item: Item,
    /// The control that opened it, which gets the focus back.
    pub opener: Option<Entity>,
}

/// The control a hint is about (#291): a place in the bar, a tab of a
/// place, a horse in the strip, a control of the map by its `id`, or a
/// piece of gear wherever it is listed (#290).
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum About {
    Place(usize),
    Tab(usize, usize),
    Horse(usize),
    Control(usize),
    Piece(Piece),
    /// The whole party, on no control: no mark and no name, only the words
    /// (#292).
    Party,
    /// The boss in the scene, by the place it stands in (#360). Not a
    /// control: the pop-up points at it and nothing is pressed.
    Boss(usize),
}

/// A lesson's one line (#291), on the hint's line under the status strip,
/// with a mark on the control it is about. It stays until its lesson says
/// the thing was done, or the player puts it away; never for a time.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Hint {
    /// The lesson it belongs to, by which a dismissed one is known again.
    pub lesson: String,
    pub about: About,
    /// What is said beside the control's name and key: one short line.
    pub words: String,
}

/// Where the player is in the menus. One selected horse for every place;
/// each place keeps its own tab and its own open detail.
#[derive(Resource, Clone, Debug, PartialEq)]
pub struct Shell {
    place: usize,
    /// Whether the place is showing. Always, in town. On the road it can be
    /// closed, which gives its room to the scene.
    showing: bool,
    horse: usize,
    /// Whether the selected horse is marked in the scene and the tags. A
    /// press on the background takes the mark away (#366); the menus still
    /// show the last horse chosen.
    marked: bool,
    tabs: Vec<usize>,
    details: Vec<Option<Detail>>,
    /// The piece the pointer rests on and the cell it is in, while its card
    /// shows beside it (#228).
    pointed: Option<(Entity, Piece)>,
    /// The fight log over the road scene has been put away by its key (#306).
    log_hidden: bool,
    /// Which rows the comparison's table has (#349): one setting for every
    /// piece and every place.
    rows: card::Rows,
    /// The hint that is showing (#291): one at a time.
    hint: Option<Hint>,
    /// The lessons whose hint the player put away. None of them is shown
    /// again unless the player asks.
    dismissed: Vec<String>,
    /// The player skipped every hint: none is shown.
    hints_off: bool,
    /// Skip all hints was pressed and has not been answered yet (#350).
    owed: bool,
    /// Where the hints went has been said: once, and never again.
    told: bool,
    /// The damage numbers are turned off (#249).
    numbers_off: bool,
    /// A pinned card shows its Details (#228): one setting for every piece
    /// and every place, as `rows` is.
    formulas: bool,
    /// The piece being dragged (#228), while the pointer holds it.
    drag: Option<Drag>,
    /// The list of the other horses beside Equip is open (#356).
    others: bool,
    /// The horse of that list the pointer or the focus is on, and its
    /// entry: what equipping there would do shows beside it.
    offered: Option<(Entity, usize)>,
}

/// A piece being dragged (#228): where it was taken from, and the horse or
/// slot under the pointer that it may be let go on, if it is over one.
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Drag {
    pub piece: Piece,
    /// The cell it was taken from, which it goes back to if let go where
    /// it may not go (#363).
    pub from: Entity,
    pub onto: Option<Onto>,
}

/// A horse or one of its slots that a dragged piece may be let go on, and
/// the cell that is it.
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Onto {
    pub cell: Entity,
    pub horse: usize,
    pub slot: Option<usize>,
}

impl Shell {
    pub fn new(places: usize) -> Shell {
        Shell {
            place: 0,
            showing: true,
            horse: 0,
            marked: true,
            tabs: vec![0; places],
            details: vec![None; places],
            pointed: None,
            log_hidden: false,
            rows: card::Rows::All,
            hint: None,
            dismissed: Vec::new(),
            hints_off: false,
            owed: false,
            told: false,
            numbers_off: false,
            formulas: false,
            drag: None,
            others: false,
            offered: None,
        }
    }

    /// Whether the list of the other horses is open beside Equip: while a
    /// piece's card is pinned in view, as Equip's other controls work.
    pub fn others_open(&self) -> bool {
        self.others && self.hand().open.is_some()
    }

    /// The horse of that list the pointer or the focus is on.
    pub fn offered(&self) -> Option<(Entity, usize)> {
        self.offered.filter(|_| self.others_open())
    }

    pub fn offer(&mut self, at: Option<(Entity, usize)>) {
        self.offered = at;
    }

    /// A horse chosen from the list beside Equip (#356): the piece whose
    /// card is open is equipped on it, as Equip would for it, and it is
    /// selected, so the card's comparison follows it. The list closes.
    /// False, and nothing changes, where it may not go.
    pub fn equip_on(&mut self, game: &mut Game, horse: usize) -> bool {
        let Some(piece) = self.hand().open else {
            return false;
        };
        let Some(hand) = game.drop_hand(piece, horse, None) else {
            return false;
        };
        let Some(to) = game.equipped_at(hand) else {
            return false;
        };
        game.act(Action::Equip, hand);
        self.piece_moved(piece, to);
        self.inspect(horse, None);
        self.others = false;
        self.offered = None;
        true
    }

    /// The piece being dragged, if one is.
    pub fn dragging(&self) -> Option<Drag> {
        self.drag
    }

    /// A drag begins on a piece (#228), taken from a cell.
    pub fn take(&mut self, piece: Piece, from: Entity) {
        self.drag = Some(Drag { piece, from, onto: None });
    }

    /// The dragged piece is over a place it may go, or over none.
    pub fn hold_over(&mut self, onto: Option<Onto>) {
        if let Some(drag) = &mut self.drag {
            drag.onto = onto;
        }
    }

    /// The pointer lets the piece go. Returns the drag that ended.
    pub fn let_go(&mut self) -> Option<Drag> {
        self.drag.take()
    }

    pub fn hint(&self) -> Option<&Hint> {
        self.hint.as_ref()
    }

    /// Whether the player put this lesson's hint away.
    pub fn put_away(&self, lesson: &str) -> bool {
        self.dismissed.iter().any(|put_away| put_away == lesson)
    }

    /// Whether the player skipped every hint.
    pub fn hints_off(&self) -> bool {
        self.hints_off
    }

    /// A lesson asks for its hint. It is shown, and true comes back, unless
    /// another is showing (one at a time: the lesson asks again), or the
    /// player put this lesson's hint away, or skipped them all.
    pub fn raise_hint(&mut self, hint: Hint) -> bool {
        if self.hints_off || self.hint.is_some() || self.dismissed.contains(&hint.lesson) {
            return false;
        }
        self.hint = Some(hint);
        true
    }

    /// The player puts the hint away: Got it, or Escape. Its lesson's hint
    /// does not come back by itself. False when no hint was showing.
    pub fn dismiss_hint(&mut self) -> bool {
        match self.hint.take() {
            Some(hint) => {
                self.dismissed.push(hint.lesson);
                true
            }
            None => false,
        }
    }

    /// The thing the hint was about has been done: it goes, and its lesson
    /// is not counted as put away.
    pub fn finish_hint(&mut self) {
        self.hint = None;
    }

    /// Skip all hints: the one showing goes, and none is shown after it.
    /// The first time, the player is owed where they went (#350).
    pub fn skip_hints(&mut self) {
        self.dismiss_hint();
        self.hints_off = true;
        self.owed = !self.told;
    }

    /// Whether Skip all hints is still to be answered, and the hint's line
    /// is free for it. Never while hints are on.
    pub fn owes_parting(&self) -> bool {
        self.owed && self.hints_off && self.hint.is_none()
    }

    /// Where the hints went, said on the hint's line though hints are off
    /// (#350): once. False, and nothing is said, when it is not owed.
    pub fn say_parting(&mut self, hint: Hint) -> bool {
        if !self.owes_parting() {
            return false;
        }
        self.hint = Some(hint);
        (self.owed, self.told) = (false, true);
        true
    }

    /// Whether the line showing is the answer to Skip all hints.
    pub fn parting(&self) -> bool {
        self.hint.as_ref().is_some_and(|hint| hint.lesson == PARTING)
    }

    /// Hints are on again: nothing is owed, and what was said of where they
    /// went has nothing left to say.
    fn hints_back(&mut self) {
        self.hints_off = false;
        self.owed = false;
        if self.parting() {
            self.finish_hint();
        }
    }

    /// Whether the player is at what a hint is about, where that is
    /// somewhere to be: a place that is open, or a tab of it that is chosen.
    pub fn at(&self, about: About) -> bool {
        match about {
            About::Place(place) => self.showing && self.place == place,
            About::Tab(place, tab) => self.showing && self.place == place && self.tabs[place] == tab,
            _ => false,
        }
    }

    /// Hints are shown again, those put away too: the dev key's. The
    /// player's own way is the Hints switch and the Journal's list (#300).
    pub fn forget_hints(&mut self) {
        self.dismissed.clear();
        self.hints_back();
    }

    /// The player asked for this hint again, from the Journal's list
    /// (#300): it is shown in the place of any other, though it was put
    /// away and though the hints are off, and is no longer put away.
    pub fn show_again(&mut self, hint: Hint) {
        self.dismissed.retain(|put_away| *put_away != hint.lesson);
        self.hint = Some(hint);
    }

    /// Which rows the comparison's table has, pinned and under the pointer.
    pub fn rows(&self) -> card::Rows {
        self.rows
    }

    /// Whether a switch is on.
    pub fn is_on(&self, switch: Switch) -> bool {
        match switch {
            Switch::ChangedOnly => self.rows == card::Rows::Changed,
            Switch::Hints => !self.hints_off,
            Switch::DamageNumbers => !self.numbers_off,
            Switch::Details => self.formulas,
            Switch::OtherHorses => self.others_open(),
        }
    }

    /// A press of a switch (#349). Changed only and Details do nothing while
    /// no piece's card is pinned in view, where they would change something
    /// the player cannot see. Hints off takes the one showing off its line
    /// without putting it away (#300), so it is back when they are on.
    pub fn press_switch(&mut self, switch: Switch) {
        match switch {
            // The switch says what it does itself: turned off here, where
            // the hints went is not owed (#350).
            Switch::Hints if self.hints_off => self.hints_back(),
            Switch::Hints => {
                self.hints_off = true;
                self.finish_hint();
            }
            Switch::ChangedOnly if self.hand().open.is_some() => {
                self.rows = match self.rows {
                    card::Rows::All => card::Rows::Changed,
                    card::Rows::Changed => card::Rows::All,
                };
            }
            Switch::ChangedOnly => {}
            Switch::Details if self.hand().open.is_some() => self.formulas = !self.formulas,
            Switch::Details => {}
            Switch::OtherHorses if self.hand().open.is_some() => self.others = !self.others,
            Switch::OtherHorses => {}
            Switch::DamageNumbers => self.numbers_off = !self.numbers_off,
        }
    }

    /// Whether the fight log is over the scene (#306): on the road, with
    /// every place closed. Its lines can be hidden; its heading stays, with
    /// the key that brings them back.
    pub fn log_in_view(&self, game: &Game) -> bool {
        !game.in_town() && !self.showing
    }

    pub fn log_hidden(&self) -> bool {
        self.log_hidden
    }

    /// Whether the list of things to try is over the scene (#303): in town,
    /// while there is something on it, and while hints are on. It is put
    /// away with them, and its entries are still ticked and paid behind it.
    pub fn list_in_view(&self, game: &Game) -> bool {
        game.lessons.list_showing() && !self.hints_off
    }

    /// A press of the fight log's key. It does nothing while the log is not
    /// in view, where it would change something the player cannot see.
    pub fn press_log_key(&mut self, game: &Game) {
        if self.log_in_view(game) {
            self.log_hidden = !self.log_hidden;
        }
    }

    pub fn place(&self) -> usize {
        self.place
    }

    pub fn horse(&self) -> usize {
        self.horse
    }

    /// The horse outlined in the scene, with its tag lit: the selected one,
    /// unless a press on the background has unselected it (#366).
    pub fn marked(&self) -> Option<usize> {
        self.marked.then_some(self.horse)
    }

    /// Whether the Gear tab shows this horse's gear (#314): the selected
    /// horse's, and no other's.
    pub fn wears(&self, horse: usize) -> bool {
        horse == self.horse
    }

    pub fn tab(&self, place: usize) -> usize {
        self.tabs[place]
    }

    /// What an action would be done to now: the selected horse, and the
    /// piece whose card is open in the side panel that is showing.
    pub fn hand(&self) -> Hand {
        Hand {
            horse: self.horse,
            open: self
                .detail(self.place)
                .filter(|_| self.showing)
                .and_then(|detail| detail.item.piece()),
        }
    }

    pub fn open(&mut self, place: usize) {
        if place < self.tabs.len() {
            self.place = place;
            self.showing = true;
        }
    }

    /// Whether a place is showing. When none is, the scene has its room.
    pub fn is_open(&self) -> bool {
        self.showing
    }

    /// Put the place away. It is found as it was left when it is opened
    /// again. The caller sees to it that this happens on the road only.
    pub fn close(&mut self) {
        self.showing = false;
    }

    /// A press on a place in the bar. It opens that place, except that on
    /// the road a press on the place already showing closes it.
    pub fn press(&mut self, place: usize, in_town: bool) {
        if !in_town && self.showing && place == self.place {
            self.close();
        } else {
            self.open(place);
        }
    }

    /// A press on the scene away from the horses. It unselects the horse
    /// (#366). On the road it also puts the open place away (#307), whatever
    /// is open in it, and the scene has its room back; in town a place is
    /// always open.
    pub fn press_scene(&mut self, in_town: bool) {
        self.marked = false;
        if !in_town {
            self.close();
        }
    }

    pub fn choose_tab(&mut self, place: usize, tab: usize) {
        if place < self.tabs.len() {
            self.tabs[place] = tab;
        }
    }

    /// A press on a horse (#311): it is selected, and its view opens in the
    /// side panel of the place the player is in, which is opened if none
    /// is showing. A piece's card pinned there stays instead, and is
    /// compared on that horse.
    pub fn inspect(&mut self, horse: usize, opener: Option<Entity>) {
        self.horse = horse;
        self.marked = true;
        self.showing = true;
        let place = self.place;
        if self.detail(place).is_some_and(|detail| detail.item.piece().is_some()) {
            return;
        }
        self.details[place] = Some(Detail {
            tab: self.tabs[place],
            item: Item::Horse(horse),
            opener,
        });
    }

    /// A press on a part of a horse's view (#311): the tab where that part
    /// is changed opens, in its place. The view, if it is open there, is put
    /// away, as it stands over what the tab changes things with.
    pub fn go_to(&mut self, place: usize, tab: usize) {
        self.open(place);
        self.choose_tab(place, tab);
        // On whichever tab it was opened: it is not to come back with it.
        if place < self.details.len()
            && self.details[place].is_some_and(|detail| matches!(detail.item, Item::Horse(_)))
        {
            self.drop_detail(place);
        }
    }

    /// The detail showing in a place: the one opened from its current tab.
    pub fn detail(&self, place: usize) -> Option<Detail> {
        self.details[place].filter(|detail| detail.tab == self.tabs[place])
    }

    /// A new card closes the list of the other horses, which was for the
    /// last one.
    pub fn show_detail(&mut self, place: usize, detail: Detail) {
        if place < self.details.len() {
            self.details[place] = Some(detail);
            self.others = false;
        }
    }

    /// Put away whatever detail a place has open, on any of its tabs.
    pub fn drop_detail(&mut self, place: usize) {
        if place < self.details.len() {
            self.details[place] = None;
        }
    }

    /// A piece has moved: every card that was open on it stays on it, in
    /// whichever place it is open.
    pub fn piece_moved(&mut self, from: Piece, to: Piece) {
        for detail in self.details.iter_mut().flatten() {
            if detail.item == Item::from(from) {
                detail.item = to.into();
            }
        }
    }

    /// Put away every card open on a place in the cargo that holds
    /// nothing: the cargo has `carried` pieces.
    pub fn forget_carried_from(&mut self, carried: usize) {
        for detail in &mut self.details {
            if detail.is_some_and(|detail| matches!(detail.item, Item::Carried(at) if at >= carried)) {
                *detail = None;
            }
        }
    }

    /// The piece whose card the pointer may show: the one it rests on, when
    /// there is a piece there and its card is not pinned beside it already.
    pub fn points_at(&self, game: &Game, piece: Piece) -> Option<Piece> {
        (game.piece(piece).is_some() && self.hand().open != Some(piece)).then_some(piece)
    }

    pub fn pointed(&self) -> Option<(Entity, Piece)> {
        self.pointed
    }

    pub fn point(&mut self, at: Option<(Entity, Piece)>) {
        self.pointed = at;
    }

    /// Close the detail showing in the open place. Returns what opened it.
    pub fn close_detail(&mut self) -> Option<Entity> {
        let detail = self.detail(self.place)?;
        self.details[self.place] = None;
        detail.opener
    }
}

#[derive(Component)]
pub struct PlaceButton(pub usize);

#[derive(Component)]
pub struct TabButton {
    pub place: usize,
    pub tab: usize,
}

/// The list under Another horse on a card (#356), open or not.
#[derive(Component)]
pub struct OtherHorses;

/// A horse in that list, under the card of a place: choosing it equips the
/// card's piece on it.
#[derive(Component)]
pub struct EquipOn {
    pub place: usize,
    pub horse: usize,
}

/// A horse's tag under it in the scene (#356): its name, key and health.
/// Not a control; the horse itself is pressed.
#[derive(Component)]
pub struct HorseTag(pub usize);

/// What a tag stands in, kept under its horse's feet: the tag is centred
/// in it.
#[derive(Component)]
struct TagSpot(usize);

/// Where the boss is drawn in the scene (#360): a box over it that nothing
/// sees, for a lesson's pop-up to stand beside and point at.
#[derive(Component)]
struct BossSpot;

/// A part of a horse's view (#311): a press goes to the tab of the horses'
/// place where that part is changed.
#[derive(Component)]
pub struct PartButton {
    pub place: usize,
    pub tab: usize,
}

/// A gear slot of a horse: on the Gear tab, where only the selected
/// horse's are in view (#314), and a tile of the Inventory. Opens the piece
/// in the side panel.
#[derive(Component)]
pub struct SlotRow {
    pub place: usize,
    pub tab: usize,
    pub horse: usize,
    pub slot: usize,
}

/// A carried piece, where a list shows it: the newest in the cargo column,
/// which is beside every tab of its place, and each of them in the
/// Inventory. A press pins its card in the side panel, as a press on a slot
/// does. `at` is where the piece is in the cargo, from the newest.
#[derive(Component)]
pub struct CarriedCell {
    pub place: usize,
    pub tab: Option<usize>,
    pub at: usize,
}

/// A type's group of the Inventory: where the tiles of the carried pieces
/// of that type go. `grow_cargo` gives it a tile for every place in the
/// cargo, since the cargo has no most it can hold (#352).
#[derive(Component)]
struct CarriedGrid {
    place: usize,
    tab: usize,
    slot: usize,
}

/// The piece dropped by the trail whose report is showing, in that report
/// (#231). A press opens its card where the piece is now: in the cargo, or
/// on the horse that wears it.
#[derive(Component)]
pub struct ReportedCell {
    pub place: usize,
    pub tab: usize,
}

/// Something shown only while a piece is carried (true), or only while
/// none is (false); or only while the party has the piece of the report
/// that is showing; or, in the Inventory, only while the party has that
/// piece of that type, or has none of the type.
#[derive(Component)]
enum When {
    Carried(bool),
    Reported,
    Held(Piece, usize),
    NoneHeld(usize),
    /// In the list of hints (#300): while that many have been given, and
    /// while none has.
    Listed(usize),
    NoneListed,
}

/// A hint in the Journal's list (#300), by its place in the list. A press
/// shows it again on its control.
#[derive(Component)]
pub struct LessonRow(pub usize);

/// A horse's gear on the Gear tab: there while it is the selected horse's,
/// and no other horse's is (#314).
#[derive(Component)]
struct Wearer(usize);

/// A control from the map of the menus. `place` is the page it is on; a
/// control of the footer is on none.
#[derive(Component)]
pub struct ControlButton {
    pub id: usize,
    pub place: Option<usize>,
}

/// The line under a group of controls that says why those of them that work
/// cannot be used now (#233). It holds their `id`s, and is hidden while all
/// of them can be used.
#[derive(Component)]
struct Blocked(Vec<usize>);

/// Something shown only while the player is there.
#[derive(Component)]
enum Page {
    Place(usize),
    Tab(usize, usize),
    Detail(usize),
    /// The cargo column of a place. The detail opens in its room (#287).
    Cargo(usize),
    /// The room of the comparison beside the Inventory, while no piece is
    /// pinned in it: the grid keeps its width (#308).
    Compare(usize),
    /// The actions of the item card, in a place's side panel: there while
    /// the detail is a piece of gear (#228).
    Card(usize),
    /// The parts of a horse's view, in a place's side panel: there while
    /// the detail is a horse (#311).
    HorseParts(usize),
    /// What the hint's line says with every place closed: how to open a
    /// place.
    RoadView,
    /// The hint, as a pop-up by its control (#353).
    Hint,
}

/// The shell's hint: the pop-up whose words follow `Shell::hint`.
#[derive(Component)]
struct HintLine;

/// How far a pop-up has come up (#353), and the hint it came up for.
#[derive(Default)]
struct Rising {
    hint: Option<Hint>,
    secs: f32,
}

/// What a scroll key moves (#304): the page of a tab, or what a place's side
/// panel says.
#[derive(Component, Clone, Copy, PartialEq, Eq)]
enum Scrolled {
    Tab(usize, usize),
    Detail(usize),
}

/// The line under what a side panel says that names the scroll keys, there
/// only while its words do not fit.
#[derive(Component)]
struct ScrollHint(Entity);

/// The card the pointer shows beside the piece it rests on (#228). It is
/// over the shell and out of its layout, and the pointer passes through it.
#[derive(Component)]
struct HoverCard;

/// The whole shell: everything below the scene.
#[derive(Component)]
struct ShellRoot;

/// What a press on the scene lands on (#307): the whole window, behind
/// every menu.
#[derive(Component)]
struct SceneCatcher;

/// The trail tracker's marks, in the footer: one per fight of the trail.
#[derive(Component)]
struct TrailTracker;

/// Text that follows the game. On a text it sets that text; on a counter or
/// a row it sets the value.
#[derive(Component)]
struct Live(Fact);

#[derive(Clone, Copy)]
enum Fact {
    Coins,
    Health(usize),
    SelectedHorse,
    SelectedStats,
    /// The piece a horse wears in a slot, and the line under it.
    Worn(usize, usize),
    WornLine(usize, usize),
    Detail(usize),
    /// A part of the horse's view open in a place: what the horse has of
    /// that tab of the horses' place.
    HorsePart(usize, usize),
    /// What the horse whose view is open in a place did on the last trail.
    HorseTrail(usize),
    /// The comparison of the piece the pointer rests on.
    Hovered,
    /// What the comparison's room says while no piece is open in it.
    CompareHint,
    /// The carried piece as a list shows it: its name and what it is.
    CarriedName,
    CarriedKind,
    /// On a tile of the Inventory: whose a worn piece is.
    WornBy(usize),
    /// The piece of the report that is showing: its name, what it is, and
    /// where it is now.
    ReportedName,
    ReportedKind,
    ReportedWhere,
    Abilities,
    Report,
    ReportSide,
    /// The heading of the fight log over the road, with its key.
    LogHeading,
    Footer,
    /// A hint in the list: what it says, and under that what became of it,
    /// where it applies and whether a press shows it now.
    LessonWords(usize),
    LessonLine(usize),
}

/// The fight log over the road scene (#306). It is out of the shell's
/// layout and the pointer passes through it.
#[derive(Component)]
struct FightLog;

/// A line of the fight log, counted from the top, and what it says now
/// with how faint it is.
#[derive(Component)]
struct LogLine(usize, String, f32);

/// The list of things to try (#303), over the town scene. It is out of the
/// shell's layout and the pointer passes through it: an entry is done on
/// the real controls, never on the list.
#[derive(Component)]
struct ToTry;

/// An entry of the list, by its place in `lessons.ron`: its row, the box
/// that is filled when it is done, and its line.
#[derive(Component)]
struct TryRow(usize);

#[derive(Component)]
struct TryBox(usize);

#[derive(Component)]
struct TryWords(usize);

/// What an entry pays, or paid, at the end of its row.
#[derive(Component)]
struct TryPays(usize);

pub struct ShellPlugin {
    /// Start with the sample hint showing (`--hint`). F6 toggles it.
    pub hint: bool,
}

impl Plugin for ShellPlugin {
    fn build(&self, app: &mut App) {
        let places = Places::load();
        let mut shell = Shell::new(places.pages().count());
        if self.hint {
            shell.raise_hint(sample_hint(&places, 0));
        }
        app.insert_resource(shell)
            .insert_resource(places)
            .init_resource::<Held>()
            .init_resource::<picture::LookImages>()
            .add_systems(Startup, spawn)
            .add_systems(
                Update,
                (
                    lose_hidden_focus,
                    arrows,
                    // Twenty is as many as one chain takes.
                    (keys, scroll, log_key, dev_hint).chain(),
                    escape,
                    follow_the_party,
                    (hold_over, point, offer).chain(),
                    (move_scene, place_tags, place_boss).chain(),
                    show_place,
                    see,
                    (mark_chosen, mark_targets, show_equip).chain(),
                    (
                        mark_unseen,
                        teach::teach.after(crate::fx::drops::Thrown),
                        point_out,
                        show_hint,
                        place_popup,
                    )
                        .chain(),
                    show_wearer,
                    (grow_cargo, show_carried, mark_lessons).chain(),
                    (show_hover_card, held::hold).chain(),
                    picture::show_pictures,
                    show_reasons,
                    show_facts,
                    (show_trail, show_log, show_list).chain(),
                    show_scrollbars,
                    follow_focus,
                )
                    .chain(),
            )
            .add_systems(
                Last,
                picture::forget_unheld_looks.run_if(bevy::time::common_conditions::on_real_timer(
                    std::time::Duration::from_secs(5),
                )),
            )
            .add_observer(on_activate)
            .add_observer(on_drag_start)
            .add_observer(on_drag_end);
    }
}

fn spawn(mut commands: Commands, tokens: Res<Tokens>, places: Res<Places>, game: Res<Game>) {
    let t = &*tokens;
    let gap = Val::Px(t.space.m);
    let edge = BorderColor::all(t.colour.panel_border.0);
    let band = |border: UiRect| {
        (
            Node {
                align_items: AlignItems::Center,
                column_gap: Val::Px(t.space.l),
                padding: UiRect::axes(Val::Px(t.space.l), Val::Px(t.space.s)),
                border,
                flex_shrink: 0.0,
                ..default()
            },
            BackgroundColor(t.colour.panel.0),
            edge,
        )
    };
    let spacer = || Node {
        flex_grow: 1.0,
        ..default()
    };
    let column = |width: f32| Node {
        width: Val::Px(width),
        flex_direction: FlexDirection::Column,
        padding: UiRect::all(Val::Px(t.space.m)),
        row_gap: Val::Px(t.space.s),
        border: UiRect::all(Val::Px(t.size.border)),
        flex_shrink: 0.0,
        ..default()
    };

    // Behind the menus, so only a press that no menu takes reaches it. The
    // fight log and the pointer's card let a press through, but neither is
    // ever under the pointer with a place open: the log is away, and the
    // card is up only while the pointer is on its piece.
    commands
        .spawn((
            Node {
                position_type: PositionType::Absolute,
                top: Val::Px(0.0),
                bottom: Val::Px(0.0),
                left: Val::Px(0.0),
                right: Val::Px(0.0),
                ..default()
            },
            GlobalZIndex(-1),
            SceneCatcher,
            // A piece dragged onto the scene may go to a horse (#356).
            Hovered::default(),
        ))
        .observe(on_scene_press);

    commands
        .spawn((
            Node {
                position_type: PositionType::Absolute,
                top: Val::Percent(t.layout.town_share * 100.0),
                bottom: Val::Px(0.0),
                left: Val::Px(0.0),
                right: Val::Px(0.0),
                flex_direction: FlexDirection::Column,
                ..default()
            },
            BackgroundColor(t.colour.screen.0),
            TabGroup::default(),
            ShellRoot,
        ))
        .with_children(|root| {
            // The bar: the places, in the order of the data file, then settings.
            root.spawn((
                Node {
                    align_items: AlignItems::FlexEnd,
                    padding: UiRect::horizontal(Val::Px(t.space.l)),
                    border: UiRect::vertical(Val::Px(t.size.border)),
                    flex_shrink: 0.0,
                    ..default()
                },
                BackgroundColor(t.colour.panel.0),
                edge,
            ))
            .with_children(|bar| {
                for (p, place) in places.places.iter().enumerate() {
                    bar.spawn((tab(t, &place.name, place.key.as_deref()), PlaceButton(p)));
                }
                bar.spawn(spacer());
                bar.spawn((
                    button(t, &places.settings.name, places.settings.key.as_deref()),
                    PlaceButton(places.settings_page()),
                ));
            });

            // The status strip. Bags, hold and the day keep their positions
            // until the systems behind them exist.
            root.spawn(band(UiRect::bottom(Val::Px(t.size.border))))
                .with_children(|strip| {
                    strip.spawn((counter(t, "coins", "0"), Live(Fact::Coins)));
                    strip.spawn(counter(t, "bags", "-"));
                    strip.spawn(counter(t, "hold", "-"));
                    strip.spawn(spacer());
                    strip.spawn(counter(t, "day", "-"));
                });

            // The hint's line (#291): one line under the strip that keeps
            // its height. With every place closed it is all that is left
            // between the strip and the footer, and it says there how to
            // open a place. The hint itself is a pop-up (#353).
            root.spawn(Node {
                height: Val::Px(t.size.hint_height),
                flex_shrink: 0.0,
                ..default()
            })
            .with_children(|line| {
                line.spawn((
                    Page::RoadView,
                    Node {
                        width: Val::Percent(100.0),
                        align_items: AlignItems::Center,
                        padding: UiRect::horizontal(Val::Px(t.space.l)),
                        display: Display::None,
                        ..default()
                    },
                ))
                .with_children(|road| {
                    road.spawn(dim(
                        t,
                        "On the road. Open a place to look and plan; press it again, the scene, or Escape, to give the scene its room back.",
                    ));
                });
            });

            // The places. All of them exist all the time; one is shown. The
            // area is clipped, so a place is covered and uncovered as the
            // scene moves instead of being squeezed.
            root.spawn(Node {
                flex_grow: 1.0,
                min_height: Val::Px(0.0),
                overflow: Overflow::clip(),
                ..default()
            })
            .with_children(|area| {
                for (p, place) in places.pages().enumerate() {
                    area.spawn((
                        Page::Place(p),
                        Node {
                            width: Val::Percent(100.0),
                            flex_direction: FlexDirection::Column,
                            flex_shrink: 0.0,
                            ..default()
                        },
                    ))
                    .with_children(|page| {
                        if !place.tabs.is_empty() {
                            page.spawn(Node {
                                padding: UiRect::horizontal(Val::Px(t.space.l)),
                                flex_shrink: 0.0,
                                ..default()
                            })
                            .with_children(|tabs| {
                                for (i, each) in place.tabs.iter().enumerate() {
                                    tabs.spawn((
                                        tab(t, &each.name, each.key.as_deref()),
                                        TabButton { place: p, tab: i },
                                    ));
                                }
                            });
                        }

                        page.spawn(Node {
                            flex_grow: 1.0,
                            min_height: Val::Px(0.0),
                            column_gap: gap,
                            // No room above: the hint's line is there (#291).
                            padding: UiRect {
                                top: Val::Px(0.0),
                                ..UiRect::all(Val::Px(t.space.m))
                            },
                            ..default()
                        })
                        .with_children(|body| {
                            body.spawn(Node {
                                flex_grow: 1.0,
                                min_width: Val::Px(0.0),
                                ..default()
                            })
                            .with_children(|pages| {
                                for (i, (heading, job, content, controls)) in
                                    place.pages().into_iter().enumerate()
                                {
                                    // A tab that does not fit scrolls, with
                                    // its bar beside it (#243): nothing is
                                    // cut off without saying so.
                                    let mut frame = pages.spawn((
                                        Page::Tab(p, i),
                                        Node {
                                            width: Val::Percent(100.0),
                                            ..default()
                                        },
                                    ));
                                    let mut scrolls = Entity::PLACEHOLDER;
                                    frame.with_children(|frame| {
                                        scrolls = frame
                                            .spawn((
                                                ScrollArea,
                                                Scrolled::Tab(p, i),
                                                Node {
                                                    flex_grow: 1.0,
                                                    min_width: Val::Px(0.0),
                                                    flex_direction: FlexDirection::Column,
                                                    padding: UiRect::all(Val::Px(t.space.m)),
                                                    row_gap: Val::Px(t.space.s),
                                                    overflow: Overflow {
                                                        x: OverflowAxis::Clip,
                                                        y: OverflowAxis::Scroll,
                                                    },
                                                    ..default()
                                                },
                                            ))
                                            .id();
                                        frame.spawn(scrollbar(t, scrolls));
                                    });
                                    frame
                                        .commands()
                                        .entity(scrolls)
                                        .with_children(|tab_page| {
                                            tab_page.spawn(title(t, &heading));
                                            tab_page.spawn(dim(t, job));
                                            // One horse is selected in every place,
                                            // and settings is not a place.
                                            if content == Content::Planned
                                                && p != places.settings_page()
                                            {
                                                tab_page.spawn((
                                                    text(t, ""),
                                                    Live(Fact::SelectedHorse),
                                                ));
                                            }
                                            spawn_content(tab_page, t, p, i, content, game.party.len(), game.lessons.defs().len());
                                            spawn_controls(tab_page, t, controls, Some(p), true);
                                        });
                                }
                            });

                            if place.horse_columns {
                                // The detail opens in the cargo's room, and
                                // a horse's view at its width, so opening
                                // one moves nothing on the tab beside it
                                // (#287). A piece's comparison is wider.
                                body.spawn((panel(t), Page::Cargo(p)))
                                    .insert(column(t.size.cargo_width))
                                    .with_children(|cargo| {
                                        cargo.spawn(dim(t, "cargo"));
                                        cargo.spawn((
                                            dim(t, card::NOTHING_CARRIED),
                                            When::Carried(false),
                                        ));
                                        spawn_carried(cargo, t, p, None);
                                        spawn_controls(cargo, t, &place.cargo, Some(p), true);
                                    });
                            }

                            if place.content == Content::Inventory {
                                body.spawn((panel(t), Page::Compare(p)))
                                    .insert(column(t.size.compare_width))
                                    .with_children(|compare| {
                                        compare.spawn(dim(t, "compare"));
                                        compare.spawn((text(t, ""), Live(Fact::CompareHint)));
                                    });
                            }

                            let side = if place.horse_columns {
                                t.size.cargo_width
                            } else {
                                t.size.side_panel_width
                            };
                            body.spawn((side_panel(t, "Detail", side), Page::Detail(p)))
                                .with_children(|side| {
                                    // What the detail says scrolls where it
                                    // does not fit, so that the card's
                                    // actions under it are always in view.
                                    let mut scrolls = Entity::PLACEHOLDER;
                                    side.spawn(Node {
                                        flex_grow: 1.0,
                                        min_height: Val::Px(0.0),
                                        ..default()
                                    })
                                    .with_children(|frame| {
                                        scrolls = frame
                                            .spawn((
                                                ScrollArea,
                                                Scrolled::Detail(p),
                                                Node {
                                                    flex_grow: 1.0,
                                                    min_width: Val::Px(0.0),
                                                    flex_direction: FlexDirection::Column,
                                                    overflow: Overflow {
                                                        x: OverflowAxis::Clip,
                                                        y: OverflowAxis::Scroll,
                                                    },
                                                    ..default()
                                                },
                                            ))
                                            .with_children(|words| {
                                                words.spawn((
                                                    text(t, ""),
                                                    Live(Fact::Detail(p)),
                                                ));
                                                spawn_horse_parts(words, t, &places, p);
                                            })
                                            .id();
                                        frame.spawn(scrollbar(t, scrolls));
                                    });
                                    // The keys that read the rest, printed
                                    // where the rest is (#304).
                                    side.spawn((
                                        dim(t, &scroll_hint(&places)),
                                        ScrollHint(scrolls),
                                    ))
                                    .insert(Node {
                                        display: Display::None,
                                        flex_shrink: 0.0,
                                        ..default()
                                    });
                                    // The card's actions, the same in every
                                    // place a piece can be opened in (#228).
                                    if p != places.settings_page() {
                                        side.spawn((
                                            Node {
                                                flex_direction: FlexDirection::Column,
                                                row_gap: Val::Px(t.space.s),
                                                flex_shrink: 0.0,
                                                ..default()
                                            },
                                            Page::Card(p),
                                        ))
                                        .with_children(|actions| {
                                            spawn_card_controls(actions, t, &places, p);
                                        });
                                    }
                                });
                        });
                    });
                }
            });

            // The footer: what the party is doing, and the one departure.
            root.spawn(band(UiRect::top(Val::Px(t.size.border))))
                .with_children(|footer| {
                    // The trail tracker (#253), first so that nothing beside
                    // it can move it: how many fights are left, at a glance.
                    footer
                        .spawn(Node {
                            align_items: AlignItems::Center,
                            column_gap: Val::Px(t.space.s),
                            flex_shrink: 0.0,
                            ..default()
                        })
                        .with_children(|trail| {
                            trail.spawn((
                                Node {
                                    align_items: AlignItems::Center,
                                    column_gap: Val::Px(t.space.s),
                                    ..default()
                                },
                                TrailTracker,
                            ));
                            trail.spawn(dim(t, "boss"));
                        });
                    footer.spawn((text(t, ""), Live(Fact::Footer)));
                    footer.spawn(spacer());
                    // One above the other and never squeezed: the footer
                    // has one line's height and little width to spare.
                    footer
                        .spawn(Node {
                            flex_direction: FlexDirection::Column,
                            flex_shrink: 0.0,
                            ..default()
                        })
                        .with_children(|reasons| {
                            spawn_reasons(reasons, t, &places.footer, false);
                        });
                    for control in &places.footer {
                        spawn_control(footer, t, control, None);
                    }
                });
        });

    commands.spawn((
        panel(t),
        HoverCard,
        GlobalZIndex(5),
        Pickable::IGNORE,
        Visibility::Hidden,
        children![(text(t, ""), Live(Fact::Hovered), Pickable::IGNORE)],
    ))
    .insert(Node {
        position_type: PositionType::Absolute,
        display: Display::None,
        // It is the comparison, which has a table to fit.
        width: Val::Px(t.size.compare_width),
        flex_direction: FlexDirection::Column,
        padding: UiRect::all(Val::Px(t.space.m)),
        border: UiRect::all(Val::Px(t.size.border)),
        ..default()
    });

    // A lesson's pop-up (#353): over everything but the hover card's
    // comparison, beside the control it is about. `place_popup` puts it.
    commands
        .spawn((popup(t, "", None, ""), Page::Hint, HintLine, GlobalZIndex(4)))
        .entry::<Node>()
        .and_modify(|mut node| {
            node.position_type = PositionType::Absolute;
            node.display = Display::None;
        });

    // Each horse's tag (#356), under its feet in the scene, where the strip
    // had a button for it. Under the menus, as the damage numbers are, so a
    // place that is opening covers it. `place_tags` puts them.
    for (h, horse) in game.party.iter().enumerate() {
        commands.spawn((
            Node {
                position_type: PositionType::Absolute,
                width: Val::Px(0.0),
                justify_content: JustifyContent::Center,
                ..default()
            },
            TagSpot(h),
            GlobalZIndex(-1),
            Pickable::IGNORE,
            Visibility::Hidden,
            children![(tag(t, &horse.name, places.horse_key(h)), HorseTag(h), Live(Fact::Health(h)))],
        ));
    }
    // The boss's box (#360): `place_boss` keeps it over the boss.
    commands.spawn((
        Node { position_type: PositionType::Absolute, display: Display::None, ..default() },
        BossSpot,
        GlobalZIndex(-1),
        Pickable::IGNORE,
        Visibility::Hidden,
    ));

    // The fight log (#306): at the top left of the scene, above the party
    // and away from where an enemy falls. The newest line is the lowest.
    commands
        .spawn((
            Node {
                position_type: PositionType::Absolute,
                display: Display::None,
                top: Val::Px(t.space.l),
                left: Val::Px(t.space.l),
                width: Val::Px(t.size.fight_log_width),
                flex_direction: FlexDirection::Column,
                row_gap: Val::Px(t.space.xs),
                ..default()
            },
            FightLog,
            Pickable::IGNORE,
            Visibility::Hidden,
        ))
        .with_children(|log| {
            log.spawn((dim(t, ""), Live(Fact::LogHeading), Pickable::IGNORE));
            for line in 0..t.size.fight_log_lines {
                log.spawn((text(t, ""), LogLine(line, String::new(), 1.0), Pickable::IGNORE));
            }
        });

    // The things to try (#303): at the top right of the town scene, clear
    // of the gate and the horses, so it has a home of its own and moves no
    // control. A row for every entry in the file is spawned once and shows
    // while its entry is on the list.
    commands
        .spawn((panel(t), ToTry, Pickable::IGNORE, Visibility::Hidden))
        .insert(Node {
            position_type: PositionType::Absolute,
            display: Display::None,
            top: Val::Px(t.space.l),
            right: Val::Px(t.space.l),
            width: Val::Px(t.size.to_try_width),
            flex_direction: FlexDirection::Column,
            padding: UiRect::all(Val::Px(t.space.m)),
            row_gap: Val::Px(t.space.xs),
            border: UiRect::all(Val::Px(t.size.border)),
            ..default()
        })
        .with_children(|list| {
            list.spawn((dim(t, TO_TRY), Pickable::IGNORE));
            for entry in 0..game.lessons.entries().len() {
                list.spawn((
                    Node {
                        align_items: AlignItems::Center,
                        column_gap: Val::Px(t.space.m),
                        display: Display::None,
                        ..default()
                    },
                    TryRow(entry),
                    Pickable::IGNORE,
                ))
                .with_children(|row| {
                    row.spawn((
                        Node {
                            width: Val::Px(t.size.trail_mark),
                            height: Val::Px(t.size.trail_mark),
                            border: UiRect::all(Val::Px(t.size.border)),
                            flex_shrink: 0.0,
                            ..default()
                        },
                        BackgroundColor(Color::NONE),
                        BorderColor::all(t.colour.text_dim.0),
                        TryBox(entry),
                        Pickable::IGNORE,
                    ));
                    row.spawn((text(t, ""), TryWords(entry), Pickable::IGNORE)).insert(Node {
                        flex_grow: 1.0,
                        ..default()
                    });
                    row.spawn((text(t, ""), TryPays(entry), Pickable::IGNORE)).insert(Node {
                        width: Val::Px(t.size.to_try_pays),
                        flex_shrink: 0.0,
                        ..default()
                    });
                });
            }
        });
}

/// The bar beside a page that scrolls. It keeps its width whether or not it
/// shows, so the page does not move when it appears.
fn scrollbar(t: &Tokens, target: Entity) -> impl Bundle + use<> {
    (
        Node {
            width: Val::Px(t.size.scrollbar_width),
            flex_shrink: 0.0,
            ..default()
        },
        Scrollbar::new(target, ControlOrientation::Vertical, t.size.control_height),
        BackgroundColor(t.colour.panel.0),
        Visibility::Hidden,
        children![(
            ScrollbarThumb {
                border_radius: BorderRadius::all(Val::Px(t.size.radius)),
                border: UiRect::ZERO,
            },
            BackgroundColor(t.colour.control_border.0),
        )],
    )
}

/// A scrollbar shows only while its page does not fit, and so does the line
/// that names the scroll keys.
fn show_scrollbars(
    mut bars: Query<(&Scrollbar, &mut Visibility)>,
    mut hints: Query<(&ScrollHint, &mut Node)>,
    areas: Query<&ComputedNode, With<ScrollArea>>,
) {
    for (hint, mut node) in &mut hints {
        let Ok(area) = areas.get(hint.0) else {
            continue;
        };
        let wanted = if area.content_size().y > area.size().y + 0.5 {
            Display::Flex
        } else {
            Display::None
        };
        if node.display != wanted {
            node.display = wanted;
        }
    }
    for (bar, mut visibility) in &mut bars {
        let Ok(area) = areas.get(bar.target) else {
            continue;
        };
        let wanted = if area.content_size().y > area.size().y + 0.5 {
            Visibility::Inherited
        } else {
            Visibility::Hidden
        };
        if *visibility != wanted {
            *visibility = wanted;
        }
    }
}

/// A control that takes the keyboard focus is brought into view, so Tab
/// never lands on something that cannot be seen.
fn follow_focus(mut commands: Commands, focus: Res<InputFocus>) {
    if focus.is_changed()
        && let Some(entity) = focus.get()
    {
        commands.trigger(ScrollIntoView { entity });
    }
}

fn spawn_content(
    page: &mut ChildSpawnerCommands,
    t: &Tokens,
    place: usize,
    tab: usize,
    content: Content,
    horses: usize,
    lessons: usize,
) {
    match content {
        Content::Planned => {
            page.spawn(reason(t, "This page is not built yet."));
        }
        Content::Hints => {
            // A row for every lesson there is, and those that have been
            // given show, in the order they came (#300). A row is a
            // control: a press shows its hint again.
            page.spawn((dim(t, NO_HINTS), When::NoneListed));
            for row in 0..lessons {
                page.spawn((cell(t), LessonRow(row), When::Listed(row)))
                    .with_children(|cell| {
                        cell.spawn((text(t, ""), Live(Fact::LessonWords(row))));
                        cell.spawn((dim(t, ""), Live(Fact::LessonLine(row))));
                    });
            }
        }
        Content::Abilities => {
            page.spawn((text(t, ""), Live(Fact::Abilities)));
        }
        Content::Gear => {
            page.spawn((text(t, ""), Live(Fact::SelectedStats)));
            // The selected horse's gear and no other's (#314): a row for
            // each slot. Every horse's rows are here, and only the selected
            // horse's show. All the party's gear at once is the Inventory.
            for horse in 0..horses {
                page.spawn((
                    Node {
                        flex_direction: FlexDirection::Column,
                        row_gap: Val::Px(t.space.s),
                        margin: UiRect::top(Val::Px(t.space.s)),
                        flex_shrink: 0.0,
                        ..default()
                    },
                    Wearer(horse),
                ))
                .with_children(|slots| {
                    for slot in Slot::ALL {
                        let slot = slot.index();
                        let piece = Piece::Worn { horse, slot };
                        slots
                            .spawn((cell(t), SlotRow { place, tab, horse, slot }))
                            .with_children(|cell| {
                                spawn_pictured(cell, t, piece, |lines| {
                                    lines.spawn((text(t, ""), Live(Fact::Worn(horse, slot))));
                                    lines.spawn((dim(t, ""), Live(Fact::WornLine(horse, slot))));
                                });
                            });
                    }
                });
            }
        }
        Content::Inventory => {
            // A headed group for each type (#308). Every tile a group can
            // come to hold is here, and shows while its piece is held, so
            // the grid follows the game with nothing rebuilt. A tile is the
            // piece's picture and whose it is; the pointer on it says the
            // rest, against what the selected horse wears.
            for slot in Slot::ALL {
                let name = slot.name();
                let slot = slot.index();
                page.spawn(text(t, name)).insert(Node {
                    margin: UiRect::top(Val::Px(t.space.s)),
                    ..default()
                });
                page.spawn((dim(t, &card::none_held(name)), When::NoneHeld(slot)));
                page.spawn((
                    CarriedGrid { place, tab, slot },
                    Node {
                    display: Display::Grid,
                    grid_template_columns: RepeatedGridTrack::minmax(
                        GridTrackRepetition::AutoFill,
                        MinTrackSizingFunction::Px(t.size.inventory_cell),
                        MaxTrackSizingFunction::Px(t.size.inventory_cell),
                    ),
                    column_gap: Val::Px(t.space.s),
                    row_gap: Val::Px(t.space.s),
                    flex_shrink: 0.0,
                    ..default()
                    },
                ))
                .with_children(|grid| {
                    for horse in 0..horses {
                        let piece = Piece::Worn { horse, slot };
                        grid.spawn((
                            cell(t),
                            SlotRow { place, tab, horse, slot },
                            When::Held(piece, slot),
                        ))
                        .insert(tile(t))
                        .with_children(|cell| {
                            cell.spawn(picture_of(t, piece, t.size.inventory_picture));
                            cell.spawn((dim(t, ""), Live(Fact::WornBy(horse))));
                        });
                    }
                    // The carried pieces' tiles come after these, as the
                    // cargo comes to hold them (`grow_cargo`).
                });
            }
        }
        Content::Report => {
            page.spawn(Node {
                column_gap: Val::Px(t.space.l),
                margin: UiRect::top(Val::Px(t.space.s)),
                ..default()
            })
            .with_children(|columns| {
                // Two halves of fixed width: neither moves as the other fills.
                for fact in [Fact::Report, Fact::ReportSide] {
                    columns
                        .spawn(Node {
                            width: Val::Percent(50.0),
                            flex_direction: FlexDirection::Column,
                            row_gap: Val::Px(t.space.s),
                            ..default()
                        })
                        .with_children(|half| {
                            half.spawn((text(t, ""), Live(fact)));
                            // The drop, under the report of the trail that
                            // dropped it, while the party has it: the card
                            // the cargo shows (#228), for the latest report
                            // and for an earlier one alike (#231).
                            if matches!(fact, Fact::Report) {
                                half.spawn((cell(t), ReportedCell { place, tab }, When::Reported))
                                    .with_children(|cell| {
                                        cell.spawn((text(t, ""), Live(Fact::ReportedName)));
                                        spawn_lines(cell, t, reported_picture(t), |lines| {
                                            lines.spawn((dim(t, ""), Live(Fact::ReportedKind)));
                                            lines.spawn((dim(t, ""), Live(Fact::ReportedWhere)));
                                        });
                                    });
                            }
                        });
                }
            });
        }
    }
}

/// How a tile of the Inventory is laid out: a picture over a line.
fn tile(t: &Tokens) -> Node {
    Node {
        flex_direction: FlexDirection::Column,
        align_items: AlignItems::Center,
        justify_content: JustifyContent::Center,
        padding: UiRect::all(Val::Px(t.space.s)),
        row_gap: Val::Px(t.space.xs),
        border: UiRect::all(Val::Px(t.size.border)),
        border_radius: BorderRadius::all(Val::Px(t.size.radius)),
        overflow: Overflow::clip(),
        ..default()
    }
}

/// Every group of the Inventory has a tile for every place in the cargo
/// (#352). A tile shows while the piece at its place is of its group's type
/// (`show_carried`), so a tile is made once and never taken away: the cargo
/// only says which of them are in sight.
fn grow_cargo(
    mut commands: Commands,
    tokens: Res<Tokens>,
    game: Res<Game>,
    mut made: Local<usize>,
    grids: Query<(Entity, &CarriedGrid)>,
) {
    let wanted = game.cargo().len();
    if wanted <= *made || grids.is_empty() {
        return;
    }
    let t = &*tokens;
    for (entity, grid) in &grids {
        commands.entity(entity).with_children(|tiles| {
            for at in *made..wanted {
                let piece = Piece::Carried(at);
                tiles
                    .spawn((
                        cell(t),
                        CarriedCell { place: grid.place, tab: Some(grid.tab), at },
                        When::Held(piece, grid.slot),
                    ))
                    .insert(tile(t))
                    .with_children(|cell| {
                        cell.spawn(picture_of(t, piece, t.size.inventory_picture));
                        cell.spawn(dim(t, card::CARRIED));
                    });
            }
        });
    }
    *made = wanted;
}

/// The newest carried piece as a list shows it: the summary of its card.
/// There only while something is carried.
fn spawn_carried(
    parent: &mut ChildSpawnerCommands,
    t: &Tokens,
    place: usize,
    tab: Option<usize>,
) {
    parent
        .spawn((cell(t), CarriedCell { place, tab, at: 0 }, When::Carried(true)))
        .with_children(|cell| {
            cell.spawn((text(t, ""), Live(Fact::CarriedName)));
            spawn_pictured(cell, t, Piece::Carried(0), |lines| {
                lines.spawn((dim(t, ""), Live(Fact::CarriedKind)));
            });
        });
}

/// What a cell holds under the piece's name: its picture (#286), and beside
/// it the lines that say what it is. The name has the cell's whole width,
/// which the longest one needs at the smallest window (#287).
fn spawn_pictured(
    cell: &mut ChildSpawnerCommands,
    t: &Tokens,
    piece: Piece,
    lines: impl FnOnce(&mut ChildSpawnerCommands),
) {
    spawn_lines(cell, t, picture(t, piece), lines);
}

/// A picture, and beside it the lines that say what it is of.
fn spawn_lines(
    cell: &mut ChildSpawnerCommands,
    t: &Tokens,
    picture: impl Bundle,
    lines: impl FnOnce(&mut ChildSpawnerCommands),
) {
    cell.spawn(Node {
        align_items: AlignItems::Center,
        column_gap: Val::Px(t.space.s),
        ..default()
    })
    .with_children(|across| {
        across.spawn(picture);
        across
            .spawn(Node {
                flex_grow: 1.0,
                flex_basis: Val::Px(0.0),
                min_width: Val::Px(0.0),
                flex_direction: FlexDirection::Column,
                row_gap: Val::Px(t.space.xs),
                ..default()
            })
            .with_children(lines);
    });
}

/// Under a horse's sheet in a side panel (#311): a part for each tab of the
/// horses' place, headed by a control that goes to that tab, since that is
/// where the part is changed, with the keys that do the same printed on it.
fn spawn_horse_parts(parent: &mut ChildSpawnerCommands, t: &Tokens, map: &Places, p: usize) {
    let Some(horses) = map.places.iter().position(|place| place.horse_columns) else {
        return;
    };
    let place = &map.places[horses];
    parent
        .spawn((
            Node {
                flex_direction: FlexDirection::Column,
                align_items: AlignItems::FlexStart,
                row_gap: Val::Px(t.space.s),
                margin: UiRect::top(Val::Px(t.space.m)),
                flex_shrink: 0.0,
                ..default()
            },
            Page::HorseParts(p),
        ))
        .with_children(|parts| {
            for (tab, page) in place.tabs.iter().enumerate() {
                if matches!(page.content, Content::Report | Content::Inventory | Content::Hints) {
                    continue;
                }
                let keys: Vec<&str> =
                    [place.key.as_deref(), page.key.as_deref()].into_iter().flatten().collect();
                let keys = keys.join(", ");
                parts.spawn((
                    button(t, &page.name, (!keys.is_empty()).then_some(keys.as_str())),
                    PartButton { place: horses, tab },
                ));
                parts.spawn((text(t, ""), Live(Fact::HorsePart(p, tab))));
            }
            parts.spawn(dim(t, "LAST TRAIL"));
            parts.spawn((text(t, ""), Live(Fact::HorseTrail(p))));
        });
}

/// A control of the map, with the key that works it printed on it. One that
/// is not built is drawn as unavailable and can still be pressed.
fn spawn_control(
    parent: &mut ChildSpawnerCommands,
    t: &Tokens,
    control: &Control,
    place: Option<usize>,
) {
    let mut entity = parent.spawn((
        button(t, &control.name, control.key.as_deref()),
        ControlButton {
            id: control.id,
            place,
        },
    ));
    if !control.built() {
        entity.insert(NotBuilt);
    }
}

/// The controls of a page, in the order of the map, wrapping as they must.
/// `hint` is whether the line about those not built says that a press says
/// more, where there is room for it.
fn spawn_controls(
    parent: &mut ChildSpawnerCommands,
    t: &Tokens,
    controls: &[Control],
    place: Option<usize>,
    hint: bool,
) {
    if controls.is_empty() {
        return;
    }
    parent
        .spawn(Node {
            flex_wrap: FlexWrap::Wrap,
            column_gap: Val::Px(t.space.s),
            row_gap: Val::Px(t.space.s),
            margin: UiRect::top(Val::Px(t.space.s)),
            flex_shrink: 0.0,
            ..default()
        })
        .with_children(|row| {
            for control in controls {
                spawn_control(row, t, control, place);
            }
        });
    spawn_reasons(parent, t, controls, hint);
}

/// The card's actions (#228): Equip, and beside it Another horse with its
/// list of the horses under it (#356), then the rest, wrapping as they
/// must, and under them all why any cannot be used.
fn spawn_card_controls(parent: &mut ChildSpawnerCommands, t: &Tokens, places: &Places, place: usize) {
    let row = || Node {
        flex_wrap: FlexWrap::Wrap,
        align_items: AlignItems::FlexStart,
        column_gap: Val::Px(t.space.s),
        row_gap: Val::Px(t.space.s),
        margin: UiRect::top(Val::Px(t.space.s)),
        flex_shrink: 0.0,
        ..default()
    };
    let column = || Node {
        flex_direction: FlexDirection::Column,
        row_gap: Val::Px(t.space.s),
        ..default()
    };
    let equips = |control: &Control| {
        control.action == Some(Action::Equip) || control.switch == Some(Switch::OtherHorses)
    };
    parent.spawn(row()).with_children(|row| {
        for control in places.card.iter().filter(|control| equips(control)) {
            if control.switch != Some(Switch::OtherHorses) {
                spawn_control(row, t, control, Some(place));
                continue;
            }
            row.spawn(column()).with_children(|list| {
                spawn_control(list, t, control, Some(place));
                list.spawn((
                    OtherHorses,
                    Node {
                        display: Display::None,
                        ..column()
                    },
                    Visibility::Hidden,
                ))
                .with_children(|entries| {
                    for horse in line_order(places.horse_keys.len()) {
                        entries.spawn((button(t, "", None), EquipOn { place, horse }));
                    }
                });
            });
        }
    });
    parent.spawn(row()).with_children(|row| {
        for control in places.card.iter().filter(|control| !equips(control)) {
            spawn_control(row, t, control, Some(place));
        }
    });
    spawn_reasons(parent, t, &places.card, false);
}

/// Under a group of controls, why any of them cannot be used (#233): a line
/// that follows the game for those that work, and a line for those that are
/// not built, which does not change.
fn spawn_reasons(parent: &mut ChildSpawnerCommands, t: &Tokens, controls: &[Control], hint: bool) {
    let working: Vec<usize> = controls
        .iter()
        .filter(|control| control.action.is_some())
        .map(|control| control.id)
        .collect();
    if !working.is_empty() {
        parent.spawn((reason(t, ""), Blocked(working)));
    }
    for note in controls.iter().filter_map(|control| control.note.as_deref()) {
        parent.spawn(reason(t, note));
    }
    let unbuilt = unbuilt_line(controls, hint);
    if !unbuilt.is_empty() {
        parent.spawn(reason(t, &unbuilt));
    }
}

/// The tags under the horses in the scene, where they are on the screen.
type Tags<'w, 's> = Query<
    'w,
    's,
    (Entity, &'static HorseTag, &'static ComputedNode, &'static UiGlobalTransform, &'static InheritedVisibility),
>;

/// The horse whose tag is under a point of the window, in logical pixels
/// (#367, #368). The tags let the pointer through to the scene, so it is
/// found here.
fn tag_under(tags: &Tags, position: Vec2) -> Option<usize> {
    tags.iter()
        .find(|(_, _, node, transform, shown)| {
            shown.get() && !node.is_empty() && node.contains_point(**transform, position / node.inverse_scale_factor())
        })
        .map(|(_, tag, ..)| tag.0)
}

/// A press on a horse in the scene, or on its tag, selects it and opens its
/// view, in town and on the road (#356, #367).
/// Anywhere else on the scene it unselects the horse (#366) and closes the
/// place that is open on the road (#307).
fn on_scene_press(
    press: On<PointerPress>,
    game: Res<Game>,
    mut shell: ResMut<Shell>,
    horses: Res<HorseBox>,
    camera: Single<(&Camera, &GlobalTransform), With<Camera3d>>,
    tags: Tags,
) {
    if press.button != PointerButton::Primary {
        return;
    }
    let (camera, at) = *camera;
    let position = press.pointer.position;
    let ray = camera.viewport_to_world(at, position).ok();
    if let Some(horse) = tag_under(&tags, position).or_else(|| ray.and_then(|ray| horses.horse_on(ray))) {
        shell.inspect(horse, None);
    } else {
        shell.press_scene(game.in_town());
    }
}

/// One observer for every control of the shell: what a press does depends
/// on what the control is.
fn on_activate(
    activate: On<Activate>,
    mut shell: ResMut<Shell>,
    mut game: ResMut<Game>,
    mut focus: ResMut<InputFocus>,
    map: Res<Places>,
    places: Query<&PlaceButton>,
    tabs: Query<&TabButton>,
    parts: Query<&PartButton>,
    slots: Query<&SlotRow>,
    cells: (Query<&CarriedCell>, Query<&ReportedCell>),
    closes: Query<(), With<ClosePanel>>,
    hints: (Query<(), With<DismissHint>>, Query<(), With<SkipHints>>, Query<&LessonRow>),
    controls: Query<&ControlButton>,
    others: Query<(Entity, &EquipOn)>,
    mut focus_visible: ResMut<InputFocusVisible>,
) {
    let (carried, reported) = cells;
    let (dismisses, skips, listed) = hints;
    let entity = activate.entity;
    if let Ok((_, entry)) = others.get(entity) {
        shell.equip_on(&mut game, entry.horse);
    } else if dismisses.contains(entity) {
        if teach::got_it(&game, &shell) {
            shell.dismiss_hint();
        }
    } else if skips.contains(entity) {
        shell.skip_hints();
    } else if let Ok(row) = listed.get(entity) {
        teach::again(&mut game, &mut shell, &map, row.0);
    } else if let Ok(place) = places.get(entity) {
        shell.press(place.0, game.in_town());
    } else if let Ok(tab) = tabs.get(entity) {
        shell.choose_tab(tab.place, tab.tab);
    } else if let Ok(part) = parts.get(entity) {
        shell.go_to(part.place, part.tab);
    } else if let Ok(row) = slots.get(entity) {
        shell.show_detail(
            row.place,
            Detail {
                tab: row.tab,
                item: Item::Worn {
                    horse: row.horse,
                    slot: row.slot,
                },
                opener: Some(entity),
            },
        );
    } else if let Ok(cell) = carried.get(entity) {
        let tab = cell.tab.unwrap_or(shell.tab(cell.place));
        shell.show_detail(
            cell.place,
            Detail {
                tab,
                item: Item::Carried(cell.at),
                opener: Some(entity),
            },
        );
    } else if let Ok(cell) = reported.get(entity) {
        if let Some(piece) = game.reported_piece() {
            shell.show_detail(
                cell.place,
                Detail {
                    tab: cell.tab,
                    item: piece.into(),
                    opener: Some(entity),
                },
            );
        }
    } else if closes.contains(entity) {
        close_detail(&mut shell, &mut focus);
    } else if let Ok(button) = controls.get(entity) {
        let control = map.control(button.id);
        if let Some(switch) = control.switch {
            shell.press_switch(switch);
            // The list opened takes the focus on its first horse that may
            // take the piece, so its key then Enter equips there.
            if switch == Switch::OtherHorses && shell.others_open() {
                let place = button.place.unwrap_or(shell.place());
                let first = line_order(game.party.len()).into_iter().find_map(|horse| {
                    let may = horse != shell.horse()
                        && shell
                            .hand()
                            .open
                            .is_some_and(|piece| game.drop_hand(piece, horse, None).is_some());
                    others
                        .iter()
                        .find(|(_, entry)| may && entry.place == place && entry.horse == horse)
                        .map(|(entity, _)| entity)
                });
                if let Some(first) = first {
                    focus.set(first, FocusCause::Navigated);
                    focus_visible.0 = true;
                }
            }
            return;
        }
        match control.action {
            Some(action) => {
                let hand = shell.hand();
                // A card that is open stays on its piece when the piece
                // is moved, so the piece is still the handle (#228), and
                // goes back with it when the move is undone (#230).
                let moved = match action {
                    Action::Equip => game
                        .equipped_at(hand)
                        .map(|to| (hand.open.unwrap_or(Piece::Carried(0)), to)),
                    Action::Undo => game.can(action, hand).ok().and(game.undone_move()),
                    _ => None,
                };
                let turned = matches!(action, Action::Earlier | Action::Later)
                    && game.can(action, hand).is_ok();
                game.act(action, hand);
                if let Some((from, to)) = moved {
                    shell.piece_moved(from, to);
                }
                // A page turned is a page shown (#231): its key works from
                // anywhere, and opens the report it turned. A card open
                // there was of the other report's piece, and is put away.
                if turned && let Some(place) = button.place {
                    shell.open(place);
                    let report = map
                        .pages()
                        .nth(place)
                        .and_then(|page| page.tabs.iter().position(|tab| tab.content == Content::Report));
                    if let Some(tab) = report {
                        shell.choose_tab(place, tab);
                    }
                    if shell.detail(place).is_some_and(|detail| detail.item.piece().is_some()) {
                        shell.drop_detail(place);
                    }
                }
            }
            // It shows something, or is not built and says so: either way
            // in the side panel of the page it is on.
            None => {
                let place = button.place.unwrap_or(shell.place());
                shell.open(place);
                let tab = shell.tab(place);
                shell.show_detail(
                    place,
                    Detail {
                        tab,
                        item: Item::Control(button.id),
                        opener: Some(entity),
                    },
                );
            }
        }
    }
}

/// The key printed on a place, a horse or a control presses it, wherever the
/// player is; the key printed on a tab chooses it while its place is showing.
/// Space is the one conflict, since a focused button also answers Space:
/// while a control shows keyboard focus, Space presses that control and no
/// other.
fn keys(
    mut commands: Commands,
    keys: Res<ButtonInput<KeyCode>>,
    focus: Res<InputFocus>,
    focus_visible: Res<InputFocusVisible>,
    map: Res<Places>,
    game: Res<Game>,
    mut shell: ResMut<Shell>,
    places: Query<(Entity, &PlaceButton)>,
    tabs: Query<(Entity, &TabButton)>,
    controls: Query<(Entity, &ControlButton)>,
) {
    if keys.get_just_pressed().len() == 0 {
        return;
    }
    let down = |key: Option<&str>| {
        key.and_then(key_code)
            .is_some_and(|code| keys.just_pressed(code))
    };
    for (entity, place) in &places {
        if down(map.page_key(place.0)) {
            commands.trigger(Activate { entity });
        }
    }
    for (entity, tab) in &tabs {
        let showing = shell.is_open() && tab.place == shell.place();
        if showing && down(map.tab_key(tab.place, tab.tab)) {
            commands.trigger(Activate { entity });
        }
    }
    // A horse's key is a press on it in the scene (#356).
    for h in 0..map.horse_keys.len().min(game.party.len()) {
        if down(map.horse_key(h)) {
            shell.inspect(h, None);
        }
    }
    let on_a_control = focus_visible.0 && focus.get().is_some();
    // A control of the card is in every place's side panel: one press, not
    // one for each.
    let mut pressed: Vec<usize> = Vec::new();
    for (entity, button) in &controls {
        let key = map.control(button.id).key.as_deref();
        let Some(code) = key.and_then(key_code) else {
            continue;
        };
        if keys.just_pressed(code)
            && !(code == KeyCode::Space && on_a_control)
            && !pressed.contains(&button.id)
        {
            pressed.push(button.id);
            commands.trigger(Activate { entity });
        }
    }
}

/// Where a press of a scroll key leaves what was `at`: `way` is -1 up and 1
/// down, `room` is what is in view and `content` all there is to read. It
/// moves by `share` of the room and stops at either end.
fn scrolled(at: f32, way: f32, room: f32, content: f32, share: f32) -> f32 {
    (at + way * room * share).clamp(0.0, (content - room).max(0.0))
}

/// The scroll keys (#304) move what the side panel says while one is open,
/// and the page of the tab otherwise, so every line can be read by keys.
fn scroll(
    keys: Res<ButtonInput<KeyCode>>,
    map: Res<Places>,
    tokens: Res<Tokens>,
    shell: Res<Shell>,
    mut areas: Query<(&Scrolled, &ComputedNode, &mut ScrollPosition)>,
) {
    let down = |key: &str| key_code(key).is_some_and(|code| keys.just_pressed(code));
    let way = match (down(&map.scroll_keys.0), down(&map.scroll_keys.1)) {
        (true, false) => -1.0,
        (false, true) => 1.0,
        _ => return,
    };
    if !shell.is_open() {
        return;
    }
    let place = shell.place();
    let wanted = match shell.detail(place) {
        Some(_) => Scrolled::Detail(place),
        None => Scrolled::Tab(place, shell.tab(place)),
    };
    for (part, node, mut at) in &mut areas {
        if *part != wanted {
            continue;
        }
        let scale = node.inverse_scale_factor();
        let to = scrolled(
            at.y,
            way,
            node.size().y * scale,
            node.content_size().y * scale,
            tokens.size.scroll_page,
        );
        if at.y != to {
            at.y = to;
        }
    }
}

/// The fight log's key (#306) hides its lines and shows them again.
fn log_key(
    keys: Res<ButtonInput<KeyCode>>,
    map: Res<Places>,
    game: Res<Game>,
    mut shell: ResMut<Shell>,
) {
    if key_code(&map.log_key).is_some_and(|code| keys.just_pressed(code)) {
        shell.press_log_key(&game);
    }
}

/// How faint a line of the fight log is: the newest is 1, and each one
/// older is a step nearer `fade`, which the oldest of a full log has.
fn log_alpha(age: usize, lines: usize, fade: f32) -> f32 {
    if lines < 2 {
        return 1.0;
    }
    1.0 - (1.0 - fade) * age.min(lines - 1) as f32 / (lines - 1) as f32
}

/// Keep the fight log over the scene (#306): there on the road with every
/// place closed, its latest lines the lowest and the older ones fainter,
/// elements and stats in their colours as everywhere (#251).
fn show_log(
    mut commands: Commands,
    tokens: Res<Tokens>,
    game: Res<Game>,
    shell: Res<Shell>,
    mut log: Single<(&mut Node, &mut Visibility), With<FightLog>>,
    mut lines: Query<(Entity, &mut LogLine, &mut Text, &mut TextColor, &TextFont)>,
    children: Query<&Children>,
    runs: Query<(), With<Run>>,
) {
    let shown = shell.log_in_view(&game);
    let (display, visibility) = match shown {
        true => (Display::Flex, Visibility::Inherited),
        false => (Display::None, Visibility::Hidden),
    };
    if log.0.display != display {
        log.0.display = display;
    }
    if *log.1 != visibility {
        *log.1 = visibility;
    }
    if !shown {
        return;
    }
    let room = tokens.size.fight_log_lines;
    let latest = match shell.log_hidden() {
        true => Vec::new(),
        false => fight_log(&game, room),
    };
    let names = ink::Names::of(&game.data.gear);
    for (entity, mut line, mut text, mut colour, font) in &mut lines {
        let words = latest.get(line.0).copied().unwrap_or("");
        let age = latest.len().saturating_sub(line.0 + 1);
        let alpha = log_alpha(age, room, tokens.size.fight_log_fade);
        if line.1 == words && line.2 == alpha {
            continue;
        }
        (line.1, line.2) = (words.to_string(), alpha);
        let plain = tokens.colour.text.0.with_alpha(alpha);
        colour.0 = plain;
        // As `show_facts` does it: the text is what comes before the first
        // colour, and each run after that a span under it.
        let mut rest = ink::runs(words, &names);
        let first = match rest.first() {
            Some((_, None)) => rest.remove(0).0,
            _ => "",
        };
        text.0 = first.into();
        for child in children.get(entity).into_iter().flatten() {
            if runs.contains(*child) {
                commands.entity(*child).despawn();
            }
        }
        for (run, ink) in rest {
            let colour = match ink {
                Some(ink) => tokens.colour.ink(ink, &game.data.gear).with_alpha(alpha),
                None => plain,
            };
            commands.spawn((Run, TextSpan::new(run), font.clone(), TextColor(colour), ChildOf(entity)));
        }
    }
}

/// What stands over the list of things to try (#303).
const TO_TRY: &str = "TO TRY IN TOWN, IF YOU LIKE. EACH PAYS ONCE.";

/// What the end of an entry's row says: what it pays, or that it paid.
fn try_pays(def: &EntryDef, tick: Tick) -> String {
    match (tick.done, def.coins) {
        (true, coins) => format!("paid {coins}"),
        (false, 1) => "1 coin".into(),
        (false, coins) => format!("{coins} coins"),
    }
}

/// Keep the list of things to try over the town scene (#303): a row for
/// each entry on it, its box filled and its line fainter once it is done.
fn show_list(
    tokens: Res<Tokens>,
    game: Res<Game>,
    shell: Res<Shell>,
    mut list: Single<(&mut Node, &mut Visibility), (With<ToTry>, Without<TryRow>)>,
    mut rows: Query<(&TryRow, &mut Node), Without<ToTry>>,
    mut boxes: Query<(&TryBox, &mut BackgroundColor)>,
    mut lines: Query<(Option<&TryWords>, Option<&TryPays>, &mut Text, &mut TextColor), Or<(With<TryWords>, With<TryPays>)>>,
) {
    let shown = shell.list_in_view(&game);
    let (display, visibility) = match shown {
        true => (Display::Flex, Visibility::Inherited),
        false => (Display::None, Visibility::Hidden),
    };
    if list.0.display != display {
        list.0.display = display;
    }
    if *list.1 != visibility {
        *list.1 = visibility;
    }
    if !shown {
        return;
    }
    for (row, mut node) in &mut rows {
        let wanted = match game.lessons.tick(row.0).listed {
            true => Display::Flex,
            false => Display::None,
        };
        if node.display != wanted {
            node.display = wanted;
        }
    }
    for (mark, mut fill) in &mut boxes {
        let wanted = match game.lessons.tick(mark.0).done {
            true => tokens.colour.text.0,
            false => Color::NONE,
        };
        if fill.0 != wanted {
            fill.0 = wanted;
        }
    }
    for (words, pays, mut text, mut colour) in &mut lines {
        let (entry, pays) = match (words, pays) {
            (Some(words), _) => (words.0, false),
            (None, Some(pays)) => (pays.0, true),
            (None, None) => continue,
        };
        let (def, tick) = (&game.lessons.entries()[entry], game.lessons.tick(entry));
        let words = match pays {
            true => try_pays(def, tick),
            false => def.words.clone(),
        };
        if text.0 != words {
            text.0 = words;
        }
        let wanted = match tick.done {
            true => tokens.colour.text_dim.0,
            false => tokens.colour.text.0,
        };
        if colour.0 != wanted {
            colour.0 = wanted;
        }
    }
}

/// A control that goes out of sight gives up the keyboard focus (#244), so
/// Enter and Space never press something on a page that is not showing.
fn lose_hidden_focus(mut focus: ResMut<InputFocus>, nodes: Query<&ComputedNode>) {
    if let Some(entity) = focus.get()
        && nodes.get(entity).is_ok_and(|node| node.is_empty())
    {
        focus.clear();
    }
}

/// The arrow keys, and the way each moves the keyboard focus.
const ARROWS: [(KeyCode, CompassOctant); 4] = [
    (KeyCode::ArrowUp, CompassOctant::North),
    (KeyCode::ArrowDown, CompassOctant::South),
    (KeyCode::ArrowLeft, CompassOctant::West),
    (KeyCode::ArrowRight, CompassOctant::East),
];

/// Where the focus goes from `from`: the nearest of `parts` that way, by the
/// rule Bevy's own directional navigation scores with. None when nothing
/// lies that way, and the focus stays where it is.
fn towards(from: Entity, way: CompassOctant, parts: &[FocusableArea]) -> Option<Entity> {
    let origin = parts.iter().find(|part| part.entity == from)?;
    find_best_candidate(origin, way, parts, &AutoNavigationConfig::default())
}

/// An arrow key moves the keyboard focus to the nearest control that way
/// (#229): down a list, along a row of buttons, across to the next column.
/// With no focus in sight, the first press puts it on the first control of
/// the page that is showing, or on the open place in the bar when none is.
fn arrows(
    keys: Res<ButtonInput<KeyCode>>,
    shell: Res<Shell>,
    mut focus: ResMut<InputFocus>,
    mut focus_visible: ResMut<InputFocusVisible>,
    root: Single<Entity, With<ShellRoot>>,
    children: Query<&Children>,
    parts: Query<(&ComputedNode, &UiGlobalTransform), With<Part>>,
    pages: Query<(Entity, &Page)>,
    places: Query<(Entity, &PlaceButton)>,
) {
    let Some(&(_, way)) = ARROWS.iter().find(|(key, _)| keys.just_pressed(*key)) else {
        return;
    };
    // Only what is laid out: a control of a page that is not showing has
    // no size.
    let area = |entity: Entity| {
        let (node, transform) = parts.get(entity).ok()?;
        if node.is_empty() {
            return None;
        }
        let scale = node.inverse_scale_factor();
        let (_, _, at) = transform.to_scale_angle_translation();
        Some(FocusableArea {
            entity,
            position: at * scale,
            size: node.size() * scale,
        })
    };
    let all: Vec<FocusableArea> = children.iter_descendants(*root).filter_map(area).collect();
    let from = focus
        .get()
        .filter(|from| all.iter().any(|part| part.entity == *from));
    let next = match from {
        Some(from) => towards(from, way, &all),
        None => pages
            .iter()
            .find(|(_, page)| match **page {
                Page::Tab(place, tab) => place == shell.place() && tab == shell.tab(place),
                _ => false,
            })
            .and_then(|(page, _)| {
                children
                    .iter_descendants_depth_first(page)
                    .find(|entity| area(*entity).is_some())
            })
            .or_else(|| {
                places
                    .iter()
                    .find(|(_, place)| place.0 == shell.place())
                    .map(|(entity, _)| entity)
            }),
    };
    if let Some(next) = next {
        focus.set(next, FocusCause::Navigated);
    }
    if from.is_some() || next.is_some() {
        focus_visible.0 = true;
    }
}

fn close_detail(shell: &mut Shell, focus: &mut InputFocus) {
    if let Some(opener) = shell.close_detail() {
        focus.set(opener, FocusCause::Navigated);
    }
}

/// Escape goes back one level: it puts a hint away where Got it would
/// (never a town lesson, #372); otherwise it closes the side panel, and on
/// the road, with no side panel open, the place.
fn escape(
    keys: Res<ButtonInput<KeyCode>>,
    game: Res<Game>,
    mut shell: ResMut<Shell>,
    mut focus: ResMut<InputFocus>,
) {
    if !keys.just_pressed(KeyCode::Escape) {
        return;
    }
    if teach::got_it(&game, &shell) {
        shell.dismiss_hint();
        return;
    }
    if !shell.is_open() {
        return;
    }
    if shell.detail(shell.place()).is_some() {
        close_detail(&mut shell, &mut focus);
    } else if !game.in_town() {
        shell.close();
    }
}

/// Setting out puts the place away, so the road starts with the scene
/// expanded; arriving opens it again, as it was left. Town is where the
/// player decides. A detail left open over a cargo column is put away on
/// arriving, since it stands where the cargo is and the party has come
/// back with something to look at there (#287).
fn follow_the_party(
    game: Res<Game>,
    map: Res<Places>,
    mut shell: ResMut<Shell>,
    mut was_in_town: Local<Option<bool>>,
) {
    // The first frame counts as a move, so a game that starts on the road
    // (#292) starts with every place closed.
    let in_town = game.in_town();
    if Some(in_town) != *was_in_town {
        *was_in_town = Some(in_town);
        if in_town {
            for (p, place) in map.pages().enumerate() {
                if place.horse_columns {
                    shell.drop_detail(p);
                }
            }
            let place = shell.place();
            shell.open(place);
        } else {
            shell.close();
        }
    }
    // A card open on a carried piece goes when there is no piece at its
    // place in the cargo any more.
    shell.forget_carried_from(game.cargo().len());
}

/// The pointer resting on a piece shows its card (#228). The first card
/// waits for the pointer to rest; with one up, the next piece's shows at
/// once, so that going down a column reads each in turn.
fn point(
    time: Res<Time<Real>>,
    tokens: Res<Tokens>,
    game: Res<Game>,
    mut shell: ResMut<Shell>,
    mut rested: Local<f32>,
    cells: Query<
        (
            Entity,
            &Hovered,
            &ComputedNode,
            Option<&SlotRow>,
            Option<&CarriedCell>,
        ),
        Or<(With<SlotRow>, With<CarriedCell>, With<ReportedCell>)>,
    >,
) {
    // While a piece is dragged its card is the preview of its drop, beside
    // the horse or slot it would go to, and nothing else is shown (#228).
    if let Some(drag) = shell.dragging() {
        *rested = 0.0;
        let wanted = drag.onto.map(|onto| (onto.cell, drag.piece));
        if shell.pointed() != wanted {
            shell.point(wanted);
        }
        return;
    }
    let under = cells.iter().find_map(|(entity, hovered, node, row, carried)| {
        // A cell of a page that is not showing has no size.
        if !hovered.get() || node.is_empty() {
            return None;
        }
        let piece = match (row, carried) {
            (Some(row), _) => Piece::Worn {
                horse: row.horse,
                slot: row.slot,
            },
            (None, Some(cell)) => Piece::Carried(cell.at),
            (None, None) => game.reported_piece()?,
        };
        shell.points_at(&game, piece).map(|piece| (entity, piece))
    });
    if under.is_none() || shell.pointed().is_some() {
        *rested = 0.0;
    } else {
        *rested += time.delta_secs();
    }
    let wanted = under.filter(|_| shell.pointed().is_some() || *rested >= tokens.time.hover_card);
    if shell.pointed() != wanted {
        shell.point(wanted);
    }
}

/// A drag begins on a piece (#228): a slot that holds one, a carried piece,
/// or the report's drop. In town only, where a piece can be put on. The
/// event comes up from what the pointer is on, through the cell it is in.
fn on_drag_start(
    drag: On<PointerDragStart>,
    game: Res<Game>,
    mut shell: ResMut<Shell>,
    cells: Query<
        (Option<&SlotRow>, Option<&CarriedCell>),
        Or<(With<SlotRow>, With<CarriedCell>, With<ReportedCell>)>,
    >,
) {
    if drag.button != PointerButton::Primary || shell.dragging().is_some() || !game.in_town() {
        return;
    }
    let Ok((row, carried)) = cells.get(drag.entity) else {
        return;
    };
    let piece = match (row, carried) {
        (Some(row), _) => Some(Piece::Worn {
            horse: row.horse,
            slot: row.slot,
        }),
        (None, Some(cell)) => Some(Piece::Carried(cell.at)),
        (None, None) => game.reported_piece(),
    };
    if let Some(piece) = piece.filter(|piece| game.piece(*piece).is_some()) {
        shell.take(piece, drag.entity);
    }
}

/// The pointer lets a dragged piece go (#228). Over a horse or slot it may
/// go to, that is Equip for that horse, and a card open on the piece goes
/// with it; anywhere else nothing changes. The piece in the hand goes into
/// the cell it went to, or back into its own (#363).
fn on_drag_end(
    _: On<PointerDragEnd>,
    mut game: ResMut<Game>,
    mut shell: ResMut<Shell>,
    mut held: ResMut<Held>,
) {
    let Some(drag) = shell.let_go() else {
        return;
    };
    let moved = drag
        .onto
        .and_then(|onto| Some((onto.cell, game.drop_piece(drag.piece, onto.horse, onto.slot)?)));
    match moved {
        Some((cell, (from, to))) => {
            shell.piece_moved(from, to);
            held.land(cell, to);
        }
        None => held.land(drag.from, drag.piece),
    }
}

/// While a piece is dragged, the horse or slot under the pointer that it may
/// be let go on (#228): a slot of its type first, then a horse in the scene
/// (#356), or its tag (#368), when the pointer is on the scene and not on a
/// menu. The horse's tag stands for it, so the preview goes beside the tag.
fn hold_over(
    game: Res<Game>,
    mut shell: ResMut<Shell>,
    slots: Query<(Entity, &Hovered, &ComputedNode, &SlotRow)>,
    scene: (
        Single<&Hovered, With<SceneCatcher>>,
        Res<HorseBox>,
        Single<(&Camera, &GlobalTransform), With<Camera3d>>,
        Query<(&PointerId, &PointerLocation)>,
    ),
    tags: Tags,
) {
    let Some(drag) = shell.dragging() else {
        return;
    };
    let (on_scene, horses, camera, pointers) = scene;
    let may = |horse: usize, slot: Option<usize>| game.drop_hand(drag.piece, horse, slot).is_some();
    let onto = slots
        .iter()
        .find(|(_, hovered, node, row)| hovered.get() && !node.is_empty() && may(row.horse, Some(row.slot)))
        .map(|(cell, _, _, row)| Onto {
            cell,
            horse: row.horse,
            slot: Some(row.slot),
        })
        .or_else(|| {
            if !on_scene.get() {
                return None;
            }
            let (camera, at) = *camera;
            let position = pointers
                .iter()
                .find(|(id, _)| **id == PointerId::Mouse)
                .and_then(|(_, location)| location.location())?
                .position;
            // A horse's tag stands for it here as well (#368).
            let horse = tag_under(&tags, position)
                .or_else(|| horses.horse_on(camera.viewport_to_world(at, position).ok()?))?;
            let (cell, ..) = tags.iter().find(|(_, tag, ..)| tag.0 == horse)?;
            may(horse, None).then_some(Onto {
                cell,
                horse,
                slot: None,
            })
        });
    if drag.onto != onto {
        shell.hold_over(onto);
    }
}

/// While a piece is dragged, every horse and slot it may be let go on is
/// marked as a target (#228), and nothing is once it is let go.
fn mark_targets(
    mut commands: Commands,
    game: Res<Game>,
    shell: Res<Shell>,
    slots: Query<(Entity, &SlotRow, Has<Target>)>,
    horses: Query<(Entity, &HorseTag, Has<Target>)>,
) {
    let may = |horse: usize, slot: Option<usize>| {
        shell
            .dragging()
            .is_some_and(|drag| game.drop_hand(drag.piece, horse, slot).is_some())
    };
    let marks = slots
        .iter()
        .map(|(entity, row, has)| (entity, may(row.horse, Some(row.slot)), has))
        .chain(horses.iter().map(|(entity, horse, has)| (entity, may(horse.0, None), has)));
    for (entity, wanted, has) in marks {
        if wanted && !has {
            commands.entity(entity).insert(Target);
        } else if !wanted && has {
            commands.entity(entity).remove::<Target>();
        }
    }
}

/// Where the hover card goes, as the edges of its node: beside the cell it
/// is about, on the side with more room, and from the cell's top or bottom
/// edge towards the middle of the window, so it is never over its piece. A
/// cell too wide for the card on either side (a slot of the Gear tab, #228)
/// has it inside the window, over the cell's far end.
fn beside(cell: Rect, window: Vec2, gap: f32, width: f32) -> UiRect {
    let (left, right) = if window.x - cell.max.x >= cell.min.x {
        let left = (cell.max.x + gap).min(window.x - width).max(0.0);
        (Val::Px(left), Val::Auto)
    } else {
        let right = (window.x - cell.min.x + gap).min(window.x - width).max(0.0);
        (Val::Auto, Val::Px(right))
    };
    let (top, bottom) = if cell.center().y < window.y / 2.0 {
        (Val::Px(cell.min.y), Val::Auto)
    } else {
        (Val::Auto, Val::Px(window.y - cell.max.y))
    };
    UiRect {
        left,
        right,
        top,
        bottom,
    }
}

fn show_hover_card(
    tokens: Res<Tokens>,
    shell: Res<Shell>,
    window: Single<&Window, With<PrimaryWindow>>,
    cells: Query<(&ComputedNode, &UiGlobalTransform)>,
    card: Single<(&mut Node, &mut Visibility), With<HoverCard>>,
) {
    let (mut node, mut visibility) = card.into_inner();
    let at = shell
        .offered()
        .map(|(cell, _)| cell)
        .or(shell.pointed().map(|(cell, _)| cell))
        .and_then(|cell| cells.get(cell).ok())
        .map(|(cell, transform)| {
            let scale = cell.inverse_scale_factor();
            let (_, _, middle) = transform.to_scale_angle_translation();
            let cell = Rect::from_center_size(middle * scale, cell.size() * scale);
            beside(cell, window.size(), tokens.space.s, tokens.size.compare_width)
        });
    let (display, wanted) = match at {
        Some(_) => (Display::Flex, Visibility::Inherited),
        None => (Display::None, Visibility::Hidden),
    };
    if node.display != display {
        node.display = display;
    }
    if *visibility != wanted {
        *visibility = wanted;
    }
    if let Some(at) = at
        && (node.left, node.right, node.top, node.bottom) != (at.left, at.right, at.top, at.bottom)
    {
        (node.left, node.right, node.top, node.bottom) = (at.left, at.right, at.top, at.bottom);
    }
}

/// Keep each horse's tag under its feet as the scene moves (#356).
fn place_tags(
    tokens: Res<Tokens>,
    camera: Single<(&Camera, &GlobalTransform), With<Camera3d>>,
    mut spots: Query<(&TagSpot, &mut Node, &mut Visibility)>,
) {
    let (camera, at) = *camera;
    for (spot, mut node, mut visibility) in &mut spots {
        let wanted = match camera.world_to_viewport(at, horse_feet(spot.0)) {
            Ok(feet) => {
                let (left, top) = (Val::Px(feet.x), Val::Px(feet.y + tokens.space.s));
                if (node.left, node.top) != (left, top) {
                    (node.left, node.top) = (left, top);
                }
                Visibility::Inherited
            }
            Err(_) => Visibility::Hidden,
        };
        if *visibility != wanted {
            *visibility = wanted;
        }
    }
}

/// Keep the boss's box over the boss while its fight is on (#360): the
/// square the body is drawn in, as the camera sees it. Gone otherwise.
fn place_boss(
    game: Res<Game>,
    announce: Res<crate::fx::boss::Announce>,
    camera: Single<(&Camera, &GlobalTransform), With<Camera3d>>,
    spot: Single<(&mut Node, &mut Visibility), With<BossSpot>>,
) {
    let (camera, at) = *camera;
    let (mut node, mut visibility) = spot.into_inner();
    // Where it is drawn while it comes up the trail, too (#19).
    let away = Vec3::X * announce.approach(game.approach_left()).0;
    let drawn = game.boss().and_then(|(slot, foe)| {
        let (middle, half) = (enemy_centre(slot, foe.max_health) + away, enemy_size(foe.max_health) / 2.0);
        let corners = [Vec3::new(-half, half, 0.0), Vec3::new(half, -half, 0.0)];
        let ends: Vec<Vec2> = corners
            .iter()
            .map(|corner| camera.world_to_viewport(at, middle + *corner).ok())
            .collect::<Option<_>>()?;
        Some(Rect::from_corners(ends[0], ends[1]))
    });
    let (display, wanted) = match drawn {
        Some(rect) => {
            let place = (
                Val::Px(rect.min.x),
                Val::Px(rect.min.y),
                Val::Px(rect.width()),
                Val::Px(rect.height()),
            );
            if (node.left, node.top, node.width, node.height) != place {
                (node.left, node.top, node.width, node.height) = place;
            }
            (Display::Flex, Visibility::Inherited)
        }
        None => (Display::None, Visibility::Hidden),
    };
    if node.display != display {
        node.display = display;
    }
    if *visibility != wanted {
        *visibility = wanted;
    }
}

/// Move the scene towards the layout the shell asks for, and the shell's
/// top edge with it.
fn move_scene(
    time: Res<Time<Real>>,
    tokens: Res<Tokens>,
    shell: Res<Shell>,
    mut view: ResMut<SceneView>,
    mut root: Single<&mut Node, With<ShellRoot>>,
) {
    let wanted = if shell.is_open() { 0.0 } else { 1.0 };
    if view.progress != wanted {
        view.step_towards(!shell.is_open(), time.delta_secs(), tokens.time.layout);
    }
    let top = Val::Percent(tokens.layout.share(view.expanded()) * 100.0);
    if root.top != top {
        root.top = top;
    }
}

fn show_place(
    shell: Res<Shell>,
    view: Res<SceneView>,
    map: Res<Places>,
    tokens: Res<Tokens>,
    mut pages: Query<(&Page, &mut Node, &mut Visibility), Without<ShellRoot>>,
) {
    // A place that is being put away stays in view until it is covered.
    let place_in_view = shell.is_open() || view.progress < 1.0;
    // The Inventory has the place's whole width but for the comparison.
    let inventory = |place: usize| map.content(place, shell.tab(place)) == Content::Inventory;
    for (page, mut node, mut visibility) in &mut pages {
        let shown = match *page {
            Page::Place(place) => place == shell.place() && place_in_view,
            Page::Tab(place, tab) => tab == shell.tab(place),
            Page::Detail(place) => shell.detail(place).is_some(),
            Page::Cargo(place) => shell.detail(place).is_none(),
            Page::Compare(place) => shell.detail(place).is_none() && inventory(place),
            Page::Card(place) => shell
                .detail(place)
                .is_some_and(|detail| detail.item.piece().is_some()),
            Page::HorseParts(place) => shell
                .detail(place)
                .is_some_and(|detail| matches!(detail.item, Item::Horse(_))),
            Page::RoadView => !place_in_view,
            Page::Hint => shell.hint().is_some(),
        };
        let wanted = if shown { Display::Flex } else { Display::None };
        if node.display != wanted {
            node.display = wanted;
        }
        // A pinned piece has the comparison's width, wherever it is pinned
        // (#312), and the Inventory keeps that room for it.
        if let Page::Detail(place) = *page {
            let columns = map.pages().nth(place).is_some_and(|page| page.horse_columns);
            let piece = shell
                .detail(place)
                .is_some_and(|detail| detail.item.piece().is_some());
            let width = Val::Px(match (inventory(place) || piece, columns) {
                (true, _) => tokens.size.compare_width,
                (false, true) => tokens.size.cargo_width,
                (false, false) => tokens.size.side_panel_width,
            });
            if node.width != width {
                node.width = width;
            }
        }
        // Hidden as well as out of the layout: Tab passes over what is
        // hidden, and stops at what is only out of the layout (#244).
        let wanted = if shown {
            Visibility::Inherited
        } else {
            Visibility::Hidden
        };
        if *visibility != wanted {
            *visibility = wanted;
        }
    }
}

/// Mark what is chosen, and what cannot be used right now.
fn mark_chosen(
    mut commands: Commands,
    shell: Res<Shell>,
    game: Res<Game>,
    controls: Query<
        (
            Entity,
            Option<&PlaceButton>,
            Option<&TabButton>,
            Option<&HorseTag>,
            Option<&SlotRow>,
            Option<&CarriedCell>,
            Option<&ReportedCell>,
            Has<Selected>,
        ),
        Or<(
            With<PlaceButton>,
            With<TabButton>,
            With<HorseTag>,
            With<SlotRow>,
            With<CarriedCell>,
            With<ReportedCell>,
        )>,
    >,
    map: Res<Places>,
    planned: Query<(
        Entity,
        &ControlButton,
        Has<Selected>,
        Has<InteractionDisabled>,
    )>,
) {
    for (entity, place, tab, horse, slot, carried, reported, selected) in &controls {
        let wanted = place.is_some_and(|place| place.0 == shell.place() && shell.is_open())
            || tab.is_some_and(|tab| tab.tab == shell.tab(tab.place))
            || horse.is_some_and(|horse| shell.marked() == Some(horse.0))
            || slot.is_some_and(|row| {
                shell.detail(row.place).is_some_and(|detail| {
                    detail.tab == row.tab
                        && detail.item
                            == Item::Worn {
                                horse: row.horse,
                                slot: row.slot,
                            }
                })
            })
            || carried.is_some_and(|cell| {
                shell.detail(cell.place).is_some_and(|detail| {
                    detail.item == Item::Carried(cell.at) && cell.tab.is_none_or(|tab| tab == detail.tab)
                })
            })
            || reported.is_some_and(|cell| {
                game.reported_piece().is_some_and(|piece| {
                    shell.detail(cell.place).is_some_and(|detail| {
                        detail.tab == cell.tab && detail.item == Item::from(piece)
                    })
                })
            });
        if wanted && !selected {
            commands.entity(entity).insert(Selected);
        } else if !wanted && selected {
            commands.entity(entity).remove::<Selected>();
        }
    }

    // A control that acts is unavailable while its action cannot be done.
    // A switch is chosen while it is on, Keep going too. Any other is
    // chosen while the panel it opened is showing.
    for (entity, button, selected, disabled) in &planned {
        let control = map.control(button.id);
        let (chosen, blocked) = match (control.action, control.switch) {
            (Some(action), _) => (
                action_chosen(&game, &shell, action)
                    || (action == Action::KeepGoing && game.keep_going()),
                game.can(action, shell.hand()).is_err(),
            ),
            (None, Some(switch)) => (shell.is_on(switch), false),
            (None, None) => (
                shell
                    .detail(button.place.unwrap_or(shell.place()))
                    .is_some_and(|detail| detail.item == Item::Control(button.id)),
                false,
            ),
        };
        if chosen && !selected {
            commands.entity(entity).insert(Selected);
        } else if !chosen && selected {
            commands.entity(entity).remove::<Selected>();
        }
        if blocked && !disabled {
            commands.entity(entity).insert(InteractionDisabled);
        } else if !blocked && disabled {
            commands.entity(entity).remove::<InteractionDisabled>();
        }
    }
}

/// Put the hint's mark on the control the hint is about (#291), wherever
/// that control is drawn, and take it off when the hint goes. A tab's mark
/// is on its place too, while the player is not at that place (#350).
fn point_out(
    mut commands: Commands,
    shell: Res<Shell>,
    controls: Query<
        (
            Entity,
            Option<&PlaceButton>,
            Option<&TabButton>,
            Option<&HorseTag>,
            Option<&ControlButton>,
            Option<&CarriedCell>,
            Option<&SlotRow>,
            Has<BossSpot>,
            Has<Pointed>,
        ),
        Or<(
            With<PlaceButton>,
            With<TabButton>,
            With<HorseTag>,
            With<ControlButton>,
            With<CarriedCell>,
            With<SlotRow>,
            With<BossSpot>,
        )>,
    >,
) {
    let about = shell.hint().map(|hint| hint.about);
    for (entity, place, tab, horse, control, carried, slot, boss, pointed) in &controls {
        let wanted = match about {
            Some(About::Place(p)) => place.is_some_and(|button| button.0 == p),
            Some(About::Tab(p, t)) => {
                tab.is_some_and(|button| button.place == p && button.tab == t)
                    || place.is_some_and(|button| button.0 == p && !shell.at(About::Place(p)))
            }
            Some(About::Horse(h)) => horse.is_some_and(|button| button.0 == h),
            Some(About::Control(id)) => control.is_some_and(|button| button.id == id),
            // A piece is marked wherever it is listed: the cargo, the
            // report, the Gear tab and the Inventory.
            Some(About::Piece(Piece::Carried(at))) => carried.is_some_and(|cell| cell.at == at),
            Some(About::Piece(Piece::Worn { horse, slot: s })) => {
                slot.is_some_and(|row| row.horse == horse && row.slot == s)
            }
            // Wherever the boss stands: `place_boss` keeps its box on it.
            Some(About::Boss(_)) => boss,
            Some(About::Party) | None => false,
        };
        if wanted && !pointed {
            commands.entity(entity).insert(Pointed);
        } else if !wanted && pointed {
            commands.entity(entity).remove::<Pointed>();
        }
    }
}

/// What the control a hint is about is called, and the key printed on it.
fn about_name(map: &Places, game: &Game, about: About) -> (String, Option<String>) {
    let key = |key: Option<&str>| key.map(String::from);
    match about {
        About::Place(p) => map
            .pages()
            .nth(p)
            .map(|page| (page.name.clone(), key(page.key.as_deref()))),
        About::Tab(p, t) => map
            .pages()
            .nth(p)
            .and_then(|page| page.tabs.get(t))
            .map(|tab| (tab.name.clone(), key(tab.key.as_deref()))),
        About::Horse(h) => game
            .party
            .get(h)
            .map(|horse| (horse.name.clone(), key(map.horse_key(h)))),
        About::Control(id) => map
            .list
            .get(id)
            .map(|control| (control.name.clone(), key(control.key.as_deref()))),
        About::Piece(piece) => game.piece(piece).map(|gear| (gear.name.clone(), None)),
        About::Boss(_) => game.boss().map(|(_, foe)| (foe.name.clone(), None)),
        About::Party => None,
    }
    .unwrap_or_default()
}

/// The hint's line says what the hint showing says: its control's name,
/// that control's key, and its words. The answer to Skip all hints has
/// Got it beside it and no Skip all hints (#350). In town a lesson, and
/// that answer, have no Got it: they go when their thing is done (#372).
fn show_hint(
    shell: Res<Shell>,
    game: Res<Game>,
    map: Res<Places>,
    line: Single<Entity, With<HintLine>>,
    children: Query<&Children>,
    mut texts: Query<(&HintText, &mut Text)>,
    mut skips: Query<(&mut Node, &mut Visibility), (With<SkipHints>, Without<DismissHint>)>,
    mut got_its: Query<(&mut Node, &mut Visibility), With<DismissHint>>,
) {
    let shown = |on: bool| match on {
        true => (Display::Flex, Visibility::Inherited),
        false => (Display::None, Visibility::Hidden),
    };
    let got_it = shell.hint().is_none() || teach::got_it(&game, &shell);
    let buttons = skips
        .iter_mut()
        .map(|button| (button, shown(!shell.parting())))
        .chain(got_its.iter_mut().map(|button| (button, shown(got_it))));
    for ((mut node, mut visibility), (display, seen)) in buttons {
        if node.display != display {
            node.display = display;
        }
        if *visibility != seen {
            *visibility = seen;
        }
    }
    let (about, key, words) = match shell.hint() {
        Some(hint) => {
            let (about, key) = about_name(&map, &game, hint.about);
            let key = key.map(|key| format!("[{key}]")).unwrap_or_default();
            (about, key, hint.words.clone())
        }
        None => Default::default(),
    };
    for child in children.iter_descendants(*line) {
        if let Ok((which, mut text)) = texts.get_mut(child) {
            let wanted = match which {
                HintText::About => &about,
                HintText::Key => &key,
                HintText::Words => &words,
            };
            if text.0 != *wanted {
                text.0 = wanted.clone();
            }
        }
    }
}

/// Which side of what it is about a pop-up stands on.
#[derive(Clone, Copy, PartialEq)]
enum Side {
    Below,
    Above,
    Left,
    Right,
}

/// Put the hint's pop-up beside the control it is about (#353): under it
/// when there is room and over it when not, so it never covers it, centred
/// on it as far as the window allows, with its point toward it. A control
/// on the item card has its pop-up beside the card instead, on the side
/// with room, so the card it is about stays in view (owner, 2026-10-17). A
/// hint on no control (the party) is over the middle of the scene, with no
/// point. When a hint comes up its pop-up rises into place, which the eye
/// catches.
fn place_popup(
    time: Res<Time<Real>>,
    tokens: Res<Tokens>,
    shell: Res<Shell>,
    window: Single<&Window, With<PrimaryWindow>>,
    targets: Query<(Entity, &ComputedNode, &UiGlobalTransform, &InheritedVisibility), With<Pointed>>,
    pages: Query<(&Page, &ComputedNode, &UiGlobalTransform, &InheritedVisibility)>,
    popup: Single<(&mut Node, &ComputedNode, &mut UiTransform), With<HintLine>>,
    arrow: Single<(&mut Node, &mut Visibility), (With<PopupArrow>, Without<HintLine>)>,
    mut rising: Local<Rising>,
) {
    let Some(hint) = shell.hint() else {
        rising.hint = None;
        return;
    };
    if rising.hint.as_ref() == Some(hint) {
        rising.secs += time.delta_secs();
    } else {
        *rising = Rising { hint: Some(hint.clone()), secs: 0.0 };
    }
    let t = &*tokens;
    let window = window.size();
    let (mut node, computed, mut transform) = popup.into_inner();
    let (mut arrow, mut arrow_seen) = arrow.into_inner();
    let height = match computed.size().y * computed.inverse_scale_factor() {
        drawn if drawn > 0.0 => drawn,
        // Not drawn yet: about what two lines and the controls take.
        _ => 4.0 * t.size.control_height,
    };
    let (width, margin) = (t.size.popup_width, t.space.l);
    // The control where it is drawn now: the first of its marks in view.
    let target = targets
        .iter()
        .filter(|(_, drawn, _, seen)| seen.get() && drawn.size().x > 0.0)
        .min_by_key(|(entity, ..)| *entity)
        .map(|(_, drawn, at, _)| {
            let scale = drawn.inverse_scale_factor();
            let (_, _, middle) = at.to_scale_angle_translation();
            Rect::from_center_size(middle * scale, drawn.size() * scale)
        });
    let reach = t.size.popup_gap + t.size.popup_arrow / 2.0;
    // The item card the control is on, if it is on one.
    let card = target.and_then(|target| {
        pages
            .iter()
            .filter(|(page, drawn, _, seen)| matches!(page, Page::Detail(_)) && seen.get() && drawn.size().x > 0.0)
            .map(|(_, drawn, at, _)| {
                let scale = drawn.inverse_scale_factor();
                let (_, _, middle) = at.to_scale_angle_translation();
                Rect::from_center_size(middle * scale, drawn.size() * scale)
            })
            .find(|card| card.contains(target.center()))
    });
    let (left, top, bottom, point) = match (target, card) {
        (Some(target), Some(card)) => {
            // Beside the card, level with the control, pointing across at it.
            let before = card.min.x - reach - width >= margin || card.max.x + reach + width > window.x - margin;
            let left = match before {
                true => (card.min.x - reach - width).max(margin),
                false => card.max.x + reach,
            };
            let top = (target.center().y - height / 2.0).clamp(margin, (window.y - height - margin).max(margin));
            let side = if before { Side::Left } else { Side::Right };
            (left, Val::Px(top), Val::Auto, Some((side, target.center().y - top)))
        }
        (Some(target), None) => {
            let below = target.max.y + reach + height <= window.y - margin || target.min.y - reach - height < margin;
            let left = (target.center().x - width / 2.0).clamp(margin, (window.x - width - margin).max(margin));
            match below {
                true => (left, Val::Px(target.max.y + reach), Val::Auto, Some((Side::Below, target.center().x - left))),
                false => (left, Val::Auto, Val::Px(window.y - target.min.y + reach), Some((Side::Above, target.center().x - left))),
            }
        }
        (None, _) => ((window.x - width) / 2.0, Val::Percent(20.0), Val::Auto, None),
    };
    if (node.left, node.top, node.bottom) != (Val::Px(left), top, bottom) {
        (node.left, node.top, node.bottom) = (Val::Px(left), top, bottom);
    }

    let seen = match point {
        Some(_) => Visibility::Inherited,
        None => Visibility::Hidden,
    };
    if *arrow_seen != seen {
        *arrow_seen = seen;
    }
    if let Some((side, along)) = point {
        // Half out of the edge nearest the control, at the control.
        let edge = Val::Px(-t.size.popup_arrow / 2.0 - t.size.popup_border);
        let inside = t.size.radius + t.space.s;
        let length = match side {
            Side::Below | Side::Above => width,
            Side::Left | Side::Right => height,
        };
        let most = (length - t.size.popup_arrow - 2.0 * t.size.popup_border - inside).max(inside);
        let at = Val::Px((along - t.size.popup_arrow / 2.0 - t.size.popup_border).clamp(inside, most));
        let wanted = match side {
            Side::Below => (edge, Val::Auto, at, Val::Auto),
            Side::Above => (Val::Auto, edge, at, Val::Auto),
            Side::Left => (at, Val::Auto, Val::Auto, edge),
            Side::Right => (at, Val::Auto, edge, Val::Auto),
        };
        if (arrow.top, arrow.bottom, arrow.left, arrow.right) != wanted {
            (arrow.top, arrow.bottom, arrow.left, arrow.right) = wanted;
        }
    }

    // It rises toward its control, and grows to its size as it does.
    let done = match t.time.popup > 0.0 {
        true => (rising.secs / t.time.popup).min(1.0),
        false => 1.0,
    };
    let eased = 1.0 - (1.0 - done).powi(3);
    let rise = t.size.popup_rise * (1.0 - eased);
    let (x, y) = match point {
        Some((Side::Above, _)) => (0.0, -rise),
        Some((Side::Left, _)) => (-rise, 0.0),
        Some((Side::Right, _)) => (rise, 0.0),
        _ => (0.0, rise),
    };
    let wanted = UiTransform {
        translation: Val2::px(x, y),
        scale: Vec2::splat(0.92 + 0.08 * eased),
        ..default()
    };
    if *transform != wanted {
        *transform = wanted;
    }
}

/// How many sample hints there are: one for each kind of control a hint
/// can be about.
const SAMPLE_HINTS: usize = 4;

/// What a sample hint has for its lesson. No lesson has this name.
pub const SAMPLE: &str = "sample";

/// What the answer to Skip all hints has for its lesson (#350). No lesson
/// has this name.
pub const PARTING: &str = "where the hints went";

/// A hint to look at the hint's line with, before any lesson raises one
/// (the lessons are #290). The first is about the Journal, which is in the
/// bar in both layouts; then Set out in the footer, the first horse in the
/// strip, and the first tab of the first place.
pub(super) fn sample_hint(map: &Places, which: usize) -> Hint {
    let journal = map.places.iter().position(|place| place.name == "Journal");
    let set_out = map.list.iter().find(|control| control.action == Some(Action::SetOut));
    let (about, words) = match (which % SAMPLE_HINTS, set_out) {
        (1, Some(control)) => (About::Control(control.id), "The party leaves town when this is pressed."),
        (2, _) => (About::Horse(0), "A press opens everything about this horse."),
        (3, _) => (About::Tab(0, 0), "What the selected horse wears is here."),
        _ => (About::Place(journal.unwrap_or(0)), "The report of every trail is kept here."),
    };
    Hint {
        lesson: SAMPLE.into(),
        about,
        words: format!("A sample hint. {words}"),
    }
}

/// Dev aid: F6 shows a sample hint, whatever was put away or skipped
/// before or is showing, and takes it away again. Each time it is about the next kind of
/// control.
fn dev_hint(
    keys: Res<ButtonInput<KeyCode>>,
    map: Res<Places>,
    mut shell: ResMut<Shell>,
    mut shown: Local<usize>,
) {
    if !keys.just_pressed(KeyCode::F6) {
        return;
    }
    let sample = shell.hint().is_some_and(|hint| hint.lesson == SAMPLE);
    // A lesson's hint gives way to the sample, and comes back after it.
    shell.finish_hint();
    if !sample {
        shell.forget_hints();
        shell.raise_hint(sample_hint(&map, *shown));
        *shown += 1;
    }
}

/// Which of the things a mark is raised for are in view: the report the
/// Journal is open at, and the piece in the cargo. The report is, while its
/// tab shows. The piece is, while the cargo column shows, or its own card
/// does, or the report that shows it does.
fn in_view(shell: &Shell, map: &Places, game: &Game) -> (bool, bool) {
    let place = shell.place();
    let Some(page) = map.pages().nth(place).filter(|_| shell.is_open()) else {
        return (false, false);
    };
    let report = page
        .tabs
        .get(shell.tab(place))
        .is_some_and(|tab| tab.content == Content::Report);
    let carried = match shell.detail(place) {
        Some(detail) => matches!(detail.item, Item::Carried(_)),
        // With no detail over it, the cargo is beside every tab; and the
        // Inventory shows every piece.
        None => page.horse_columns || page.content == Content::Inventory,
    };
    let reported = report && matches!(game.reported_piece(), Some(Piece::Carried(_)));
    (report, carried || reported)
}

/// What is in view has been seen, and raises no mark again (#232).
fn look(shell: &Shell, map: &Places, game: &mut Game) {
    let (report, carried) = in_view(shell, map, game);
    if report {
        game.see_report();
    }
    if carried {
        game.see_drop();
    }
}

fn see(shell: Res<Shell>, map: Res<Places>, mut game: ResMut<Game>) {
    look(&shell, &map, &mut game);
}

/// Put the attention mark on every place and tab that has something the
/// player has not seen (#232).
fn mark_unseen(
    mut commands: Commands,
    game: Res<Game>,
    map: Res<Places>,
    parts: Query<
        (Entity, Option<&PlaceButton>, Option<&TabButton>, Has<Unseen>),
        Or<(With<PlaceButton>, With<TabButton>)>,
    >,
) {
    for (entity, place, tab, marked) in &parts {
        let wanted = match (place, tab) {
            (Some(place), _) => map.wants(&game, place.0, None),
            (_, Some(tab)) => map.wants(&game, tab.place, Some(tab.tab)),
            _ => false,
        };
        if wanted && !marked {
            commands.entity(entity).insert(Unseen);
        } else if !wanted && marked {
            commands.entity(entity).remove::<Unseen>();
        }
    }
}

/// Out of the layout and hidden, or in it: hidden too, so that Tab passes
/// over it (#244).
fn set_shown(node: &mut Node, visibility: &mut Visibility, shown: bool) {
    let display = if shown { Display::Flex } else { Display::None };
    if node.display != display {
        node.display = display;
    }
    let wanted = if shown {
        Visibility::Inherited
    } else {
        Visibility::Hidden
    };
    if *visibility != wanted {
        *visibility = wanted;
    }
}

/// Equip names the selected horse, and the list under Another horse
/// (#356) is in view while it is open: in it, every horse but the selected
/// one, in the line's order. One the piece may not go to is unavailable,
/// and the horse that wears it says so.
fn show_equip(
    mut commands: Commands,
    game: Res<Game>,
    shell: Res<Shell>,
    map: Res<Places>,
    equips: Query<(&ControlButton, &Children)>,
    mut lists: Query<(&mut Node, &mut Visibility), (With<OtherHorses>, Without<EquipOn>)>,
    mut entries: Query<
        (Entity, &EquipOn, &Children, &mut Node, &mut Visibility, Has<InteractionDisabled>),
        Without<OtherHorses>,
    >,
    mut texts: Query<(&PartText, &mut Text)>,
) {
    let mut label = |children: &Children, wanted: &str| {
        for child in children {
            if let Ok((PartText::Main, mut text)) = texts.get_mut(*child)
                && text.0 != wanted
            {
                text.0 = wanted.to_string();
            }
        }
    };
    let Some(selected) = game.party.get(shell.horse()) else {
        return;
    };
    let equip = format!("Equip on {}", selected.name);
    for (button, children) in &equips {
        if map.control(button.id).action == Some(Action::Equip) {
            label(children, &equip);
        }
    }
    let open = shell.others_open();
    for (mut node, mut visibility) in &mut lists {
        set_shown(&mut node, &mut visibility, open);
    }
    let piece = shell.hand().open;
    for (entity, entry, children, mut node, mut visibility, disabled) in &mut entries {
        let Some(horse) = game.party.get(entry.horse) else {
            set_shown(&mut node, &mut visibility, false);
            continue;
        };
        set_shown(&mut node, &mut visibility, entry.horse != shell.horse());
        let wears = matches!(piece, Some(Piece::Worn { horse, .. }) if horse == entry.horse);
        label(
            children,
            &if wears {
                format!("{} wears it", horse.name)
            } else {
                horse.name.clone()
            },
        );
        let blocked = piece.and_then(|piece| game.drop_hand(piece, entry.horse, None)).is_none();
        if blocked && !disabled {
            commands.entity(entity).insert(InteractionDisabled);
        } else if !blocked && disabled {
            commands.entity(entity).remove::<InteractionDisabled>();
        }
    }
}

/// The horse of the list beside Equip that the pointer is on, or else the
/// focus: what equipping there would do shows beside it (#356).
fn offer(
    mut shell: ResMut<Shell>,
    focus: Res<InputFocus>,
    focus_visible: Res<InputFocusVisible>,
    entries: Query<(Entity, &EquipOn, &Hovered, &ComputedNode, Has<InteractionDisabled>)>,
) {
    let place = shell.place();
    let open = shell.others_open() && shell.dragging().is_none();
    let mut in_view = entries
        .iter()
        .filter(|(_, entry, _, node, disabled)| open && entry.place == place && !node.is_empty() && !disabled);
    let pointed = in_view.clone().find(|(_, _, hovered, _, _)| hovered.get());
    let focused = in_view.find(|(entity, ..)| focus_visible.0 && focus.get() == Some(*entity));
    let wanted = pointed.or(focused).map(|(entity, entry, ..)| (entity, entry.horse));
    if shell.offered != wanted {
        shell.offer(wanted);
    }
}

/// The Gear tab shows the selected horse's gear and no other's (#314).
fn show_wearer(shell: Res<Shell>, mut horses: Query<(&Wearer, &mut Node, &mut Visibility)>) {
    for (wearer, mut node, mut visibility) in &mut horses {
        let shown = shell.wears(wearer.0);
        let wanted = if shown { Display::Flex } else { Display::None };
        if node.display != wanted {
            node.display = wanted;
        }
        // Hidden too, so that Tab passes over it (#244).
        let wanted = if shown {
            Visibility::Inherited
        } else {
            Visibility::Hidden
        };
        if *visibility != wanted {
            *visibility = wanted;
        }
    }
}

/// The carried piece is in its lists while there is one, and what an empty
/// list says is there while there is none. A report's piece is in the
/// report while the party has it. A cell of the Inventory is in its type's
/// group while its piece is held, and the group says so while it is empty.
fn show_carried(game: Res<Game>, mut parts: Query<(&When, &mut Node, &mut Visibility)>) {
    let listed = game.lessons.listed().len();
    let carried = game.pending_drop().is_some();
    let reported = game.reported_piece().is_some();
    let held: Vec<Vec<Piece>> = (0..Slot::ALL.len()).map(|slot| held(&game, slot)).collect();
    for (when, mut node, mut visibility) in &mut parts {
        let shown = match *when {
            When::Carried(when) => when == carried,
            When::Reported => reported,
            When::Held(piece, slot) => held[slot].contains(&piece),
            When::NoneHeld(slot) => held[slot].is_empty(),
            When::Listed(row) => row < listed,
            When::NoneListed => listed == 0,
        };
        let wanted = if shown { Display::Flex } else { Display::None };
        if node.display != wanted {
            node.display = wanted;
        }
        // Hidden too, so that Tab passes over it (#244).
        let wanted = if shown {
            Visibility::Inherited
        } else {
            Visibility::Hidden
        };
        if *visibility != wanted {
            *visibility = wanted;
        }
    }
}

/// What the list of hints says while none has been given (#300).
const NO_HINTS: &str = "No hint has been given yet. Each one is listed here once it has, to show again.";

/// A hint in the list is marked while it is the one showing, and is
/// unavailable while a press cannot show it: its line says why (#300).
fn mark_lessons(
    mut commands: Commands,
    game: Res<Game>,
    shell: Res<Shell>,
    map: Res<Places>,
    rows: Query<(Entity, &LessonRow, Has<Selected>, Has<InteractionDisabled>)>,
) {
    for (entity, row, selected, disabled) in &rows {
        let listed = teach::listed(&game, &map, shell.horse(), row.0);
        let showing = listed.as_ref().is_some_and(|listed| listed.showing);
        let blocked = listed.is_some_and(|listed| !listed.can);
        if showing && !selected {
            commands.entity(entity).insert(Selected);
        } else if !showing && selected {
            commands.entity(entity).remove::<Selected>();
        }
        if blocked && !disabled {
            commands.entity(entity).insert(InteractionDisabled);
        } else if !blocked && disabled {
            commands.entity(entity).remove::<InteractionDisabled>();
        }
    }
}

/// Say under each group of controls why those that work cannot be used now,
/// and put the line away while all of them can.
fn show_reasons(
    game: Res<Game>,
    shell: Res<Shell>,
    map: Res<Places>,
    mut lines: Query<(&Blocked, &mut Node, &Children)>,
    mut texts: Query<&mut Text>,
) {
    for (blocked, mut node, children) in &mut lines {
        let words = blocked_line(blocked.0.iter().map(|&id| map.control(id)), &game, shell.hand());
        let wanted = if words.is_empty() {
            Display::None
        } else {
            Display::Flex
        };
        if node.display != wanted {
            node.display = wanted;
        }
        for child in children.iter() {
            if let Ok(mut text) = texts.get_mut(child)
                && text.0 != words
            {
                text.0 = words.clone();
            }
        }
    }
}

/// What a live text says now, kept beside it: what it says in colour is in
/// spans under it, so its own text is not all of it.
#[derive(Component)]
struct Said(String);

/// A run of a live text, under it.
#[derive(Component)]
struct Run;

fn show_facts(
    mut commands: Commands,
    tokens: Res<Tokens>,
    game: Res<Game>,
    uncollected: Res<Uncollected>,
    shell: Res<Shell>,
    map: Res<Places>,
    live: Query<(Entity, &Live)>,
    children: Query<&Children>,
    values: Query<(), Or<(With<CounterValue>, With<RowValue>)>>,
    mut texts: Query<(&mut Text, &TextFont, &TextColor, Option<&PartText>, Option<&Said>)>,
    runs: Query<(), With<Run>>,
) {
    let names = ink::Names::of(&game.data.gear);
    for (entity, live) in &live {
        let target = if texts.contains(entity) {
            Some(entity)
        } else {
            children
                .iter_descendants(entity)
                .find(|child| values.contains(*child))
        };
        let Some(target) = target else {
            continue;
        };
        let Ok((mut text, font, colour, part, said)) = texts.get_mut(target) else {
            continue;
        };
        let words = fact(&game, &uncollected, &shell, &map, live.0);
        if said.is_some_and(|said| said.0 == words) {
            continue;
        }
        // The text itself is what comes before the first colour; each run
        // after that is a span under it, an element or a stat in its colour
        // (#251), a piece's name in its rarity's (#309), and the rest in the
        // text's own, which follows the state of the part it is in.
        let mut rest = ink::runs(&words, &names);
        let first = match rest.first() {
            Some((_, None)) => rest.remove(0).0,
            _ => "",
        };
        if text.0 != first {
            text.0 = first.into();
        }
        for child in children.get(target).into_iter().flatten() {
            if runs.contains(*child) {
                commands.entity(*child).despawn();
            }
        }
        for (run, ink) in rest {
            let mut span = commands.spawn((Run, TextSpan::new(run), font.clone(), ChildOf(target)));
            match (ink, part) {
                (Some(ink), _) => span.insert(TextColor(tokens.colour.ink(ink, &game.data.gear))),
                (None, Some(part)) => span.insert((*colour, *part)),
                (None, None) => span.insert(*colour),
            };
        }
        commands.entity(target).insert(Said(words));
    }
}

/// One stop per fight of a trail of `total`, with `cleared` of them behind
/// the party and the next the one it is at.
fn stops(total: usize, cleared: usize) -> Vec<Stop> {
    (0..total)
        .map(|fight| match fight.cmp(&cleared) {
            std::cmp::Ordering::Less => Stop::Behind,
            std::cmp::Ordering::Equal => Stop::Here,
            std::cmp::Ordering::Greater => Stop::Ahead,
        })
        .collect()
}

/// The trail as the tracker shows it, taken from the trail's plan: in town
/// the next trail, all of it ahead; on the road the one being walked. The
/// last stop is the boss.
fn trail_stops(game: &Game) -> Vec<Stop> {
    match &game.screen {
        Screen::Camp { .. } => vec![Stop::Ahead; game.plan.encounters.len()],
        Screen::Trail(expedition) => stops(
            expedition.plan.encounters.len(),
            match &expedition.outcome {
                Some(outcome) => outcome.cleared,
                None => expedition.encounter,
            },
        ),
    }
}

/// Draw the trail tracker again when the trail, or the party's place on it,
/// has changed.
fn show_trail(
    mut commands: Commands,
    tokens: Res<Tokens>,
    game: Res<Game>,
    tracker: Single<Entity, With<TrailTracker>>,
    mut shown: Local<Vec<Stop>>,
) {
    let stops = trail_stops(&game);
    if *shown == stops {
        return;
    }
    commands
        .entity(*tracker)
        .despawn_children()
        .with_children(|trail| {
            for (fight, stop) in stops.iter().enumerate() {
                trail.spawn(trail_mark(&tokens, *stop, fight + 1 == stops.len()));
            }
        });
    *shown = stops;
}

fn fact(game: &Game, uncollected: &Uncollected, shell: &Shell, map: &Places, fact: Fact) -> String {
    let selected = &game.party[shell.horse().min(game.party.len() - 1)];
    match fact {
        // A coin counts when it reaches the party, not when it drops.
        Fact::Coins => (game.coins_earned() - uncollected.coins).to_string(),
        Fact::Health(horse) => match &game.screen {
            Screen::Trail(expedition) => {
                let fighter = &expedition.party[horse];
                format!("{}/{}", fighter.health, fighter.max_health)
            }
            Screen::Camp { .. } => {
                let health = game.party[horse].stats().health;
                format!("{health}/{health}")
            }
        },
        Fact::SelectedHorse => format!("Selected horse: {}", selected.name),
        Fact::SelectedStats => {
            let stats = selected.stats();
            let named: Vec<String> = stats.named().iter().map(|(label, value)| format!("{label} {value}")).collect();
            format!("{}: {}", selected.name, named.join(", "))
        }
        Fact::Worn(horse, slot) => match &game.party[horse].gear[slot] {
            Some(gear) => gear.name.clone(),
            None => "nothing worn".into(),
        },
        Fact::WornLine(horse, slot) => match &game.party[horse].gear[slot] {
            // The slot, and what the piece in it adds.
            Some(gear) => format!("{}: {}", Slot::ALL[slot].name(), stats_line(gear.stats)),
            None => Slot::ALL[slot].name().into(),
        },
        Fact::Detail(place) => {
            let Some(detail) = shell.detail(place) else {
                return String::new();
            };
            match detail.item.piece() {
                // A piece is pinned to be compared, in every place (#312).
                Some(piece) => card::comparison_with(game, piece, shell.hand(), shell.rows(), shell.is_on(Switch::Details)),
                None => {
                    let id = match detail.item {
                        Item::Control(id) => id,
                        Item::Horse(horse) => {
                            let horse = horse.min(game.party.len() - 1);
                            let health = self::fact(game, uncollected, shell, map, Fact::Health(horse));
                            return card::horse_sheet(game, horse, &health);
                        }
                        _ => return String::new(),
                    };
                    let control = map.control(id);
                    match control.shows {
                        Some(Shows::Keys) => {
                            let horses: Vec<&str> =
                                game.party.iter().map(|horse| horse.name.as_str()).collect();
                            format!("{}\n{}", control.name, keys_list(map, &horses))
                        }
                        None => not_built(control),
                    }
                }
            }
        }
        Fact::HorsePart(place, tab) => match shell.detail(place).map(|detail| detail.item) {
            Some(Item::Horse(horse)) => horse_part(game, map, horse, tab),
            _ => String::new(),
        },
        Fact::HorseTrail(place) => match shell.detail(place).map(|detail| detail.item) {
            Some(Item::Horse(horse)) => horse_trail(game, horse),
            _ => String::new(),
        },
        // A dragged piece's card is the preview of its drop (#228).
        Fact::Hovered => match (shell.dragging(), shell.offered()) {
            (Some(drag), _) => drag
                .onto
                .and_then(|onto| game.drop_hand(drag.piece, onto.horse, onto.slot))
                .map_or(String::new(), |hand| card::dragged(game, drag.piece, hand, shell.rows())),
            // A horse of the list beside Equip (#356): the comparison on it.
            (None, Some((_, horse))) => shell
                .hand()
                .open
                .and_then(|piece| game.drop_hand(piece, horse, None).map(|hand| (piece, hand)))
                .map_or(String::new(), |(piece, hand)| card::offered(game, piece, hand, shell.rows())),
            (None, None) => shell.pointed().map_or(String::new(), |(_, piece)| {
                card::pointed_comparison(game, piece, shell.horse(), shell.rows())
            }),
        },
        Fact::CompareHint => card::compare_hint(selected),
        Fact::CarriedName => game
            .pending_drop()
            .map_or(String::new(), |gear| gear.name.clone()),
        Fact::CarriedKind => game
            .pending_drop()
            .map_or(String::new(), |gear| card::kind(game, gear)),
        Fact::WornBy(horse) => game.party[horse].name.clone(),
        Fact::ReportedName | Fact::ReportedKind | Fact::ReportedWhere => {
            let Some(at) = game.reported_piece() else {
                return String::new();
            };
            let Some(gear) = game.piece(at) else {
                return String::new();
            };
            match (fact, at) {
                (Fact::ReportedName, _) => gear.name.clone(),
                (Fact::ReportedKind, _) => card::kind(game, gear),
                // Where it is, and nothing of where it should go (#351).
                (_, Piece::Carried(_)) => card::IN_CARGO.into(),
                (_, Piece::Worn { horse, .. }) => format!("{} wears it.", game.party[horse].name),
            }
        }
        Fact::Abilities => {
            let rules = &game.data.rules;
            let horse = &game.party[shell.horse().min(game.party.len() - 1)];
            let ability = &game.data.abilities[horse.ability];
            format!(
                "{}: {}. {}\nA plain hit has no element; an ability's has its own. \
                 Fire beats Plant, Plant beats Ice, Ice beats Fire (strong x{}, resisted x{}).",
                horse.name,
                ability.named(),
                ability.does,
                rules.strong_multiplier,
                rules.weak_multiplier,
            )
        }
        // No blank line at the end: it would push the controls down.
        Fact::Report => report_text(game).trim_end().into(),
        Fact::ReportSide => report_side(game).trim_end().into(),
        Fact::LogHeading => log_heading(map, shell.log_hidden()),
        Fact::LessonWords(row) | Fact::LessonLine(row) => {
            match (teach::listed(game, map, shell.horse(), row), fact) {
                (Some(listed), Fact::LessonWords(_)) => listed.words,
                (Some(listed), _) => listed.line,
                (None, _) => String::new(),
            }
        }
        Fact::Footer => match &game.screen {
            // The tracker beside it shows the trail's fights. No town
            // leaves by itself, so there is no countdown, unless Keep going
            // is on (#305).
            Screen::Camp { .. } => match game.leaving_in() {
                Some(secs) => format!(
                    "Level {} trail. Keep going: leaving in {:.0}s.",
                    game.plan.level,
                    secs.ceil(),
                ),
                None => format!("Level {} trail. Waiting for Set out.", game.plan.level),
            },
            Screen::Trail(expedition) => {
                let total = expedition.plan.encounters.len();
                let at = expedition.encounter + 1;
                match expedition.phase {
                    _ if game.secs_to_town().is_some() => "On the way back to town.".into(),
                    Phase::Fighting => format!("On the trail: fighting encounter {at} of {total}."),
                    Phase::Finished => format!("On the trail: all {total} encounters cleared."),
                    _ => format!("On the trail: walking to encounter {at} of {total}."),
                }
            }
        },
    }
}

/// Whether an action's control is shown chosen: an ability's while the
/// selected horse has it (#357).
fn action_chosen(game: &Game, shell: &Shell, action: Action) -> bool {
    match action {
        Action::Ability(ability) => game.party.get(shell.horse()).is_some_and(|horse| horse.ability == ability),
        _ => false,
    }
}

/// What a horse has of one tab of the horses' place (#311), under that
/// tab's control in the horse's view. A part that is not built says so.
fn horse_part(game: &Game, map: &Places, horse: usize, tab: usize) -> String {
    let horse = horse.min(game.party.len() - 1);
    let tab = map
        .places
        .iter()
        .find(|place| place.horse_columns)
        .and_then(|place| place.tabs.get(tab));
    match tab {
        Some(tab) => match tab.content {
            Content::Gear => card::horse_wears(game, horse),
            Content::Abilities => {
                let ability = &game.data.abilities[game.party[horse].ability];
                format!("{}. {}", ability.named(), ability.does)
            }
            Content::Planned => format!("Not built yet. {}", tab.job),
            Content::Report | Content::Inventory | Content::Hints => String::new(),
        },
        None => String::new(),
    }
}

/// What a horse did on the last trail the party finished (#311), a short
/// line a thing, for the narrowest side panel.
fn horse_trail(game: &Game, horse: usize) -> String {
    let Some(kept) = game.last_report() else {
        return "No trail walked yet.".into();
    };
    let Some(deeds) = kept.outcome.deeds.get(horse) else {
        return String::new();
    };
    let count = |n: u32, one: &str, many: &str| format!("{n} {}", if n == 1 { one } else { many });
    let mut lines = vec![
        format!(
            "Trail {}, {}",
            kept.number,
            if kept.outcome.won { "won" } else { "lost" }
        ),
        format!("Dealt {}, {}", deeds.dealt, count(deeds.kills, "kill", "kills")),
        format!("{} strong, {} resisted", deeds.strong_hits, deeds.resisted_hits),
        format!("Took {}", deeds.taken),
    ];
    if deeds.wards > 0 {
        lines.push(format!("Raised {}", count(deeds.wards, "Ward", "Wards")));
    }
    if deeds.fell {
        lines.push("Fell in the fight".into());
    }
    lines.join("
")
}

/// The working controls among these that cannot be used now, gathered by the
/// reason, in the order of the map. The reason is `Game::can`'s, the check
/// `Game::act` makes, so what is shown is why the press would do nothing.
fn blocked<'a>(
    controls: impl IntoIterator<Item = &'a Control>,
    game: &Game,
    hand: Hand,
) -> Vec<(Vec<&'a str>, &'static str)> {
    let mut reasons: Vec<(Vec<&str>, &str)> = Vec::new();
    for control in controls {
        let Some(Err(why)) = control.action.map(|action| game.can(action, hand)) else {
            continue;
        };
        match reasons.iter_mut().find(|(_, reason)| *reason == why) {
            Some((names, _)) => names.push(&control.name),
            None => reasons.push((vec![&control.name], why)),
        }
    }
    reasons
}

/// What a group of controls says in place about those: "Fire, Ice, Plant:
/// in town only." Empty when every working control can be used.
fn blocked_line<'a>(
    controls: impl IntoIterator<Item = &'a Control>,
    game: &Game,
    hand: Hand,
) -> String {
    let lines: Vec<String> = blocked(controls, game, hand)
        .iter()
        .map(|(names, why)| reason_line(names, why))
        .collect();
    lines.join(" ")
}

/// What a group of controls says in place about those of them that are not
/// built. It names them, or, where that is shorter, the ones that work:
/// "All but Equip: not built yet." `hint` adds that a press says more,
/// where there is room for it.
fn unbuilt_line(controls: &[Control], hint: bool) -> String {
    let (working, unbuilt): (Vec<&Control>, Vec<&Control>) =
        controls.iter().partition(|control| control.built());
    if unbuilt.is_empty() {
        return String::new();
    }
    let names = |controls: &[&Control]| {
        let names: Vec<&str> = controls.iter().map(|c| c.name.as_str()).collect();
        names.join(", ")
    };
    let mut out = if working.is_empty() && unbuilt.len() > 1 {
        "None of these is built yet.".to_string()
    } else if !working.is_empty() && working.len() < unbuilt.len() {
        format!("All but {}: not built yet.", names(&working))
    } else {
        format!("{}: not built yet.", names(&unbuilt))
    };
    if hint {
        out += match unbuilt.len() {
            1 => " Press it to see what it will do.",
            _ => " Press one to see what it will do.",
        };
    }
    out
}

/// The controls a reason is about, then the reason, as one sentence.
fn reason_line(names: &[&str], why: &str) -> String {
    let mut letters = why.chars();
    let why: String = match letters.next() {
        Some(first) => first.to_lowercase().chain(letters).collect(),
        None => String::new(),
    };
    format!("{}: {}", names.join(", "), why)
}

/// What a control that is not built says when it is pressed.
fn not_built(control: &Control) -> String {
    let mut out = format!("{}\nNot built yet.\n{}", control.name, control.does);
    if let Some(issue) = control.issue {
        out += &match control.proposed {
            true => format!("\nA proposal, not decided: issue #{issue}."),
            false => format!("\nPlanned: issue #{issue}."),
        };
    }
    out
}

/// The heading of the fight log (#306), with its key printed on it; it
/// stays when the lines are hidden, so the key that brings them back does.
fn log_heading(map: &Places, hidden: bool) -> String {
    match hidden {
        false => format!("FIGHT LOG  [{}] hides it", map.log_key),
        true => format!("FIGHT LOG hidden  [{}] shows it", map.log_key),
    }
}

/// The line that names the scroll keys, under words that do not fit.
fn scroll_hint(map: &Places) -> String {
    format!("{}, {}: read the rest", map.scroll_keys.0, map.scroll_keys.1)
}

/// What the Keys control shows (#229): every key and what it works, taken
/// from the map, so a key changed in `places.ron` is changed here. Escape,
/// Tab and Enter are not in the file; what they do is in `escape` and Bevy.
fn keys_list(map: &Places, horses: &[&str]) -> String {
    let pair = |key: &str, name: &str| format!("{key} {name}");
    let places: Vec<String> = map
        .pages()
        .filter_map(|page| Some(pair(page.key.as_deref()?, &page.name)))
        .collect();
    // Every place numbers its own tabs, so the keys are listed once.
    let mut tabs: Vec<&str> = Vec::new();
    for key in map
        .pages()
        .flat_map(|page| &page.tabs)
        .filter_map(|tab| tab.key.as_deref())
    {
        if !tabs.contains(&key) {
            tabs.push(key);
        }
    }
    let party: Vec<String> = map
        .horse_keys
        .iter()
        .zip(horses)
        .map(|(key, name)| pair(key, name))
        .collect();
    let actions: Vec<String> = map
        .list
        .iter()
        .filter_map(|control| Some(pair(control.key.as_deref()?, &control.name)))
        .collect();
    [
        format!("Places: {}", places.join(", ")),
        format!("Tabs of a place: {}", tabs.join(", ")),
        format!("Horses: {}", party.join(", ")),
        format!("Actions: {}", actions.join(", ")),
        "Esc: back one level".into(),
        "Arrows: move focus that way".into(),
        format!("{}, {}: scroll", map.scroll_keys.0, map.scroll_keys.1),
        format!("{}: hide or show the fight log, on the road", map.log_key),
        "Tab, Shift+Tab: move focus".into(),
        "Enter: press what has focus".into(),
    ]
    .join("\n")
}

#[cfg(test)]
mod tests {
    use super::*;

    /// "No control is silent" (#238): every control says what it does, and
    /// one that does nothing yet names the issue that will make it work.
    #[test]
    fn no_control_is_silent() {
        let places = Places::load();
        assert!(!places.controls().is_empty());
        for (id, control) in places.controls().iter().enumerate() {
            assert_eq!(control.id, id);
            assert!(!control.name.is_empty());
            assert!(!control.does.is_empty(), "{} does nothing", control.name);
            assert!(
                control.built() || control.issue.is_some(),
                "{} neither works nor names its issue",
                control.name
            );
            if !control.built() {
                assert!(not_built(control).contains("Not built yet."));
            }
        }
    }

    fn a_hint(lesson: &str, about: About) -> Hint {
        Hint {
            lesson: lesson.into(),
            about,
            words: "One line.".into(),
        }
    }

    /// The hint (#291): one shows at a time; one the player put away does
    /// not come back by itself, and one whose thing was done may; after
    /// Skip all, none is shown.
    #[test]
    fn a_hint_put_away_does_not_come_back_by_itself() {
        let places = Places::load();
        let mut shell = Shell::new(places.pages().count());
        assert!(shell.hint().is_none());
        assert!(!shell.dismiss_hint(), "nothing to put away");

        assert!(shell.raise_hint(a_hint("first", About::Place(0))));
        assert!(!shell.raise_hint(a_hint("second", About::Horse(1))), "one at a time");
        assert_eq!(shell.hint().map(|hint| hint.lesson.as_str()), Some("first"));

        assert!(shell.dismiss_hint());
        assert!(shell.hint().is_none());
        assert!(!shell.raise_hint(a_hint("first", About::Place(0))), "it came back");
        assert!(shell.raise_hint(a_hint("second", About::Horse(1))));

        // Done, not put away: its lesson may show it again.
        shell.finish_hint();
        assert!(shell.raise_hint(a_hint("second", About::Horse(1))));

        shell.skip_hints();
        assert!(shell.hint().is_none());
        assert!(!shell.raise_hint(a_hint("second", About::Horse(1))));
        assert!(!shell.raise_hint(a_hint("third", About::Place(1))));

        shell.forget_hints();
        assert!(shell.raise_hint(a_hint("first", About::Place(0))));

        // Asked for again (#300): shown though it was put away and though
        // the hints are off, in the place of the one showing, and it is no
        // longer put away.
        assert!(shell.dismiss_hint() && shell.put_away("first"));
        shell.skip_hints();
        shell.show_again(a_hint("first", About::Place(0)));
        assert_eq!(shell.hint().map(|hint| hint.lesson.as_str()), Some("first"));
        assert!(!shell.put_away("first") && shell.hints_off());
    }

    /// Hints (#300) is one switch, in Settings, on in a new game. Off, the
    /// hint showing goes without being put away and none is raised; on
    /// again, it can be. Skip all hints turns the same switch off. It
    /// changes nothing in the game, and the Journal has the list on a tab.
    #[test]
    fn hints_are_turned_off_and_on_in_settings() {
        let places = Places::load();
        let switches: Vec<&Control> = places
            .controls()
            .iter()
            .filter(|control| control.switch == Some(Switch::Hints))
            .collect();
        assert_eq!(switches.len(), 1);
        assert!(places.settings.controls.iter().any(|control| control.id == switches[0].id));
        let lists = places
            .pages()
            .flat_map(|page| &page.tabs)
            .filter(|tab| tab.content == Content::Hints)
            .count();
        assert_eq!(lists, 1, "one list of hints");

        let mut shell = Shell::new(places.pages().count());
        assert!(shell.is_on(Switch::Hints));
        assert!(shell.raise_hint(a_hint("first", About::Place(0))));
        shell.press_switch(Switch::Hints);
        assert!(!shell.is_on(Switch::Hints) && shell.hint().is_none());
        assert!(!shell.put_away("first"), "turned off is not put away");
        assert!(!shell.raise_hint(a_hint("first", About::Place(0))));
        shell.press_switch(Switch::Hints);
        assert!(shell.is_on(Switch::Hints));
        assert!(shell.raise_hint(a_hint("first", About::Place(0))));

        shell.skip_hints();
        assert!(!shell.is_on(Switch::Hints) && shell.put_away("first"));
    }

    /// Damage numbers (#249) is one switch, in Settings, with a key, on in a
    /// new game; a press turns it off and the next on again, and it touches
    /// nothing else.
    #[test]
    fn damage_numbers_are_turned_off_and_on_in_settings() {
        let places = Places::load();
        let switches: Vec<&Control> = places
            .controls()
            .iter()
            .filter(|control| control.switch == Some(Switch::DamageNumbers))
            .collect();
        assert_eq!(switches.len(), 1);
        assert!(switches[0].key.is_some());
        assert!(places.settings.controls.iter().any(|control| control.id == switches[0].id));

        let mut shell = Shell::new(places.pages().count());
        assert!(shell.is_on(Switch::DamageNumbers));
        let before = shell.clone();
        shell.press_switch(Switch::DamageNumbers);
        assert!(!shell.is_on(Switch::DamageNumbers) && shell.is_on(Switch::Hints));
        shell.press_switch(Switch::DamageNumbers);
        assert_eq!(shell, before);
    }

    /// Where the hints went (#350) is owed by Skip all hints and by nothing
    /// else, is said once on a free line though hints are off, and is never
    /// said again: not by a second Skip all, and not while hints are on.
    /// Turning hints on takes it off the line.
    #[test]
    fn where_the_hints_went_is_said_once() {
        let places = Places::load();
        let parting = || a_hint(PARTING, About::Tab(0, 1));
        let mut shell = Shell::new(places.pages().count());
        assert!(!shell.owes_parting() && !shell.say_parting(parting()), "hints are on");

        // The switch in Settings says so itself.
        shell.press_switch(Switch::Hints);
        assert!(!shell.owes_parting() && !shell.say_parting(parting()));
        shell.press_switch(Switch::Hints);

        // Skip all, on a hint: its line is free, and the answer has it.
        assert!(shell.raise_hint(a_hint("first", About::Place(0))));
        shell.skip_hints();
        assert!(shell.owes_parting());
        assert!(shell.say_parting(parting()) && shell.parting() && shell.hints_off());
        assert!(!shell.owes_parting() && !shell.say_parting(parting()));

        // Hints turned on: it goes, and is not a hint put away.
        shell.press_switch(Switch::Hints);
        assert!(shell.hint().is_none() && shell.is_on(Switch::Hints));
        assert!(shell.raise_hint(a_hint("second", About::Place(0))));

        // A second Skip all is not answered.
        shell.skip_hints();
        assert!(shell.hint().is_none() && !shell.owes_parting() && !shell.say_parting(parting()));

        // Owed and not yet said, then hints turned on: nothing is owed.
        let mut shell = Shell::new(places.pages().count());
        shell.skip_hints();
        shell.press_switch(Switch::Hints);
        assert!(!shell.owes_parting());
        shell.press_switch(Switch::Hints);
        assert!(!shell.owes_parting(), "the switch does not raise it");

        // Got it puts it away like any hint.
        let mut shell = Shell::new(places.pages().count());
        shell.skip_hints();
        assert!(shell.say_parting(parting()) && shell.dismiss_hint());
        assert!(shell.hint().is_none() && !shell.owes_parting());

        // Where a hint's place is: open at it, on its tab.
        let mut shell = Shell::new(places.pages().count());
        assert!(shell.at(About::Place(0)) && shell.at(About::Tab(0, 0)));
        assert!(!shell.at(About::Tab(0, 1)) && !shell.at(About::Place(1)) && !shell.at(About::Horse(0)));
        shell.choose_tab(0, 1);
        assert!(shell.at(About::Tab(0, 1)));
    }

    /// A hint names its control as the control names itself, and prints
    /// the key printed on it (#291), for every kind of control it can be
    /// about. One about something that is not there says nothing.
    #[test]
    fn a_hint_names_its_control_and_prints_its_key() {
        let places = Places::load();
        let game = Game::new(6);
        let named = |about| about_name(&places, &game, about);
        for (p, page) in places.pages().enumerate() {
            assert_eq!(named(About::Place(p)), (page.name.clone(), page.key.clone()));
            for (t, tab) in page.tabs.iter().enumerate() {
                assert_eq!(named(About::Tab(p, t)), (tab.name.clone(), tab.key.clone()));
            }
        }
        for (h, horse) in game.party.iter().enumerate() {
            let key = places.horse_key(h).map(String::from);
            assert!(key.is_some());
            assert_eq!(named(About::Horse(h)), (horse.name.clone(), key));
        }
        for control in places.controls() {
            assert_eq!(
                named(About::Control(control.id)),
                (control.name.clone(), control.key.clone())
            );
        }
        assert_eq!(named(About::Control(usize::MAX)), Default::default());
        assert_eq!(named(About::Horse(99)), Default::default());

        // Every sample is about something the map has, and no two about
        // the same thing.
        let samples: Vec<String> = (0..SAMPLE_HINTS)
            .map(|which| named(sample_hint(&places, which).about).0)
            .collect();
        assert_eq!(samples[0], "Journal");
        for (i, name) in samples.iter().enumerate() {
            assert!(!name.is_empty());
            assert!(!samples[..i].contains(name), "{name} twice");
        }
    }

    /// The party in each state the slice has: in town before any trail, in
    /// town with a drop to equip, and on the road.
    fn states() -> Vec<(&'static str, Game)> {
        let with_a_drop = (0..50)
            .map(Game::back_from_a_trail)
            .find(|game| game.pending_drop().is_some())
            .expect("no seed under 50 brings a drop home");
        let mut on_the_road = Game::new(6);
        on_the_road.act(Action::SetOut, Hand::horse(0));
        vec![
            ("in town", Game::new(6)),
            ("in town with a drop", with_a_drop),
            ("on the road", on_the_road),
        ]
    }

    /// "Nothing is silently unavailable" (#233): in every state, a control
    /// that cannot be used is named in a line under its group, and for one
    /// that works the reason given is the one `Game::can` refuses it with.
    /// A control that can be used is in neither line.
    #[test]
    fn no_unavailable_control_is_without_its_reason() {
        let places = Places::load();
        let grouped: usize = places.groups().iter().map(|group| group.len()).sum();
        assert_eq!(grouped, places.controls().len(), "a control is in no group");
        for (state, game) in states() {
            for group in places.groups() {
                let reasons = blocked(group, &game, Hand::default());
                let unbuilt = unbuilt_line(group, true);
                for control in group {
                    let name = control.name.as_str();
                    let given: Vec<&str> = reasons
                        .iter()
                        .filter(|(names, _)| names.contains(&name))
                        .map(|(_, why)| *why)
                        .collect();
                    match control.action.map(|action| game.can(action, Hand::default())) {
                        Some(Ok(())) => assert!(given.is_empty(), "{state}: {name}"),
                        Some(Err(why)) => {
                            assert_eq!(given, [why], "{state}: {name}");
                            assert!(!why.is_empty());
                            let line = blocked_line(group, &game, Hand::default());
                            assert!(line.contains(name), "{state}: {line}");
                        }
                        // It shows something, whatever the party is doing:
                        // no line says it cannot be used.
                        None if control.built() => {
                            assert!(given.is_empty(), "{state}: {name}");
                            let named = unbuilt.contains(name);
                            assert_eq!(named, unbuilt.starts_with("All but "), "{unbuilt}");
                        }
                        // Named, or covered by a line that names the others.
                        None => {
                            assert!(given.is_empty(), "{state}: {name}");
                            let covered = unbuilt.starts_with("None of these")
                                || unbuilt.starts_with("All but ")
                                || unbuilt.contains(name);
                            assert!(covered, "{state}: {unbuilt}");
                            assert!(unbuilt.contains("built yet"), "{unbuilt}");
                        }
                    }
                }
            }
        }
    }

    /// The trail tracker (#253) has a mark for every fight of the plan,
    /// however long the trail, and every fight left is counted ahead.
    #[test]
    fn the_tracker_has_a_mark_for_every_fight_of_the_plan() {
        use Stop::{Ahead, Behind, Here};
        assert_eq!(stops(4, 1), [Behind, Here, Ahead, Ahead]);
        assert_eq!(stops(1, 0), [Here]);
        for total in 1..9 {
            for cleared in 0..=total {
                let stops = stops(total, cleared);
                assert_eq!(stops.len(), total);
                assert_eq!(stops.iter().filter(|s| **s == Behind).count(), cleared);
                assert!(stops.iter().filter(|s| **s == Here).count() <= 1);
            }
        }

        // In town it is the next trail, none of it walked.
        let mut game = Game::new(6);
        let total = game.plan.encounters.len();
        assert_eq!(trail_stops(&game), vec![Ahead; total]);

        // On the road it follows the party, and a longer trail has more marks.
        game.act(Action::SetOut, Hand::horse(0));
        assert_eq!(trail_stops(&game), stops(total, 0));
        let Screen::Trail(expedition) = &mut game.screen else {
            unreachable!("the party set out");
        };
        expedition.encounter = 2;
        let another = expedition.plan.encounters[0].clone();
        expedition.plan.encounters.push(another);
        assert_eq!(trail_stops(&game), stops(total + 1, 2));

        // A won trail is all behind, the boss too.
        let mut won = Game::new(6);
        won.act(Action::SetOut, Hand::horse(0));
        let Screen::Trail(expedition) = &mut won.screen else {
            unreachable!("the party set out");
        };
        while expedition.outcome.is_none() {
            expedition.tick(&won.data, 0.1);
        }
        let outcome = expedition.outcome.clone().unwrap();
        assert!(outcome.won, "seed 6 wins its first trail");
        assert_eq!(trail_stops(&won), vec![Behind; total]);
    }

    #[test]
    fn a_reason_reads_as_one_sentence() {
        assert_eq!(
            reason_line(&["Fire", "Ice"], "In town only."),
            "Fire, Ice: in town only."
        );
        let mut game = Game::new(6);
        let places = Places::load();
        assert_eq!(blocked_line(&places.footer, &game, Hand::default()), "");
        // Undo (#230) says why it has nothing to do, until something is done.
        let gear = &places.places[0].tabs[0].controls;
        assert_eq!(
            blocked_line(gear, &game, Hand::default()),
            "Undo: nothing changed this visit."
        );
        game.act(Action::Ability(1), Hand::horse(0));
        assert_eq!(blocked_line(gear, &game, Hand::default()), "");
    }

    /// The line about controls that are not built names whichever is the
    /// shorter list, and a control that works is never said to be unbuilt.
    #[test]
    fn the_unbuilt_line_names_the_shorter_list() {
        let places = Places::load();
        for group in places.groups() {
            let line = unbuilt_line(group, false);
            let working = group.iter().filter(|c| c.built());
            if line.starts_with("All but ") {
                for control in working {
                    assert!(line.contains(control.name.as_str()), "{line}");
                }
            } else if !line.starts_with("None of these") {
                for control in working {
                    assert!(!line.contains(control.name.as_str()), "{line}");
                }
            }
        }
        let cargo = &places.places[0].cargo;
        assert_eq!(unbuilt_line(cargo, false), "None of these is built yet.");
        assert!(unbuilt_line(&places.card, false).starts_with("All but Equip, Another horse, Changed only, Details: "));
        assert_eq!(
            unbuilt_line(&places.footer, false),
            "Planned changes: not built yet."
        );
    }

    /// An empty list says what belongs in it and where that comes from
    /// (#233): the cargo, an empty slot and the report, before any trail.
    #[test]
    fn an_empty_list_says_what_belongs_in_it() {
        let places = Places::load();
        let game = Game::new(6);
        let uncollected = Uncollected::default();
        let mut shell = Shell::new(places.pages().count());
        shell.show_detail(
            0,
            Detail {
                tab: 0,
                item: Item::Worn { horse: 0, slot: 0 },
                opener: None,
            },
        );
        let say = |what| fact(&game, &uncollected, &shell, &places, what);
        assert!(card::NOTHING_CARRIED.contains("Gear the horses bring back"));
        assert_eq!(say(Fact::CarriedName), "");
        assert!(say(Fact::Detail(0)).contains("Gear comes from the trail"));
        assert!(say(Fact::Report).contains("is reported here"));
    }

    /// A printed key is one the game can read, works a control that works,
    /// and works only that control. The keys of places, horses and controls
    /// work everywhere, so no two of them are the same.
    #[test]
    fn every_key_is_a_real_key_and_works_one_control() {
        let places = Places::load();
        let mut seen = Vec::new();
        for control in places.controls() {
            let Some(key) = &control.key else { continue };
            assert!(
                key_code(key).is_some(),
                "{}: no key called {key}",
                control.name
            );
            assert!(
                control.built(),
                "{} has a key and does nothing",
                control.name
            );
            assert!(!seen.contains(key), "two controls answer {key}");
            seen.push(key.clone());
        }
        let pages = places.pages().filter_map(|page| page.key.as_ref());
        let scroll = [&places.scroll_keys.0, &places.scroll_keys.1, &places.log_key];
        for key in pages.chain(&places.horse_keys).chain(scroll) {
            assert!(key_code(key).is_some(), "no key called {key}");
            assert!(!seen.contains(key), "two things answer {key}");
            seen.push(key.clone());
        }
    }

    /// "A key for everything" (#229): every place, settings, every tab and
    /// every horse of the party has a key. A tab's key works in its place
    /// only: no other tab there has it, and nothing that works everywhere.
    #[test]
    fn every_place_tab_and_horse_has_its_key() {
        let places = Places::load();
        let mut everywhere: Vec<&str> = places
            .controls()
            .iter()
            .filter_map(|control| control.key.as_deref())
            .collect();
        everywhere.extend(places.horse_keys.iter().map(String::as_str));
        for (p, page) in places.pages().enumerate() {
            let key = places.page_key(p);
            assert!(key.is_some(), "{} has no key", page.name);
            everywhere.extend(key);
        }
        for (p, page) in places.pages().enumerate() {
            let mut seen = Vec::new();
            for (i, tab) in page.tabs.iter().enumerate() {
                let Some(key) = places.tab_key(p, i) else {
                    panic!("{}, {} has no key", page.name, tab.name);
                };
                assert!(key_code(key).is_some(), "{}: no key called {key}", tab.name);
                assert!(!seen.contains(&key), "{}: two tabs answer {key}", page.name);
                assert!(!everywhere.contains(&key), "{}: {key} is taken", tab.name);
                seen.push(key);
            }
        }
        let party = crate::sim::data::Data::load().horses.len();
        assert!(places.horse_keys.len() >= party, "a horse has no key");
    }

    /// The Keys control lists every key in the file beside what it works
    /// (#229), and is itself a control that works, on the Settings page.
    #[test]
    fn the_keys_control_lists_every_key() {
        let places = Places::load();
        let data = crate::sim::data::Data::load();
        let horses: Vec<&str> = data
            .horses
            .iter()
            .map(|horse| horse.name.as_str())
            .collect();
        let list = keys_list(&places, &horses);
        let line = |start: &str| {
            list.lines()
                .find(|line| line.starts_with(start))
                .unwrap_or_else(|| panic!("no line for {start}"))
        };
        for page in places.pages() {
            let key = page.key.as_deref().unwrap();
            assert!(
                line("Places").contains(&format!("{key} {}", page.name)),
                "{list}"
            );
            for tab in &page.tabs {
                let key = tab.key.as_deref().unwrap();
                assert!(
                    line("Tabs").split([' ', ',']).any(|word| word == key),
                    "{list}"
                );
            }
        }
        for (key, horse) in places.horse_keys.iter().zip(&horses) {
            assert!(line("Horses").contains(&format!("{key} {horse}")), "{list}");
        }
        let mut keyed = 0;
        for control in places.controls() {
            let Some(key) = &control.key else { continue };
            assert!(
                line("Actions").contains(&format!("{key} {}", control.name)),
                "{list}"
            );
            keyed += 1;
        }
        assert!(keyed > 0);

        let shows: Vec<&Control> = places
            .settings
            .controls
            .iter()
            .filter(|c| c.shows == Some(Shows::Keys))
            .collect();
        assert_eq!(shows.len(), 1, "Settings has one Keys control");
        assert!(shows[0].built() && shows[0].action.is_none());

        // Pressed, it says the list in the side panel of Settings.
        let page = places.settings_page();
        let mut shell = Shell::new(places.pages().count());
        shell.show_detail(
            page,
            Detail {
                tab: 0,
                item: Item::Control(shows[0].id),
                opener: None,
            },
        );
        let game = Game::new(6);
        let said = fact(
            &game,
            &Uncollected::default(),
            &shell,
            &places,
            Fact::Detail(page),
        );
        assert!(said.starts_with(&shows[0].name), "{said}");
        assert!(
            said.contains("Space Set out") && !said.contains("Not built"),
            "{said}"
        );
        assert!(said.contains("Arrows:"), "{said}");
        let (up, down) = &places.scroll_keys;
        assert!(said.contains(&format!("{up}, {down}: scroll")), "{said}");
        assert!(scroll_hint(&places).starts_with(&format!("{up}, {down}")));
        assert!(said.contains(&format!("{}: hide or show the fight log", places.log_key)), "{said}");
    }

    /// Changed only (#349) is a control of the card with its key, in the
    /// Keys list. A press narrows the table of the pinned comparison and of
    /// the one under the pointer, changes nothing in the game, and stays as
    /// it was set from piece to piece and place to place. It does nothing
    /// while no piece's card is pinned in view.
    #[test]
    fn changed_only_is_one_switch_for_every_piece_and_place() {
        let places = Places::load();
        let switches: Vec<&Control> = places
            .controls()
            .iter()
            .filter(|control| control.switch == Some(Switch::ChangedOnly))
            .collect();
        assert_eq!(switches.len(), 1);
        let control = switches[0];
        assert!(places.card.iter().any(|on_card| on_card.id == control.id), "it is on the card");
        assert!(control.built() && control.action.is_none() && control.shows.is_none());
        let key = control.key.as_deref().expect("its key is printed on it");
        let listed = keys_list(&places, &[]);
        assert!(listed.contains(&format!("{key} {}", control.name)), "{listed}");

        // A piece with one stat the same as what the selected horse wears.
        let mut game = (0..50)
            .map(Game::back_from_a_trail)
            .find(|game| game.pending_drop().is_some())
            .unwrap();
        let gear = game.pending_drop().unwrap().clone();
        let mut worn = gear.clone();
        worn.name = "Old Thing".into();
        worn.stats.health += 3;
        game.party[0].gear[gear.slot.index()] = Some(worn.clone());
        let uncollected = Uncollected::default();
        let same = |shell: &Shell, place: usize| {
            fact(&game, &uncollected, shell, &places, Fact::Detail(place))
                .lines()
                .filter(|row| row.ends_with("  same"))
                .count()
        };

        let mut shell = Shell::new(places.pages().count());
        shell.press_switch(Switch::ChangedOnly);
        assert!(!shell.is_on(Switch::ChangedOnly), "nothing is pinned");
        let pin = |shell: &mut Shell, place: usize| {
            shell.open(place);
            let tab = shell.tab(place);
            shell.show_detail(place, Detail { tab, item: Item::Carried(0), opener: None });
        };
        pin(&mut shell, 0);
        assert!(same(&shell, 0) > 0);
        shell.press_switch(Switch::ChangedOnly);
        assert!(shell.is_on(Switch::ChangedOnly));
        assert_eq!(same(&shell, 0), 0);
        let said = fact(&game, &uncollected, &shell, &places, Fact::Detail(0));
        assert!(said.contains("Changed only: "), "{said}");

        // Another place, the card put away, and the card under the pointer.
        pin(&mut shell, 1);
        assert_eq!(same(&shell, 1), 0);
        shell.close_detail();
        assert!(shell.is_on(Switch::ChangedOnly));
        shell.point(Some((Entity::PLACEHOLDER, Piece::Carried(0))));
        let pointed = fact(&game, &uncollected, &shell, &places, Fact::Hovered);
        assert!(pointed.contains("Changed only: "), "{pointed}");
        pin(&mut shell, 1);
        shell.press_switch(Switch::ChangedOnly);
        assert!(same(&shell, 1) > 0 && same(&shell, 0) > 0);
        // A switch changes nothing in the game.
        assert_eq!(game.party[0].gear[gear.slot.index()].as_ref(), Some(&worn));
        assert_eq!(game.pending_drop(), Some(&gear));
    }

    /// Details (#228) is a control of the card with its key, in the Keys
    /// list. Pressed with a card pinned it adds the card's formulas there,
    /// in every place, and stays set; it changes nothing in the game, and
    /// nothing while no card is pinned. The card under the pointer keeps
    /// to the comparison.
    #[test]
    fn details_is_one_switch_for_every_piece_and_place() {
        let places = Places::load();
        let control = places
            .card
            .iter()
            .find(|control| control.switch == Some(Switch::Details))
            .expect("Details is on the card");
        assert!(control.built() && control.action.is_none() && control.shows.is_none());
        let key = control.key.as_deref().expect("its key is printed on it");
        assert!(keys_list(&places, &[]).contains(&format!("{key} {}", control.name)));

        let game = (0..50)
            .map(Game::back_from_a_trail)
            .find(|game| game.pending_drop().is_some())
            .unwrap();
        let uncollected = Uncollected::default();
        let said = |shell: &Shell, place: usize| fact(&game, &uncollected, shell, &places, Fact::Detail(place));
        let mut shell = Shell::new(places.pages().count());
        shell.press_switch(Switch::Details);
        assert!(!shell.is_on(Switch::Details), "nothing is pinned");
        let pin = |shell: &mut Shell, place: usize| {
            shell.open(place);
            let tab = shell.tab(place);
            shell.show_detail(place, Detail { tab, item: Item::Carried(0), opener: None });
        };
        pin(&mut shell, 0);
        assert!(!said(&shell, 0).contains("DETAILS"));
        shell.press_switch(Switch::Details);
        assert!(shell.is_on(Switch::Details));
        assert!(said(&shell, 0).contains("DETAILS"));
        pin(&mut shell, 1);
        assert!(said(&shell, 1).contains("DETAILS"), "another place");
        shell.close_detail();
        shell.point(Some((Entity::PLACEHOLDER, Piece::Carried(0))));
        let pointed = fact(&game, &uncollected, &shell, &places, Fact::Hovered);
        assert!(!pointed.is_empty() && !pointed.contains("DETAILS"), "{pointed}");
        pin(&mut shell, 1);
        shell.press_switch(Switch::Details);
        assert!(!said(&shell, 1).contains("DETAILS") && !said(&shell, 0).contains("DETAILS"));
    }

    /// Another horse (#356) opens a list of the other horses beside Equip,
    /// in the line's order across the scene. Choosing one equips the
    /// card's piece on it, selects it so the card compares there, and
    /// closes the list; the pointer on one shows what that would do. The
    /// horse that wears the piece cannot take it, and a new card closes
    /// the list.
    #[test]
    fn another_horse_equips_on_the_horse_chosen() {
        let places = Places::load();
        let control = places
            .card
            .iter()
            .find(|control| control.switch == Some(Switch::OtherHorses))
            .expect("Another horse is on the card");
        let key = control.key.as_deref().expect("its key is printed on it");
        assert!(keys_list(&places, &[]).contains(&format!("{key} {}", control.name)));
        assert_eq!(line_order(3), vec![2, 1, 0], "left to right, the leader last");

        let mut game = (0..50)
            .map(Game::back_from_a_trail)
            .find(|game| game.pending_drop().is_some())
            .unwrap();
        let gear = game.pending_drop().unwrap().clone();
        let slot = gear.slot.index();
        let uncollected = Uncollected::default();
        let mut shell = Shell::new(places.pages().count());
        shell.press_switch(Switch::OtherHorses);
        assert!(!shell.is_on(Switch::OtherHorses), "nothing is pinned");
        shell.open(0);
        let tab = shell.tab(0);
        shell.show_detail(0, Detail { tab, item: Item::Carried(0), opener: None });
        shell.press_switch(Switch::OtherHorses);
        assert!(shell.is_on(Switch::OtherHorses));

        shell.offer(Some((Entity::PLACEHOLDER, 2)));
        let offered = fact(&game, &uncollected, &shell, &places, Fact::Hovered);
        let name = game.party[2].name.clone();
        assert!(offered.starts_with(&format!("For {name}, ")), "{offered}");
        assert!(offered.contains(&format!("Equip puts it on {name}")), "{offered}");
        assert!(offered.ends_with(card::CHOOSE), "{offered}");

        assert!(shell.equip_on(&mut game, 2));
        assert_eq!(game.party[2].gear[slot].as_ref().map(|worn| &worn.name), Some(&gear.name));
        assert_eq!(shell.horse(), 2, "the horse chosen is selected");
        let worn = Piece::Worn { horse: 2, slot };
        assert_eq!(shell.hand().open, Some(worn), "the card is on the piece");
        assert!(!shell.is_on(Switch::OtherHorses), "the list closed");
        let said = fact(&game, &uncollected, &shell, &places, Fact::Detail(0));
        assert!(said.contains(&format!("{name} wears it.")), "{said}");

        // From another horse, the list again: the wearer may not take it.
        shell.inspect(0, None);
        shell.press_switch(Switch::OtherHorses);
        assert!(!shell.equip_on(&mut game, 2), "it wears it already");
        assert!(shell.is_on(Switch::OtherHorses));
        assert!(shell.equip_on(&mut game, 1));
        assert!(game.party[1].gear[slot].as_ref().is_some_and(|worn| worn.name == gear.name));
        assert!(game.party[2].gear[slot].as_ref().is_none_or(|worn| worn.name != gear.name));

        shell.press_switch(Switch::OtherHorses);
        shell.show_detail(0, Detail { tab, item: Item::Worn { horse: 0, slot }, opener: None });
        assert!(!shell.is_on(Switch::OtherHorses), "a new card closes it");
    }

    /// The list of things to try (#303) is over the scene in town once a
    /// trail has brought a piece back, and never on the road, where the
    /// fight log has that room. Hints turned off puts it away, and on brings
    /// it back. No entry is a paragraph: a row is one line that fits, with
    /// what it pays at its end, which says so once it has paid.
    #[test]
    fn the_list_is_over_the_town_scene_and_goes_with_the_hints() {
        let places = Places::load();
        let mut shell = Shell::new(places.pages().count());
        assert!(!shell.list_in_view(&Game::new(6)), "before any trail");
        let mut game = (0..50)
            .map(Game::back_from_a_trail)
            .find(|game| game.pending_drop().is_some())
            .expect("no seed under 50 brings a drop home");
        assert!(shell.list_in_view(&game));
        shell.press_switch(Switch::Hints);
        assert!(!shell.list_in_view(&game), "hints are off");
        shell.press_switch(Switch::Hints);
        assert!(shell.list_in_view(&game));

        // A letter of the body size is 8.4 wide. Beside the words are the
        // panel's edge and padding, the box, two gaps, and what it pays.
        let t = Tokens::load();
        let beside = 2.0 * (t.size.border + t.space.m) + t.size.trail_mark + 2.0 * t.space.m;
        let room = ((t.size.to_try_width - beside - t.size.to_try_pays) / 8.4) as usize;
        for (entry, def) in game.lessons.entries().iter().enumerate() {
            assert!(!def.words.contains('\n') && def.words.chars().count() <= room, "{}", def.words);
            let open = try_pays(def, game.lessons.tick(entry));
            let paid = try_pays(def, Tick { listed: true, done: true });
            assert!(open.starts_with(&def.coins.to_string()) && open.contains("coin"), "{open}");
            assert!(paid.starts_with("paid") && paid.contains(&def.coins.to_string()), "{paid}");
            for pays in [open, paid] {
                assert!(pays.chars().count() as f32 * 8.4 <= t.size.to_try_pays, "{pays}");
            }
        }

        game.act(Action::SetOut, Hand::horse(0));
        shell.close();
        assert!(!shell.list_in_view(&game) && shell.log_in_view(&game));
    }

    /// The fight log (#306) is over the scene on the road with every place
    /// closed and nowhere else; its key hides and shows its lines only while
    /// it is there, and is printed on its heading either way. The Journal's
    /// report does not carry the log any more.
    #[test]
    fn the_fight_log_is_over_the_road_and_its_key_hides_it() {
        let places = Places::load();
        let lines = Tokens::load().size.fight_log_lines;
        let mut game = Game::new(6);
        let mut shell = Shell::new(places.pages().count());
        let heading = |shell: &Shell| log_heading(&places, shell.log_hidden());

        assert!(!shell.log_in_view(&game), "not in town");
        assert!(fight_log(&game, lines).is_empty());
        shell.press_log_key(&game);
        assert!(!shell.log_hidden(), "the key does nothing out of sight");

        game.act(Action::SetOut, Hand::horse(0));
        assert!(!shell.log_in_view(&game), "not under an open place");
        shell.close();
        assert!(shell.log_in_view(&game));
        assert!(heading(&shell).contains(&format!("[{}] hides", places.log_key)));
        shell.press_log_key(&game);
        assert!(shell.log_hidden() && shell.log_in_view(&game));
        assert!(heading(&shell).contains(&format!("[{}] shows", places.log_key)));
        shell.press_log_key(&game);
        assert!(!shell.log_hidden());

        // A fight under way: the latest lines, the newest last.
        let data = crate::sim::data::Data::load();
        let Screen::Trail(expedition) = &mut game.screen else {
            panic!("the party has set out");
        };
        while expedition.log.len() <= lines && expedition.outcome.is_none() {
            expedition.tick(&data, 0.1);
        }
        let Screen::Trail(expedition) = &game.screen else {
            unreachable!();
        };
        assert!(expedition.log.len() > lines);
        let latest = fight_log(&game, lines);
        assert_eq!(latest.len(), lines);
        assert_eq!(latest.last().copied(), expedition.log.last().map(String::as_str));
        let side = report_side(&game);
        for line in &expedition.log {
            assert!(!side.contains(line.as_str()), "the report still carries: {line}");
        }

        // The newest line is whole and the oldest of a full log the faintest.
        let fade = 0.3;
        assert_eq!(log_alpha(0, lines, fade), 1.0);
        assert!((log_alpha(lines - 1, lines, fade) - fade).abs() < 1e-6);
        assert!(log_alpha(1, lines, fade) > log_alpha(2, lines, fade));
    }

    /// The scroll keys (#304): a press moves by most of what is in view, so
    /// no line is passed over, enough presses reach the last line, and it
    /// stops at both ends. Words that fit do not move.
    #[test]
    fn the_scroll_keys_reach_every_line_and_stop_at_the_ends() {
        let (room, content, share) = (200.0, 730.0, 0.8);
        let mut at = 0.0;
        let mut presses = 0;
        loop {
            let to = scrolled(at, 1.0, room, content, share);
            if to == at {
                break;
            }
            assert!(to - at < room, "a line was passed over");
            at = to;
            presses += 1;
        }
        assert_eq!(at, content - room);
        assert_eq!(presses, 4);
        while at > 0.0 {
            at = scrolled(at, -1.0, room, content, share);
        }
        assert_eq!(scrolled(at, -1.0, room, content, share), 0.0);
        assert_eq!(scrolled(0.0, 1.0, room, 150.0, share), 0.0);
        let share = Tokens::load().size.scroll_page;
        assert!(share > 0.0 && share < 1.0);
    }

    /// The arrow keys (#229): down a list is the next row and up is the one
    /// before, left and right leave the list for what is beside it, and
    /// where nothing lies that way the focus stays.
    #[test]
    fn an_arrow_moves_to_the_nearest_control_that_way() {
        let mut world = World::new();
        let mut part = |x: f32, y: f32, width: f32| FocusableArea {
            entity: world.spawn_empty().id(),
            position: Vec2::new(x, y),
            size: Vec2::new(width, 30.0),
        };
        // Three rows of a list, a button beside the middle one, and a
        // button under the list. Down the screen is a greater y.
        let rows = [
            part(300.0, 100.0, 300.0),
            part(300.0, 130.0, 300.0),
            part(300.0, 160.0, 300.0),
        ];
        let beside = part(600.0, 130.0, 100.0);
        let under = part(200.0, 220.0, 100.0);
        let mut parts = rows.to_vec();
        parts.extend([beside, under]);
        let go = |from: FocusableArea, key: KeyCode| {
            let (_, way) = ARROWS.iter().find(|(arrow, _)| *arrow == key).unwrap();
            towards(from.entity, *way, &parts)
        };
        assert_eq!(go(rows[0], KeyCode::ArrowDown), Some(rows[1].entity));
        assert_eq!(go(rows[1], KeyCode::ArrowDown), Some(rows[2].entity));
        assert_eq!(go(rows[2], KeyCode::ArrowDown), Some(under.entity));
        assert_eq!(go(rows[2], KeyCode::ArrowUp), Some(rows[1].entity));
        assert_eq!(go(rows[1], KeyCode::ArrowUp), Some(rows[0].entity));
        assert_eq!(go(rows[1], KeyCode::ArrowRight), Some(beside.entity));
        assert_eq!(go(beside, KeyCode::ArrowLeft), Some(rows[1].entity));
        assert_eq!(go(rows[0], KeyCode::ArrowUp), None);
        assert_eq!(go(beside, KeyCode::ArrowRight), None);

        // A focus that is not among them moves nowhere.
        let elsewhere = world.spawn_empty().id();
        assert_eq!(towards(elsewhere, CompassOctant::South, &parts), None);
    }

    /// Everything the text overlay did is done from a control: each ability
    /// (#357), equipping the drop, and setting out, once each. An ability's
    /// control is named as `abilities.ron` names it.
    #[test]
    fn everything_the_slice_can_do_has_its_one_control() {
        let places = Places::load();
        let data = crate::sim::data::Data::load();
        for (at, ability) in data.abilities.iter().enumerate() {
            let control = places.controls().iter().find(|control| control.action == Some(Action::Ability(at)));
            assert_eq!(control.map(|control| control.name.as_str()), Some(ability.name.as_str()));
        }
        for control in places.controls() {
            if let Some(Action::Ability(at)) = control.action {
                assert!(at < data.abilities.len(), "{}", control.name);
            }
        }
        let mut actions: Vec<Action> = (0..data.abilities.len()).map(Action::Ability).collect();
        actions.extend([
            Action::Equip,
            Action::SetOut,
            Action::KeepGoing,
            Action::Undo,
            Action::Earlier,
            Action::Later,
        ]);
        for action in actions {
            let controls = places
                .controls()
                .iter()
                .filter(|control| control.action == Some(action));
            assert_eq!(controls.count(), 1, "{action:?}");
        }
    }

    /// One name for one thing: on any one page, counting the cargo column
    /// that stays beside it, no two controls share a name.
    #[test]
    fn no_two_controls_of_a_page_share_a_name() {
        for place in Places::load().pages() {
            for (heading, _, _, controls) in place.pages() {
                let mut names: Vec<&str> = controls
                    .iter()
                    .chain(&place.cargo)
                    .map(|control| control.name.as_str())
                    .collect();
                let count = names.len();
                names.sort();
                names.dedup();
                assert_eq!(names.len(), count, "{heading}");
            }
        }
    }

    /// Every page has at least one control planned for it. The Inventory's
    /// controls are its cells (#308), and the list of hints' its rows (#300).
    #[test]
    fn every_page_has_its_controls() {
        for place in Places::load().pages() {
            for (heading, _, content, controls) in place.pages() {
                assert!(
                    !controls.is_empty() || matches!(content, Content::Inventory | Content::Hints),
                    "{heading} has no controls"
                );
            }
        }
    }

    /// The Inventory (#308) has every piece the party holds, worn and
    /// carried, each once and under its own type, and follows a piece from
    /// the cargo onto a horse.
    #[test]
    fn the_inventory_has_every_piece_once_under_its_type() {
        let mut game = Game::back_with_one_piece();
        let everything = |game: &Game| {
            let mut all: Vec<Piece> = (0..game.cargo().len()).map(Piece::Carried).collect();
            for horse in 0..game.party.len() {
                all.extend((0..Slot::ALL.len()).map(|slot| Piece::Worn { horse, slot }));
            }
            all.retain(|piece| game.piece(*piece).is_some());
            all
        };
        let check = |game: &Game| {
            let mut shown = Vec::new();
            for slot in 0..Slot::ALL.len() {
                for piece in held(game, slot) {
                    let gear = game.piece(piece).expect("an empty cell is shown");
                    assert_eq!(gear.slot.index(), slot, "{} is under the wrong type", gear.name);
                    assert!(!shown.contains(&piece), "{} is shown twice", gear.name);
                    shown.push(piece);
                }
            }
            let all = everything(game);
            assert_eq!(shown.len(), all.len());
            assert!(all.iter().all(|piece| shown.contains(piece)));
            shown.len()
        };
        // A trail's worth of pieces (#352), each once.
        assert!(check(&Game::back_from_a_trail(6)) > 3);
        let slot = game.pending_drop().unwrap().slot.index();
        assert_eq!(held(&game, slot).last(), Some(&Piece::Carried(0)));
        let before = check(&game);
        let replaced = game.party[0].gear[slot].is_some();
        game.act(Action::Equip, Hand::horse(0));
        assert!(held(&game, slot).contains(&Piece::Worn { horse: 0, slot }));
        assert!(!held(&game, slot).contains(&Piece::Carried(0)));
        assert_eq!(check(&game), before - usize::from(replaced));

        // A type nobody has a piece of is an empty group that says so.
        let mut bare = Game::new(6);
        for horse in &mut bare.party {
            horse.gear[0] = None;
        }
        assert!(held(&bare, 0).is_empty());
        assert!(card::none_held(Slot::ALL[0].name()).contains("head"));

        // It is one place of the bar and a tab of none (the owner,
        // 2026-10-11), with a key the game can read.
        let places = Places::load();
        let own: Vec<_> = places
            .places
            .iter()
            .filter(|place| place.content == Content::Inventory)
            .collect();
        assert_eq!(own.len(), 1);
        assert!(own[0].tabs.is_empty() && !own[0].horse_columns);
        assert!(own[0].key.as_deref().and_then(key_code).is_some());
        assert!(
            places
                .pages()
                .flat_map(|place| &place.tabs)
                .all(|tab| tab.content != Content::Inventory)
        );
    }

    /// A press on a horse (#311) selects it and opens its view where the
    /// player is, on the road too; a pinned piece stays, compared on that
    /// horse; the view has the horse's sheet and a part for every tab of
    /// the horses' place; and a press on a part goes to its tab.
    #[test]
    fn a_press_on_a_horse_opens_everything_about_it() {
        let places = Places::load();
        let game = Game::new(6);
        let uncollected = Uncollected::default();
        let mut shell = Shell::new(places.pages().count());
        shell.inspect(1, None);
        assert_eq!(shell.horse(), 1);
        assert_eq!(shell.detail(0).map(|detail| detail.item), Some(Item::Horse(1)));
        assert_eq!(shell.hand(), Hand::horse(1));
        let said = fact(&game, &uncollected, &shell, &places, Fact::Detail(0));
        let health = game.party[1].stats().health;
        assert!(said.starts_with(&format!("{}\nHealth {health}/{health}\n", game.party[1].name)), "{said}");
        let horses = &places.places[0];
        assert!(horses.horse_columns);
        for (tab, page) in horses.tabs.iter().enumerate() {
            let part = fact(&game, &uncollected, &shell, &places, Fact::HorsePart(0, tab));
            assert!(!part.is_empty(), "{} has no part in the view", page.name);
            assert_eq!(part.contains("Head: nothing worn"), page.content == Content::Gear);
            assert_eq!(part.starts_with("Not built yet."), page.content == Content::Planned);
        }
        assert_eq!(
            fact(&game, &uncollected, &shell, &places, Fact::HorseTrail(0)),
            "No trail walked yet."
        );
        // A part is of the view that is open, and of nothing else.
        assert_eq!(fact(&game, &uncollected, &shell, &places, Fact::HorseTrail(2)), "");
        // Back from a trail, it is what that horse did on it.
        let back = Game::back_from_a_trail(6);
        let deeds = back.last_report().unwrap().outcome.deeds[1];
        let said = fact(&back, &uncollected, &shell, &places, Fact::HorseTrail(0));
        assert!(said.starts_with("Trail 1, won\n"), "{said}");
        assert!(said.contains(&format!("Dealt {}, ", deeds.dealt)), "{said}");
        assert!(said.contains(&format!("Took {}", deeds.taken)), "{said}");
        assert!(said.lines().all(|line| line.len() <= 25), "{said}");
        assert_eq!(fact(&game, &uncollected, &shell, &places, Fact::HorsePart(2, 0)), "");

        // A press on a part goes to its tab, from another place too; the
        // view is put away where the part is changed, and stays where the
        // player came from.
        shell.open(2);
        shell.inspect(1, None);
        shell.go_to(0, 1);
        assert!(shell.is_open() && shell.place() == 0 && shell.tab(0) == 1);
        assert_eq!(shell.detail(0), None);
        assert_eq!(shell.detail(2).map(|detail| detail.item), Some(Item::Horse(1)));
        shell.inspect(1, None);
        shell.go_to(0, 0);
        assert_eq!(shell.detail(0), None);
        shell.go_to(0, 1);
        shell.inspect(1, None);
        shell.go_to(0, 0);
        shell.choose_tab(0, 1);
        assert_eq!(shell.detail(0), None, "the view comes back with its tab");
        assert_eq!(shell.horse(), 1);
        shell.choose_tab(0, 0);
        shell.drop_detail(2);

        // From the road with every place closed, it opens the place.
        shell.close();
        shell.open(2);
        shell.close();
        shell.inspect(2, None);
        assert!(shell.is_open() && shell.place() == 2);
        assert_eq!(shell.detail(2).map(|detail| detail.item), Some(Item::Horse(2)));

        // A pinned piece is the handle: it stays, and the horse changes.
        let pinned = Detail { tab: 0, item: Item::Carried(0), opener: None };
        shell.open(0);
        shell.show_detail(0, pinned);
        shell.inspect(0, None);
        assert_eq!(shell.detail(0), Some(pinned));
        assert_eq!(shell.horse(), 0);
    }

    /// The owner's five places (epic #225), in her order, and the Inventory
    /// she gave a place of its own on 2026-10-11, beside the horses.
    #[test]
    fn the_bar_is_the_places_in_order() {
        let places = Places::load();
        let names: Vec<&str> = places.places.iter().map(|p| p.name.as_str()).collect();
        assert_eq!(names, ["Horses", "Inventory", "Trade", "Map", "Town", "Journal"]);
    }

    /// One name for one thing: no two tabs of a place share a name, and
    /// every page says what is decided there.
    #[test]
    fn every_page_has_its_own_name_and_a_job() {
        for place in Places::load().places {
            let pages = place.pages();
            for (i, (heading, job, _, _)) in pages.iter().enumerate() {
                assert!(!job.is_empty(), "{heading} has no job");
                assert!(
                    pages[i + 1..]
                        .iter()
                        .all(|(other, _, _, _)| other != heading),
                    "two pages are called {heading}"
                );
            }
        }
    }

    /// A report shows the piece its trail dropped where the piece is now
    /// (#231): in the cargo as the cargo shows it, and then on its horse.
    #[test]
    fn a_report_shows_its_piece_where_it_is() {
        let places = Places::load();
        let uncollected = Uncollected::default();
        let shell = Shell::new(places.pages().count());
        let mut game = (0..50)
            .map(Game::back_from_a_trail)
            .find(|game| game.pending_drop().is_some())
            .expect("no seed under 50 brings a drop home");
        let name = game.pending_drop().unwrap().name.clone();
        let say = |game: &Game, what| fact(game, &uncollected, &shell, &places, what);
        assert_eq!(say(&game, Fact::ReportedName), name);
        assert_eq!(say(&game, Fact::ReportedKind), say(&game, Fact::CarriedKind));
        assert_eq!(say(&game, Fact::ReportedWhere), card::IN_CARGO);

        game.act(Action::Equip, Hand::horse(2));
        assert_eq!(say(&game, Fact::ReportedName), name);
        let wearer = &game.party[2].name;
        assert_eq!(say(&game, Fact::ReportedWhere), format!("{wearer} wears it."));

        // On the road the report is the fight's and has no piece, until an
        // earlier one is opened.
        game.act(Action::SetOut, Hand::horse(0));
        assert_eq!(say(&game, Fact::ReportedName), "");
        game.act(Action::Earlier, Hand::default());
        assert_eq!(say(&game, Fact::ReportedName), name);
        assert!(say(&game, Fact::Report).starts_with("EARLIER REPORT 1 of 1"));
        assert!(say(&game, Fact::ReportSide).starts_with("REPORTS KEPT"));
    }

    /// The marks of the map (#232) are on places that show their thing: new
    /// gear where the cargo is, a report on the tab that shows reports,
    /// and a rarity is one `gear.ron` has.
    #[test]
    fn every_mark_is_on_a_place_that_shows_its_thing() {
        let places = Places::load();
        let rarities = crate::sim::data::Data::load().gear.rarities;
        assert!(!places.marks.is_empty());
        for rule in &places.marks {
            let Some((place, tab)) = places.mark_at(rule) else {
                panic!("{rule:?} is on no place of the map");
            };
            let page = &places.places[place];
            match &rule.raise {
                Raise::NewGear => assert!(page.horse_columns && tab.is_none(), "{rule:?}"),
                Raise::Report { rarity, .. } => {
                    let content = tab.map(|tab| page.tabs[tab].content);
                    assert_eq!(content, Some(Content::Report), "{rule:?}");
                    if let Some(name) = rarity {
                        assert!(rarities.iter().any(|tier| tier.name == *name), "{rule:?}");
                    }
                }
            }
        }
    }

    /// A mark (#232) leads to its thing in at most two actions, seeing the
    /// thing clears it, and it does not come back for the same thing.
    #[test]
    fn a_mark_leads_to_its_thing_and_seeing_it_clears_it() {
        let places = Places::load();
        let pages = places.pages().count();
        let at = |name: &str| places.places.iter().position(|p| p.name == name).unwrap();
        let (horses, trade, journal) = (at("Horses"), at("Trade"), at("Journal"));
        let marked = |game: &Game| -> Vec<usize> {
            (0..pages).filter(|&p| places.wants(game, p, None)).collect()
        };
        let quiet = Raise::Report { lost: false, rarity: None };

        // Home with an upgrade and a report of no note, the player in Trade.
        let home = || {
            (0..50)
                .map(Game::back_from_a_trail)
                .find(|game| game.pending_drop().is_some() && !places.wants(game, journal, None))
                .expect("no seed under 50 brings home a drop and a plain report")
        };
        let mut game = home();
        assert!(!game.wants(&quiet), "a rule that names nothing raises nothing");
        let mut shell = Shell::new(pages);
        shell.open(trade);
        look(&shell, &places, &mut game);
        assert_eq!(marked(&game), [horses]);
        assert!(!places.wants(&game, horses, Some(0)), "the cargo is on no tab");
        // A slot's card stands where the cargo is: the place opened onto
        // it shows nothing of the cargo. Closing it does: two actions.
        shell.show_detail(
            horses,
            Detail {
                tab: 0,
                item: Item::Worn { horse: 0, slot: 0 },
                opener: None,
            },
        );
        shell.open(horses);
        look(&shell, &places, &mut game);
        assert_eq!(marked(&game), [horses]);
        shell.close_detail();
        look(&shell, &places, &mut game);
        assert!(marked(&game).is_empty());
        shell.open(trade);
        look(&shell, &places, &mut game);
        assert!(marked(&game).is_empty(), "it does not come back");

        // The report that shows the drop shows it too.
        let mut game = home();
        shell.open(journal);
        shell.choose_tab(journal, 0);
        look(&shell, &places, &mut game);
        assert!(marked(&game).is_empty());

        // Home from a trail the map's rules call a highlight.
        let seed = (0..200)
            .find(|&seed| places.wants(&Game::back_from_a_rolled_trail(seed), journal, None))
            .expect("no seed under 200 brings home a report worth a mark");
        let mut game = Game::back_from_a_rolled_trail(seed);
        shell.open(trade);
        look(&shell, &places, &mut game);
        assert!(marked(&game).contains(&journal));
        assert!(places.wants(&game, journal, Some(0)));
        assert!(!places.wants(&game, journal, Some(1)), "only the tab it is on");
        // In town the place opens on it: one action, or two from another tab.
        shell.choose_tab(journal, 1);
        shell.open(journal);
        look(&shell, &places, &mut game);
        assert!(places.wants(&game, journal, Some(0)));
        shell.choose_tab(journal, 0);
        look(&shell, &places, &mut game);
        assert!(!marked(&game).contains(&journal));

        // Not read in town, it is still wanted on the road, where the place
        // opens on the fight. Earlier, by its key from anywhere, is on it.
        let mut game = Game::back_from_a_rolled_trail(seed);
        shell.open(trade);
        game.act(Action::SetOut, Hand::horse(0));
        shell.close();
        look(&shell, &places, &mut game);
        assert!(marked(&game).contains(&journal));
        shell.open(journal);
        look(&shell, &places, &mut game);
        assert!(marked(&game).contains(&journal), "the fight is not the report");
        game.act(Action::Earlier, Hand::default());
        look(&shell, &places, &mut game);
        assert!(marked(&game).is_empty());
    }

    #[test]
    fn town_opens_on_the_first_place() {
        let shell = Shell::new(5);
        assert_eq!((shell.place(), shell.tab(0), shell.horse()), (0, 0, 0));
    }

    /// Leaving a place and returning shows it as it was left.
    #[test]
    fn a_place_is_found_as_it_was_left() {
        let mut shell = Shell::new(5);
        shell.choose_tab(0, 2);
        shell.open(4);
        shell.choose_tab(4, 1);
        shell.open(0);
        assert_eq!(shell.tab(0), 2);
        shell.open(4);
        assert_eq!(shell.tab(4), 1);
    }

    /// A detail belongs to the tab it was opened from: it is out of the way
    /// on another tab and back on return, until it is closed.
    #[test]
    fn a_detail_stays_with_its_tab() {
        let mut shell = Shell::new(5);
        let detail = Detail {
            tab: 0,
            item: Item::Worn { horse: 0, slot: 1 },
            opener: None,
        };
        shell.show_detail(0, detail);
        assert_eq!(shell.detail(0), Some(detail));
        shell.choose_tab(0, 1);
        assert_eq!(shell.detail(0), None);
        shell.open(3);
        shell.open(0);
        shell.choose_tab(0, 0);
        assert_eq!(shell.detail(0), Some(detail));
        shell.close_detail();
        assert_eq!(shell.detail(0), None);
    }

    /// Selecting a horse in one place selects it in all of them.
    /// Gear is the selected horse's gear and no other horse's (#314), and
    /// another horse's is one press away. A piece pinned there stays, to be
    /// compared on the horse pressed.
    #[test]
    fn gear_shows_the_selected_horse_and_follows_a_press() {
        let mut shell = Shell::new(6);
        let shown = |shell: &Shell| (0..3).filter(|&horse| shell.wears(horse)).collect::<Vec<_>>();
        assert_eq!(shown(&shell), [0]);
        shell.inspect(2, None);
        assert_eq!(shown(&shell), [2]);
        shell.close_detail();
        let worn = Item::Worn { horse: 2, slot: 0 };
        shell.show_detail(0, Detail { tab: 0, item: worn, opener: None });
        shell.inspect(1, None);
        assert_eq!(shown(&shell), [1]);
        assert_eq!(shell.detail(0).map(|detail| detail.item), Some(worn));
    }

    #[test]
    fn one_horse_is_selected_everywhere() {
        let mut shell = Shell::new(5);
        shell.inspect(2, None);
        for place in 0..5 {
            shell.open(place);
            assert_eq!(shell.horse(), 2);
        }
    }

    /// On the road a press on the open place puts it away and gives the
    /// scene its room; any press on the bar brings a place back. In town a
    /// place is always open.
    #[test]
    fn on_the_road_a_place_closes_and_opens_again() {
        let mut shell = Shell::new(5);
        shell.press(0, true);
        assert!(shell.is_open(), "a press in town never closes the place");
        shell.press(0, false);
        assert!(!shell.is_open());
        shell.press(0, false);
        assert!(shell.is_open());
        shell.press(3, false);
        assert!(shell.is_open());
        assert_eq!(shell.place(), 3);
    }

    /// A place put away on the road is found as it was left.
    #[test]
    fn a_closed_place_keeps_its_tab_and_detail() {
        let mut shell = Shell::new(5);
        let detail = Detail {
            tab: 1,
            item: Item::Worn { horse: 0, slot: 2 },
            opener: None,
        };
        shell.choose_tab(0, 1);
        shell.show_detail(0, detail);
        shell.close();
        shell.open(0);
        assert_eq!(shell.tab(0), 1);
        assert_eq!(shell.detail(0), Some(detail));
    }

    /// A press on the scene (#307) puts the place away on the road, side
    /// panel and all, and it is found as it was left; in town, and with
    /// every place closed, it changes nothing but the mark (#366).
    #[test]
    fn a_press_on_the_scene_closes_the_place_on_the_road_only() {
        let mut shell = Shell::new(5);
        let detail = Detail {
            tab: 0,
            item: Item::Worn { horse: 0, slot: 2 },
            opener: None,
        };
        shell.open(1);
        shell.show_detail(1, detail);
        shell.marked = false;
        let in_town = shell.clone();
        shell.press_scene(true);
        assert_eq!(shell, in_town, "nothing in town");
        shell.press_scene(false);
        assert!(!shell.is_open());
        let closed = shell.clone();
        shell.press_scene(false);
        assert_eq!(shell, closed, "nothing with every place closed");
        shell.open(1);
        assert_eq!(shell, in_town, "as it was left");
    }

    /// A press on the background unselects the horse, in town and on the
    /// road (#366): no horse is marked. The menus keep the last one, and a
    /// press on a horse marks it again.
    #[test]
    fn a_press_on_the_background_unselects_the_horse() {
        for in_town in [true, false] {
            let mut shell = Shell::new(5);
            shell.inspect(2, None);
            assert_eq!(shell.marked(), Some(2));
            shell.press_scene(in_town);
            assert_eq!(shell.marked(), None, "in town: {in_town}");
            assert_eq!(shell.horse(), 2, "the menus keep the last horse");
            shell.inspect(1, None);
            assert_eq!(shell.marked(), Some(1));
        }
    }

    /// The pointer shows the card of a piece (#228), and not of an empty
    /// slot, nor of the piece whose card is pinned beside it already.
    #[test]
    fn the_pointer_shows_a_piece_that_is_not_pinned() {
        let game = (0..50)
            .map(Game::back_from_a_trail)
            .find(|game| game.pending_drop().is_some())
            .expect("no seed under 50 brings a drop home");
        let mut shell = Shell::new(5);
        let empty = (0..Slot::ALL.len())
            .find(|slot| game.party[0].gear[*slot].is_none())
            .unwrap();
        assert_eq!(shell.points_at(&game, Piece::Carried(0)), Some(Piece::Carried(0)));
        assert_eq!(shell.points_at(&game, Piece::Worn { horse: 0, slot: empty }), None);
        shell.show_detail(
            0,
            Detail {
                tab: 0,
                item: Item::Carried(0),
                opener: None,
            },
        );
        assert_eq!(shell.points_at(&game, Piece::Carried(0)), None);
        // Pinned in a place that is not showing, it is not beside it.
        shell.open(1);
        assert_eq!(shell.points_at(&game, Piece::Carried(0)), Some(Piece::Carried(0)));
    }

    /// The hover card is beside its piece, never over it, and on the side
    /// of the window with more room, at the smallest window too.
    #[test]
    fn the_hover_card_is_beside_its_piece_and_in_the_window() {
        let t = Tokens::load();
        let window = Vec2::new(t.layout.min_width, t.layout.min_height);
        let width = t.size.compare_width;
        let size = Vec2::new(t.size.cargo_width, t.size.cell_height);
        for x in [0.0, 300.0, window.x / 2.0, 700.0, window.x - size.x] {
            for y in [0.0, 200.0, window.y / 2.0, window.y - size.y] {
                let cell = Rect::from_corners(Vec2::new(x, y), Vec2::new(x, y) + size);
                let (left, right) = across(beside(cell, window, t.space.s, width), window, width);
                assert!(left >= 0.0 && right <= window.x, "{cell:?}: {left} to {right}");
                assert!(left >= cell.max.x || right <= cell.min.x, "{cell:?}: over its piece");
                let at = beside(cell, window, t.space.s, width);
                match (at.top, at.bottom) {
                    (Val::Px(top), Val::Auto) => assert_eq!(top, cell.min.y),
                    (Val::Auto, Val::Px(bottom)) => assert_eq!(window.y - bottom, cell.max.y),
                    other => panic!("{other:?}"),
                }
            }
        }
        // A slot of the Gear tab is most of the window wide: the card stays
        // in the window, over the slot's far end (#228).
        let wide = Rect::from_corners(Vec2::new(16.0, 300.0), Vec2::new(window.x - 300.0, 340.0));
        let (left, right) = across(beside(wide, window, t.space.s, width), window, width);
        assert!(left >= 0.0 && right <= window.x, "{left} to {right}");
    }

    /// Where a card placed by `beside` is across the window, left to right.
    fn across(at: UiRect, window: Vec2, width: f32) -> (f32, f32) {
        match (at.left, at.right) {
            (Val::Px(left), Val::Auto) => (left, left + width),
            (Val::Auto, Val::Px(right)) => (window.x - right - width, window.x - right),
            other => panic!("{other:?}"),
        }
    }

    #[test]
    fn a_place_that_does_not_exist_cannot_be_opened() {
        let mut shell = Shell::new(5);
        shell.open(9);
        assert_eq!(shell.place(), 0);
    }
}
