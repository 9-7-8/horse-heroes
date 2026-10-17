# Hit reactions

Whatever is struck, enemy or horse, reacts on the frame the hit lands
(#16; `docs/philosophy/combat-rhythm.md`, the impact phase). Four things
happen at once, besides the damage number ([damage-numbers.md](damage-numbers.md)):

- **Flash.** The body's colour goes toward white and fades back. Each enemy
  place and each horse has materials of its own (`ElementColours`,
  `HorseShades` in `game::scene`), so only the body struck flashes. A
  horse's body, hooves and hair flash; worn gear does not.
- **Recoil.** The flinch of [horse-model.md](horse-model.md) (`flinch` in
  `attacks.ron`) is multiplied by the kind's `recoil`. A weaker blow landing
  while a stronger one's flinch is still running does not cut it short.
- **Sparks.** Small unlit cubes spray up off the struck face, back toward
  what struck it, fall and shrink away (`fx::impact`). They are the hit
  number's colour: the element's, the physical one, or the colour of a hit
  taken; a resisted hit's are pulled toward grey.
- **Health bar.** A bar over every enemy and every horse on a trail drops to
  the new health at once. A light trailing piece holds where the health was
  for `hold_secs`, then falls to it at `drain` of a bar a second. A new enemy
  in a place starts a full bar. Bars are hidden in town, over an empty
  place, and over enemies on the way back to town, as the scene hides those
  bodies; they sit under the menus.

## Strong, plain, resisted

The kind of hit (`HitKind`) decides the size of every reaction, so the three
can be told apart without reading a number: Strong flashes hardest and
longest, recoils furthest and throws the most sparks, fastest and biggest;
Resisted the least. A hit softened by the Ward on a horse (`Warded`) lands
as a resisted one. Tests hold that order for every number
(`a_stronger_blow_is_a_bigger_reaction`, `a_stronger_hit_throws_more_sparks`).

## Numbers

- `assets/data/attacks.ron`, `blows`: `recoil`, `flash` and `flash_secs`
  for each kind.
- `assets/data/impacts.ron`: sparks for each kind (`count`, `speed`,
  `size`), how they fly and how long they last, and the bars' size, place,
  hold, drain and colours.

## Seen

`--seed 6`: at 5.27 Morrow's 8 has just landed on the Mire Toad,
its sparks off the toad's left face and its bar's trailing piece showing;
at 5.75 the Gloom Sprite's 3 on Morrow, whose bar trails; at 28.05 a Strong
Frost Burst on a Thornling. Pictures: `docs/gallery/2026-10-16-hit-reactions`.
