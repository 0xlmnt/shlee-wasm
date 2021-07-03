use wasm_bindgen::prelude::*;
use lib_simulation as sim;
use rand::prelude::*;
use serde::Serialize;


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

    pub fn world(&self) -> JsValue {
        let world = World::from(self.sim.world());
        JsValue::from_serde(&world).unwrap()
    }

    pub fn step(&mut self) {
        self.sim.step();
    }
}


#[derive(Clone, Debug, Serialize)]
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


#[derive(Debug, Clone, Serialize)]
pub struct Animal {
    pub x: f32,
    pub y: f32,
    pub rotation: f32

}

impl From<&sim::Animal> for Animal {
    fn from(animal: &sim::Animal) -> Self {
        Self {
            x: animal.position().x,
            y: animal.position().y,
            rotation: animal.rotation().angle()
        }
    }
}