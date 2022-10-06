use std::env;
use std::fs::File;
use std::io::Write;

use crate::Bank;
pub struct BankSerializer;

impl BankSerializer {
    pub fn save(bank: &Bank) {
        let serialized_bank = serde_json::to_string(bank).unwrap();
        let mut file = File::create("bank_instance.json").unwrap();
        writeln!(&mut file, "{:?}", serialized_bank).unwrap();
    }
}

