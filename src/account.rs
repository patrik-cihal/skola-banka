use super::*;


#[derive(Debug, Default, Serialize, Deserialize)]
pub struct Account {
    owner_id: UserId,
    category: AccountCategory,
    balance: u64,
    card: Option<Card>,
}

impl Account {
    pub fn new(owner_id: UserId, category: AccountCategory) -> Self {
        Self {
            owner_id,
            category,
            balance: 0,
            card: None,
        }
    }
    pub fn decrease_balance(&mut self, amount: u64) -> Result<(), BankError> {
        if amount > self.balance {
            Err(BankError::LowBalance)
        } else {
            self.balance -= amount;
            Ok(())
        }
    }
    pub fn increase_balance(&mut self, amount: u64) -> Result<(), BankError> {
        self.balance += amount;
        Ok(())
    }
    pub fn generate_id() -> AccountId {
        let mut result: AccountId = rand::random();
        result
    }
}

#[derive(Debug, Default, Serialize, Deserialize)]
pub enum AccountCategory {
    #[default]
    Adult,
    Student,
}
