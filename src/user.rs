use crate::{AccountId, Deserialize, Serialize, UserId};

#[derive(Debug, Default, Serialize, Deserialize)]
pub struct User {
    pub first_name: String,
    pub last_name: String,
    pub accounts: Vec<AccountId>,
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
        rand::random()
    }

    pub fn add_account(&mut self, account_id: AccountId) {
        self.accounts.push(account_id);
    }
}
