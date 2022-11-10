use super::*;

#[derive(Debug, Default, Serialize, Deserialize, Eq, PartialEq, Clone)]
pub struct Account {
    owner_id: UserId,
    pub category: AccountCategory,
    balance: u64,
    cards: Vec<CardId>,
}

impl Account {
    pub fn new(owner_id: UserId, category: AccountCategory) -> Self {
        Self {
            owner_id,
            category,
            balance: 0,
            cards: vec![],
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
        rand::random()
    }
    pub fn register_card(&mut self, card_number: CardId) {
        self.cards.push(card_number);
    }
}

#[derive(
    Debug,
    Default,
    Serialize,
    Deserialize,
    EnumVariantNames,
    EnumString,
    FromRepr,
    Eq,
    PartialEq,
    Clone,
)]
#[strum(serialize_all = "camel_case")]
pub enum AccountCategory {
    #[default]
    Adult,
    Student,
}
