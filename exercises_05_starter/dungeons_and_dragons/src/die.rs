use rand::prelude::*;
use rand_chacha::ChaCha20Rng;

/// Takes in a max value and returns a random number between 1 and max
// DO NOT MODIFY
fn get_random_value(max: u8) -> u8 {
    let mut rng = ChaCha20Rng::seed_from_u64(2);
    rng.gen_range(1..=max)
}

pub enum Die {
    D4,
    D6,
    D8,
    D10,
    D12,
    D20,
}

pub struct Coin;


// MODIFY/ADD BELOW HERE ONLY

// Task 1: Create a Trait
/// Describes a rollable item, like a die or a coin.
pub trait Rollable {
    /// Returns the value of a roll, between 1 and the item's max.
    fn get_roll_value(&self) -> u8;
}

// Task 2: Implement the trait for Coin
impl Rollable for Coin {
    fn get_roll_value(&self) -> u8 {
        // A coin roll is 1 or 2. We must use the provided function.
        get_random_value(2)
    }
}

// Task 3: Implement the trait for Die
impl Rollable for Die {
    fn get_roll_value(&self) -> u8 {
        // Match on the die variant to determine the max roll value.
        match self {
            Die::D4 => get_random_value(4),
            Die::D6 => get_random_value(6),
            Die::D8 => get_random_value(8),
            Die::D10 => get_random_value(10),
            Die::D12 => get_random_value(12),
            Die::D20 => get_random_value(20),
        }
    }
}

// Task 4: Add a generic trait bound for the roll function
/// Rolls any item that implements the `Rollable` trait.
pub fn roll<T: Rollable>(item: T) -> u8 {
    item.get_roll_value()
}
