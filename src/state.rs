use crate::config;
use rand::Rng;
use std::collections;

#[derive(Clone, Copy, Debug, Hash, Eq, Ord, PartialEq, PartialOrd)]
#[repr(transparent)]
pub struct SessionId(pub uuid::Uuid);

#[derive(Clone, Copy, Debug, Hash, Eq, Ord, PartialEq, PartialOrd)]
#[repr(transparent)]
pub struct Secret(pub uuid::Uuid);

#[derive(Clone, Debug, Default)]
pub struct State {
    ships: collections::HashMap<SessionId, Ship>,
}

#[derive(Clone, Debug)]
pub struct Ship {
    pub name: String,
    pub secret: Secret,
    pub position: (f64, f64),
    pub velocity: (f64, f64),
    pub direction: f64,
    pub acceleration: f64,
    pub energy: f64,
}

impl State {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn join(&mut self, name: String, secret: Secret) -> SessionId {
        let session_id = SessionId(uuid::Uuid::new_v4());
        self.ships.insert(session_id, Ship::new(name, secret));
        session_id
    }

    pub fn update(&mut self, dt: f64) {
        for ship in self.ships.values_mut() {
            ship.update(dt);
            log::trace!("ship: {:?}", ship);
        }
    }

    pub fn iter_ships(&self) -> impl Iterator<Item = &Ship> {
        self.ships.values()
    }
}

impl Ship {
    pub fn new(name: String, secret: Secret) -> Self {
        use std::f64;

        let mut rng = rand::thread_rng();
        let position = (
            rng.gen_range(0.0, config::WORLD_WIDTH as f64),
            rng.gen_range(0.0, config::WORLD_HEIGHT as f64),
        );
        let direction = rng.gen_range(0.0, 2.0 * f64::consts::PI);
        let velocity = (
            direction.cos() * config::SHIP_INITIAL_VELOCITY,
            direction.sin() * config::SHIP_INITIAL_VELOCITY,
        );
        let acceleration = 0.0;
        let energy = 0.5;

        Self {
            name,
            secret,
            position,
            velocity,
            direction,
            acceleration,
            energy,
        }
    }

    pub fn update(&mut self, dt: f64) {
        self.position.0 =
            (self.position.0 + self.velocity.0 * dt).rem_euclid(f64::from(config::WORLD_WIDTH));
        self.position.1 =
            (self.position.1 + self.velocity.1 * dt).rem_euclid(f64::from(config::WORLD_HEIGHT));

        if !self.out_of_energy() {
            self.velocity.0 += self.direction.cos() * self.acceleration * dt;
            self.velocity.1 += self.direction.sin() * self.acceleration * dt;
        }

        let energy_delta = self.energy + config::ENERGY_REPLENISH_RATE * dt
            - self.acceleration * config::ENERGY_BOOST_COST * dt;
        self.energy = (energy_delta).max(0.0).min(config::ENERGY_MAX_LEVEL);
    }

    pub fn out_of_energy(&self) -> bool {
        self.energy < config::ENERGY_MIN_LEVEL
    }
}
