use rand::Rng;
use rand::rngs::ThreadRng;
use crate::star::Star;

pub struct Manager {
    stars: Vec<Star>,
    rng: ThreadRng
}

const SPEED: f64 = 1.0;

impl Manager {
    pub fn new() -> Self {
        Self {
            stars: Vec::new(),
            rng: rand::thread_rng()
        }
    }

    pub fn gen_new_star(&mut self, estimate: f64) {
        let case: f64 = self.rng.gen::<f64>();
        let count = (case/(estimate*10.0)) as i32;
        for i in 0..count {
            let star = Star::new(
                self.rng.gen_range(-1.0..=1.0),
                self.rng.gen_range(-1.0..=1.0),
                1.0,
                self.rng.gen_range(0.1..1.0));
            self.stars.push(star);
        }
    }

    pub fn tick(&mut self, estimate: f64) {
        self.stars.iter_mut().for_each(|mut s| s.add_z(-SPEED * estimate));
        self.stars.retain(|s| {s.z() >= 0.0});
        self.gen_new_star(estimate);
    }

    pub fn stars(&self) -> &Vec<Star> {
        &self.stars
    }
}