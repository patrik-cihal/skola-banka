use std::collections::HashMap;
use std::error::Error;
use std::time::Instant;
use serde::Serializer;
use crate::account::AccountCategory;
use crate::user::User;

use super::*;

#[derive(Default, Serialize, Deserialize, Debug)]
pub struct Bank {
    accounts: HashMap<AccountId, Account>,
    pub users: HashMap<UserId, User>,
    cards: HashMap<CardNumber, Card>
}

#[derive(Debug)]
pub enum BankError {
    LowBalance,
    AccountNotFound,
    UserNotFound,
}

impl Bank {
    pub fn transfer_money(
        &mut self,
        from_account_id: AccountId,
        to_account_id: AccountId,
        amount: u64,
    ) -> Result<(), BankError> {
        if !self.accounts.contains_key(&from_account_id)
            || !self.accounts.contains_key(&to_account_id)
        {
            return Err(BankError::AccountNotFound);
        }
        if let Some(from_account) = self.accounts.get_mut(&from_account_id) {
            from_account.decrease_balance(amount)?;
        }
        if let Some(to_account) = self.accounts.get_mut(&to_account_id) {
            to_account.increase_balance(amount)?;
        }
        Ok(())
    }
    pub fn register_user(&mut self, user: User) -> UserId {
        let user_id = User::generate_id();
        self.users.insert(user_id, user);
        user_id
    }

    pub fn create_account(
        &mut self,
        owner_id: UserId,
        category: AccountCategory,
    ) -> Result<AccountId, BankError> {
        if let Some(user) = self.users.get_mut(&owner_id) {
            let account_id = Account::generate_id();
            let account = Account::new(owner_id, category);

            self.accounts.insert(account_id, account);
            user.add_account(account_id);

            Ok(account_id)
        }
        else {
            Err(BankError::UserNotFound)
        }
    }
    
    pub fn create_card(&mut self, account_id: AccountId) -> Result<(), BankError> {
        if let Some(account) = self.accounts.get_mut(&account_id) {
            let card = Card::new();
            self.cards.insert(card.number.clone(), card);
            account.register_card(card.number);

            Ok(())
        }
        else {
            Err(BankError::AccountNotFound)
        }
    }

    pub fn get_account(&self, account_id: AccountId) -> Option<&Account> {
        self.accounts.get(&account_id)
    }

    pub fn reward_account(&mut self, account_id: AccountId, money: u64) -> Result<(), BankError> {
        if let Some(account) = self.accounts.get_mut(&account_id) {
            account.increase_balance(money)
        }
        else {
            Err(BankError::AccountNotFound)
        }
    }

}
