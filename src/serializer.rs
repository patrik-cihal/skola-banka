use std::{env, fs};
use std::fs::File;
use std::io::Write;
use std::path::Path;

use crate::Bank;
pub struct BankSerializer;

impl BankSerializer {
    pub fn save(bank: &Bank) {
        let serialized_bank = serde_json::to_string_pretty(bank).unwrap();
        let mut file = File::create("bank_instance.json").unwrap();
        file.write_all(serialized_bank.as_bytes()).unwrap();
    }

    pub fn load(path: &Path) -> Bank {
        let serialized_bank = fs::read_to_string(&path).unwrap();
        serde_json::from_str::<Bank>(&serialized_bank).unwrap()
    }
}

