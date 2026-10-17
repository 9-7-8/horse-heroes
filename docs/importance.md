# Importance

One scale for how important a thing that happens is, shared by every effect
(#39; [juice.md](philosophy/juice.md), "Juice the decision, not every
number"). There are three levels. Each sets what an effect at it may spend,
and no effect sets its own strength: it names its level and reads the
level's numbers. Changing a level changes every effect at it.

The numbers and the mapping are in `assets/data/importance.ron`, read by
`fx::importance` into the `Importance` resource.

## The levels

| level | for | `shake` | `flash` | `shake_secs` | `flash_secs` | `sound` |
|---|---|---|---|---|---|---|
| Low | routine gains | 0 | 0 | 0.25 | 0.2 | 0.7 |
| Medium | decisions and thresholds | 0.14 | 0.03 | 0.3 | 0.25 | 0.85 |
| High | rare, game-changing, first-time | 0.3 | 0.07 | 0.4 | 0.35 | 1.0 |

`shake` is how far the view is thrown, in world units; `flash` how solid the
flash over the scene is at its start; `sound` how loud a sound is played
against the way its recipe in `sounds.ron` was written. The settings' scales
turn every level down together: `shake_scale`, `flash_scale` and `volume` in
`loot_shower.ron`.

A higher level never spends less of anything than a lower one, and is
always louder (`the_levels_climb`).

## Which event sits where

| event | level | read by |
|---|---|---|
| a burst of coins alone | Low (`coins`) | the screen response |
| a burst whose best piece is Normal or Magic | Low | the screen response |
| a burst whose best piece is Rare | Medium | the screen response |
| a burst whose best piece is Legendary or Set | High | the screen response |
| a piece of gear thrown | its rarity's, as above | its drop sound |
| an ordinary kill (fountain `small`, death `small`) | Low | its launch and breaking sounds |
| a trail's last enemy (fountain `full`, death `big`) | High | its launch and breaking sounds |
| a piece touching the ground | Low (`landing`) | the landing sound, then quieter for each bounce |
| a piece reaching the party | Low (`pickup`) | the pickup sound |
| a horse's hit on an enemy | Low (`hit`) | the hit sound |
| an ability being used | Medium (`ability`) | the ability's sound |

Every rarity, fountain and death in the game has a level, a better rarity is
never at a lower one, a trail's last enemy is never below an ordinary one,
and nothing routine is High (`everything_has_its_level`).

## What it does not cover

- A body's own flash and recoil as it is struck (`blows` in `attacks.ron`,
  [hit-reactions.md](hit-reactions.md)) say how hard it was hit, not how
  much it matters, so they stay with the hit.
- Nothing pauses for importance yet. The lesson pop-up's wait (#353) and the
  final enemy's approach ([boss-announce.md](boss-announce.md)) are their
  own rules; a level that pauses other things waits for an event that needs
  it.
- Camp and town have no effects of their own yet; when they do, each names
  a level here.

All of the numbers are first guesses; nobody has tuned them by eye or ear.
Against what each effect had before (2026-10-16): Rare and Set shake and
flash as they did, Legendary as hard as Set (was 0.26 and 0.06), and
Normal and Magic not at all (were 0.03 and 0.06); every response lasts a
little longer at High and shorter at Medium (was 0.35 and 0.3 for all);
hits play as they did (0.7), and landings, pickups and an ordinary kill's
sounds are as quiet as hits now, abilities a little quieter than before.
