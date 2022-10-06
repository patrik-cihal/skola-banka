use std::env;
use std::fs::File;
use std::io::Write;

use crate::Bank;
pub struct BankSerializer;

impl BankSerializer {
    pub fn save(bank: &Bank) {
        let serialized_bank = serde_json::to_string_pretty(bank).unwrap();
        let mut file = File::create("bank_instance.json").unwrap();
        file.write_all(serialized_bank.as_bytes()).unwrap();
    }
}

