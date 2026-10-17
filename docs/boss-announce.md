# The final enemy, announced

A trail's last encounter is its final enemy (`finals` in `encounters.ron`).
It is marked as it comes into view, so the player sees the payoff coming
before it happens (#19; `docs/philosophy/loot-fountain.md`, "Anticipation").
Its body was already the biggest on the trail (`scene::enemy_size` grows with
health); this adds an approach, a name and its own bar.

## The approach (sim)

When the final encounter opens, `Expedition::approach` is set to
`final_approach` from `rules.ron` (1.6 s). While it is above zero, `fight`
spends the step on it and does nothing else: no hit, no ability, no enemy
blow, and the Ward's seconds do not run. Every timer resumes where it was,
so a fight plays exactly as it would without the approach, later by that
much (the seed 6 times in `procedures/build-and-run.txt` moved by 1.6 s).
`swing_in` and `cast_in` add the approach's steps to the timer's, so the
wind-ups of `fx::tells` and the horses' leans still start on time.

The boss's lesson (#360) comes up as the fight opens, so with the game
waiting on it, the enemy stands partway up the trail until Got it.

Test: `the_final_enemy_comes_up_before_the_first_blow` (expedition.rs). The
`fighting` test helper starts its fight with no approach, as those tests are
of the fight itself.

## What is drawn (`fx::boss`, numbers in `boss.ron`)

- **The walk.** `Announce::approach(left)` gives how far back up the trail
  the body is and how far it leans: it starts `walk` back and slows into its
  place by `rear_at` of the way, then rears back by up to `rear` and settles
  by the first blow. `game::scene` draws the body there, and the boss's box
  for its lesson (`place_boss` in `menu::shell`) follows it.
  `Game::approach_left` is the share of the approach left.
- **The banner.** At the top of the window, centred, or just right of the
  fight log when the window is too narrow for both: the enemy's name, its
  element in the element's colour, and a wide bar. During the approach the
  name fades in and the bar fills; through the fight the bar drops at once
  with a trailing piece that holds and drains as the small bars do
  (`impact::trail_after`, the numbers in `impacts.ron`). After the kill, on
  a won trail, the emptied bar stays `linger` seconds while the words fade.
  On a lost trail it goes at once. It sits under the menus.
- **No small bar.** The bar over the final enemy's head is not drawn while
  the banner shows it (`fx::impact`).

Test: `the_final_enemy_walks_into_its_place` (boss.rs).

Pictures: `docs/gallery/2026-10-16-boss-announce`.
