// The width of the world, before coordinates wrap around.
pub const WORLD_WIDTH: u32 = 1024;

// The height of the world, before coordinates wrap around.
pub const WORLD_HEIGHT: u32 = 1024;

// The minimum energy level needed in the ship battery to perform any action.
pub const ENERGY_MIN_LEVEL: f64 = 0.1;

// The maximum energy level that can be stored in the ship battery.
pub const ENERGY_MAX_LEVEL: f64 = 1.0;

// The rate at which energy replenishes (i.e. from the ships reactor core).
pub const ENERGY_REPLENISH_RATE: f64 = 0.05;

// The relative cost of performing an engine boost.
//
// In other words, to accelerate with acceleration `x units/sec²`, you need `x * ENERGY_BOOST_COST energy/sec`.
pub const ENERGY_BOOST_COST: f64 = 0.1;

// The radius of a ship.
pub const SHIP_RADIUS: f64 = 2.0;

// The initial velocity of the ship.
pub const SHIP_INITIAL_VELOCITY: f64 = 32.0;
