mod account;
mod user;

use user::User;
use account::Account;

use std::collections::HashMap;
use std::error::Error;
use std::time::Instant;

type UserId = [u8; 16];
type AccountId = [u8; 32];

#[derive(Default)]
struct Bank {
    accounts: HashMap<AccountId, Account>,
    users: HashMap<UserId, User>,
}

#[derive(Debug)]
enum BankError {
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
        self.users.insert(user_id.clone(), user);
        user_id
    }

    pub fn create_account(
        &mut self,
        owner_id: UserId,
        category: AccountCategory,
    ) -> Result<AccountId, BankError> {
        if !self.users.contains_key(&owner_id) {
            Err(BankError::UserNotFound)
        } else {
            let account_id = Account::generate_id();
            let account = Account::new(owner_id, category);
            self.accounts.insert(account_id.clone(), account);
            Ok(account_id)
        }
    }
}



