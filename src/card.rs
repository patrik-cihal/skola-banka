#[derive(Debug)]
struct CardNumber([u16; 4]);

impl CardNumber {
    pub fn generate() -> Self {
        let mut result = [0; 4];
        for val in &mut result {
            *val = rand::random::<u16>() % 10_000;
        }
        Self(result)
    }
}

#[derive(Debug)]
struct CVC(u16);

impl CVC {
    pub fn generate() -> Self {
        Self(rand::random::<u16>() % 1_000)
    }
}

#[derive(Debug)]
struct Card {
    number: CardNumber,
    cvc: CVC,
    expiration: Instant,
}

impl Card {
    fn new() -> Self {
        Self {
            number: CardNumber::generate(),
            cvc: CVC::generate(),
            expiration: Instant::default(),
        }
    }
}