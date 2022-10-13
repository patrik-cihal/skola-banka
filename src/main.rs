use banka_delta::*;

use std::fs::File;
use std::io::Write;
use std::env;
use std::path::Path;

mod serializer;
use serializer::BankSerializer;


fn main() {
    let mut bank = Bank::default();
    let user = User::new("Patrik".to_string(), "Cihal".to_string());
    let user_id = bank.register_user(user);

    let account_id = bank
        .create_account(user_id, AccountCategory::Student)
        .unwrap();
    let account2_id = bank
        .create_account(user_id, AccountCategory::Adult)
        .unwrap();
    
    bank.create_card(account_id).unwrap();

    println!("{:?}", bank.get_account(account_id).unwrap());

    bank.reward_account(account_id, 1000).unwrap();

    println!("{:?}", bank.get_account(account_id));

    bank.transfer_money(account_id, account2_id, 1000).unwrap();
    println!("{:?}", bank.get_account(account_id));
    println!("{:?}", bank.get_account(account2_id));

    FileStorage::save(&bank, Path::new("bank_instance.json")).unwrap();

    let bank = BankSerializer::load(Path::new("bank_instance.json"));

    println!("{:?}", bank);
}
