mod storage;
pub use storage::{FileStorage, Storage};

mod bank;
pub use bank::Bank;

mod user;
pub use user::User;

mod account;
pub use account::{Account, AccountCategory};

mod card;
pub use card::CardNumber;

mod observer;
pub use observer::Observable;
pub use observer::Publisher;
pub use observer::SubscribeEvent;

pub type UserId = u64;
pub type AccountId = u64;
pub type CardId = u64;

pub use serde::{Deserialize, Serialize};

pub use bank::BankError;
pub use card::Card;
pub use std::collections::HashMap;

pub use strum::{EnumString, EnumVariantNames, FromRepr};

mod ui;
pub use ui::UIInterface;

const BANK_SAVE_PATH: &str = "bank_instance.json";
