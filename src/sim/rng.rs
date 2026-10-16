//! Small seeded RNG (SplitMix64). The same seed always gives the same rolls.

#[derive(Clone, Debug)]
pub struct Rng(u64);

impl Rng {
    pub fn new(seed: u64) -> Self {
        Self(seed)
    }

    pub fn next_u64(&mut self) -> u64 {
        self.0 = self.0.wrapping_add(0x9E37_79B9_7F4A_7C15);
        let mut z = self.0;
        z = (z ^ (z >> 30)).wrapping_mul(0xBF58_476D_1CE4_E5B9);
        z = (z ^ (z >> 27)).wrapping_mul(0x94D0_49BB_1331_11EB);
        z ^ (z >> 31)
    }

    /// Uniform in `0..n`. `n` must be above zero.
    pub fn below(&mut self, n: u32) -> u32 {
        (((self.next_u64() >> 32) * n as u64) >> 32) as u32
    }

    /// Uniform in `lo..=hi`.
    pub fn range(&mut self, lo: i32, hi: i32) -> i32 {
        lo + self.below((hi - lo + 1) as u32) as i32
    }

    /// Uniform in `0.0..1.0`.
    pub fn unit(&mut self) -> f32 {
        (self.next_u64() >> 40) as f32 / (1u64 << 24) as f32
    }

    pub fn chance(&mut self, p: f32) -> bool {
        self.unit() < p
    }

    pub fn pick<'a, T>(&mut self, items: &'a [T]) -> &'a T {
        &items[self.below(items.len() as u32) as usize]
    }

    /// Index into `weights`, chosen in proportion to each weight.
    pub fn weighted(&mut self, weights: &[u32]) -> usize {
        let mut roll = self.below(weights.iter().sum());
        for (i, &w) in weights.iter().enumerate() {
            if roll < w {
                return i;
            }
            roll -= w;
        }
        weights.len() - 1
    }

    pub fn shuffle<T>(&mut self, items: &mut [T]) {
        for i in (1..items.len()).rev() {
            items.swap(i, self.below(i as u32 + 1) as usize);
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn same_seed_same_rolls() {
        let (mut a, mut b, mut c) = (Rng::new(7), Rng::new(7), Rng::new(8));
        let rolls = |rng: &mut Rng| (0..32).map(|_| rng.next_u64()).collect::<Vec<_>>();
        let first = rolls(&mut a);
        assert_eq!(first, rolls(&mut b));
        assert_ne!(first, rolls(&mut c));
    }

    #[test]
    fn rolls_stay_in_range() {
        let mut rng = Rng::new(1);
        let mut seen = [false; 6];
        for _ in 0..10_000 {
            assert!(rng.below(1) == 0);
            assert!(rng.below(7) < 7);
            let unit = rng.unit();
            assert!((0.0..1.0).contains(&unit));
            let roll = rng.range(-2, 3);
            assert!((-2..=3).contains(&roll));
            seen[(roll + 2) as usize] = true;
        }
        // Both ends of an inclusive range come up.
        assert!(seen.iter().all(|&s| s));
        assert_eq!(rng.range(4, 4), 4);
    }

    #[test]
    fn chance_follows_its_probability() {
        let mut rng = Rng::new(2);
        assert!(!(0..1000).any(|_| rng.chance(0.0)));
        assert!((0..1000).all(|_| rng.chance(1.0)));
        let hits = (0..100_000).filter(|_| rng.chance(0.25)).count();
        assert!((24_000..26_000).contains(&hits), "{hits}");
    }

    #[test]
    fn weighted_follows_the_weights() {
        let mut rng = Rng::new(3);
        let weights = [70, 25, 5, 0];
        let mut counts = [0u32; 4];
        for _ in 0..100_000 {
            counts[rng.weighted(&weights)] += 1;
        }
        assert!((69_000..71_000).contains(&counts[0]), "{counts:?}");
        assert!((24_000..26_000).contains(&counts[1]), "{counts:?}");
        assert!((4_500..5_500).contains(&counts[2]), "{counts:?}");
        assert_eq!(counts[3], 0);
    }

    #[test]
    fn pick_reaches_every_item_evenly() {
        let mut rng = Rng::new(5);
        let items = ['a', 'b', 'c', 'd'];
        let mut counts = [0u32; 4];
        for _ in 0..100_000 {
            let picked = rng.pick(&items);
            counts[items.iter().position(|item| item == picked).unwrap()] += 1;
        }
        assert!(counts.iter().all(|&n| (24_000..26_000).contains(&n)), "{counts:?}");
        assert_eq!(rng.pick(&['x']), &'x');
    }

    #[test]
    fn shuffle_keeps_every_item_and_reaches_every_order() {
        let mut rng = Rng::new(4);
        let mut orders = std::collections::HashSet::new();
        for _ in 0..1000 {
            let mut items = [0, 1, 2];
            rng.shuffle(&mut items);
            let mut sorted = items;
            sorted.sort();
            assert_eq!(sorted, [0, 1, 2]);
            orders.insert(items);
        }
        assert_eq!(orders.len(), 6);
    }
}