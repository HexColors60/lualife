use bevy::prelude::*;
use rand::rngs::StdRng;
use rand::{Rng, SeedableRng};

#[derive(Resource, Debug)]
pub struct GameRng {
    rng: StdRng,
    seed: u64,
}

impl Default for GameRng {
    fn default() -> Self {
        Self::new(0)
    }
}

impl GameRng {
    pub fn new(seed: u64) -> Self {
        let seed = if seed == 0 {
            rand::random()
        } else {
            seed
        };
        Self {
            rng: StdRng::seed_from_u64(seed),
            seed,
        }
    }

    pub fn seed(&self) -> u64 {
        self.seed
    }

    pub fn gen_range<T, R>(&mut self, range: R) -> T
    where
        R: rand::distributions::uniform::SampleRange<T>,
        T: rand::distributions::uniform::SampleUniform + PartialOrd + Copy,
    {
        self.rng.gen_range(range)
    }

    pub fn gen_bool(&mut self, probability: f64) -> bool {
        self.rng.gen_bool(probability)
    }

    pub fn gen<T>(&mut self) -> T
    where
        rand::distributions::Standard: rand::distributions::Distribution<T>,
    {
        self.rng.gen()
    }

    pub fn shuffle<T>(&mut self, slice: &mut [T]) {
        use rand::seq::SliceRandom;
        slice.shuffle(&mut self.rng);
    }

    pub fn choose<'a, T>(&mut self, slice: &'a [T]) -> Option<&'a T> {
        use rand::seq::SliceRandom;
        slice.choose(&mut self.rng)
    }
}