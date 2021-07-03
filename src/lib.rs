use wasm_bindgen::prelude::*;
use lib_simulation as sim;
use rand::prelude::*;


#[wasm_bindgen]
pub struct Simulation {
    rng: ThreadRng,
    sim: sim::Simulation,
}

#[wasm_bindgen]
impl Simulation {
    #[wasm_bindgen(constructor)]
    pub fn new() -> Self {
        let mut rng = thread_rng();
        let mut sim = sim::Simulation::random(&mut rng);
        Self {
            rng,
            sim,
        }
    }

    pub fn world(&self) -> World {
        World::from(self.sim.world())
    }
}

#[wasm_bindgen]
#[derive(Clone, Debug)]
pub struct World {
    pub animals: Vec<Animal>,
}

impl From<&sim::World> for World {
    fn from(world: &sim::World) -> Self {
        Self {
            animals: world.animals()
                .iter()
                .map(Animal::from)
                .collect()
        }
    }
}

pub struct Animal {
    pub x: f32,
    pub y: f32,
}

impl From<&sim::Animal> for Animal {
    fn from(animal: &sim::Animal) -> Self {
        Self {
            x: animal.position().x,
            y: animal.position().y,
        }
    }
}