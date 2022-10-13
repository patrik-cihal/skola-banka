use std::ops::Add;
use std::time::{Duration, Instant};
use super::*;

#[derive(Debug, Serialize, Deserialize, Hash, PartialEq, Eq, Clone)]
pub struct CardNumber([u16; 4]);

impl CardNumber {
    pub fn generate() -> Self {
        let mut result = [0; 4];
        for val in &mut result {
            *val = rand::random::<u16>() % 10_000;
        }
        Self(result)
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Expiration(u64);

impl Expiration {
    pub fn generate() -> Self {
        Self(Instant::now().add(Duration::from_secs(1000)).elapsed().as_secs())
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CVC(u16);

impl CVC {
    pub fn generate() -> Self {
        Self(rand::random::<u16>() % 1_000)
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Card {
    pub number: CardNumber,
    pub cvc: CVC,
    pub expiration: Expiration,
}

impl Card {
    pub fn new() -> Self {
        Self {
            number: CardNumber::generate(),
            cvc: CVC::generate(),
            expiration: Expiration::generate(),
        }
    }
}
