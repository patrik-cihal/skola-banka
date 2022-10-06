use bank_lib::*;

use std::fs::File;
use std::io::Write;
use std::env;

mod serializer;
use serializer::BankSerializer;


fn main() {
    let mut bank = Bank::default();
    let user = User::new("Patrik".to_string(), "Cihal".to_string());
    let user_id = bank.register_user(user);

    let account_id = bank
        .create_account(user_id.clone(), AccountCategory::Student)
        .unwrap();
    let account2_id = bank
        .create_account(user_id.clone(), AccountCategory::Adult)
        .unwrap();
    
    bank.create_card(account_id);

    println!("{:?}", bank.accounts.get(&account_id));
    bank.accounts
        .get_mut(&account_id)
        .unwrap()
        .increase_balance(1000);
    println!("{:?}", bank.accounts.get(&account_id));
    bank.transfer_money(account_id, account2_id, 1000).unwrap();
    println!("{:?}", bank.accounts.get(&account_id));
    println!("{:?}", bank.accounts.get(&account2_id));

    BankSerializer::save(&bank);

    let serialized_bank = serde_json::to_string(&bank).unwrap();
    let deserialized_bank = serde_json::from_str::<Bank>(&serialized_bank).unwrap();

    println!("{:?}", deserialized_bank);


}
