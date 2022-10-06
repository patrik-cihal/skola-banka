use crate::{AccountId, UserId, Serialize, Deserialize};

#[derive(Debug, Default, Serialize, Deserialize)]
pub struct User {
    first_name: String,
    last_name: String,
    accounts: Vec<AccountId>,
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
        let mut result: UserId = rand::random();
        result
    }
}