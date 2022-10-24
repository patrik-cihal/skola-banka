use banka_delta::*;

use std::env;
use std::fs::File;
use std::io::Write;
use std::path::Path;

mod serializer;
use serializer::BankSerializer;

use dialoguer::console::Term;
use dialoguer::theme::ColorfulTheme;
use dialoguer::{Input, Select};

use strum::VariantNames;

use std::error::Error;

fn main() {
    let mut bank = Bank::default();
    let user = User::new("Patrik".to_string(), "Cihal".to_string());
    let user_id = bank.register_user(user);

    fn select_account_type() -> AccountCategory {
        AccountCategory::from_repr(
            Select::with_theme(&ColorfulTheme::default())
                .default(0)
                .items(AccountCategory::VARIANTS)
                .interact_on(&Term::stdout())
                .unwrap(),
        )
        .unwrap()
    }

    const BANK_ACTIONS: [&str; 5] = ["Create account", "Edit account", "Load", "Save", "Debug"];
    const BANK_SAVE_PATH: &str = "bank_instance.json";

    loop {
        let select = dialoguer::Select::with_theme(&ColorfulTheme::default())
            .default(0)
            .items(&BANK_ACTIONS)
            .interact_on(&Term::stdout())
            .unwrap();

        match select {
            0 => {
                println!(
                    "Account created successully with id: {:?} ",
                    bank.create_account(user_id, select_account_type()).unwrap()
                );
            }
            1 => {
                let account_ids = &bank.users.get(&user_id).unwrap().accounts;
                let account_id = Select::with_theme(&ColorfulTheme::default())
                    .default(0)
                    .items(account_ids)
                    .interact_on(&Term::stdout())
                    .unwrap();
                let new_account_type = select_account_type();
                bank.change_account_type(account_ids[account_id], new_account_type);
            }
            2 => {
                FileStorage::save(&bank, Path::new(BANK_SAVE_PATH));
            }
            3 => {
                bank = FileStorage::load(Path::new(BANK_SAVE_PATH)).unwrap();
            }
            4 => {
                println!("{:?}", bank);
            }

            _ => println!("Unknown option"),
        }
    }

    //println!("{:?}", bank);
}
