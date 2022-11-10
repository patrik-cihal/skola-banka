use dialoguer::console::Term;
use dialoguer::theme::ColorfulTheme;
use dialoguer::Select;
use std::path::Path;

use strum::VariantNames;

use super::*;

const CREATE_ACCOUNT_ACTION: &str = "Create account";
const EDIT_ACCOUNT_ACTION: &str = "Edit account";
const SAVE_ACTION: &str = "Save";
const LOAD_ACTION: &str = "Load";
const DEBUG_ACTION: &str = "Debug";

pub struct UIInterface {
    bank: Bank,
    user_id: Option<UserId>,
}

impl UIInterface {
    pub fn new(bank: Bank, user_id: Option<UserId>) -> UIInterface {
        Self { bank, user_id }
    }

    pub fn main_menu(&mut self) {
        let logged_in = self.user_id.is_some();

        // filters action based on booleans passed inside tuple
        let actions: Vec<&str> = vec![
            (CREATE_ACCOUNT_ACTION, vec![logged_in]),
            (EDIT_ACCOUNT_ACTION, vec![logged_in]),
            (LOAD_ACTION, vec![]),
            (SAVE_ACTION, vec![]),
            (DEBUG_ACTION, vec![]),
        ]
        .iter()
        .filter_map(|(txt, conds)| {
            if conds.into_iter().all(|x| *x) {
                Some(*txt)
            } else {
                None
            }
        })
        .collect();

        let select = dialoguer::Select::with_theme(&ColorfulTheme::default())
            .default(0)
            .items(&actions)
            .interact_on(&Term::stdout())
            .unwrap();

        match actions[select] {
            CREATE_ACCOUNT_ACTION => {
                if let Some(active_user_id) = self.user_id {
                    println!(
                        "Account created successully with id: {:?} ",
                        self.bank
                            .create_account(active_user_id, Self::choose_account_category())
                            .unwrap()
                    );
                } else {
                    println!("You need to login or register first.");
                }
            }
            EDIT_ACCOUNT_ACTION => {
                let account_ids = &self
                    .bank
                    .users
                    .get(&self.user_id.unwrap())
                    .unwrap()
                    .accounts;
                let account_id = Select::with_theme(&ColorfulTheme::default())
                    .default(0)
                    .items(account_ids)
                    .interact_on(&Term::stdout())
                    .unwrap();
                let new_account_type = Self::choose_account_category();
                self.bank
                    .change_account_type(account_ids[account_id], new_account_type)
                    .unwrap();
            }
            LOAD_ACTION => {
                self.bank = FileStorage::load(Path::new(BANK_SAVE_PATH)).unwrap();
            }
            SAVE_ACTION => {
                FileStorage::save(&self.bank, Path::new(BANK_SAVE_PATH)).unwrap();
            }
            DEBUG_ACTION => {
                println!("{:?}", self.bank);
            }

            _ => println!("Unknown option"),
        }
        self.main_menu();
    }

    pub fn choose_account_category() -> AccountCategory {
        AccountCategory::from_repr(
            Select::with_theme(&ColorfulTheme::default())
                .default(0)
                .items(AccountCategory::VARIANTS)
                .interact_on(&Term::stdout())
                .unwrap(),
        )
        .unwrap()
    }
}
