#[derive(Debug, Default)]
struct User {
    first_name: String,
    last_name: String,
    accounts: Vec<Account>,
}

impl User {
    pub fn new(first_name: String, last_name: String) -> Self {
        Self {
            first_name,
            last_name,
            accounts: vec![],
        }
    }
    pub fn generate_id() -> UserId {
        let mut result: UserId = Default::default();
        for val in &mut result {
            *val = rand::random();
        }
        result
    }
}