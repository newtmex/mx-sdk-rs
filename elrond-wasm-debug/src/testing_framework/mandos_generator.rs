use std::{collections::BTreeMap, fs::File, io::Write};

use elrond_wasm::types::Address;
use mandos::serde_raw::{ScenarioRaw, StepRaw};
use serde::Serialize;

use super::raw_converter::*;
use crate::world_mock::AccountData;

pub struct MandosGenerator {
    scenario: ScenarioRaw,
}

impl MandosGenerator {
    pub fn new() -> Self {
        Self {
            scenario: ScenarioRaw {
                check_gas: None,
                comment: None,
                gas_schedule: None,
                name: None,
                steps: Vec::new(),
            },
        }
    }

    pub fn write_mandos_output(self, file_path: &str) {
        let buf = Vec::new();
        let formatter = serde_json::ser::PrettyFormatter::with_indent(b"    ");
        let mut ser = serde_json::Serializer::with_formatter(buf, formatter);
        self.scenario.serialize(&mut ser).unwrap();
        let mut serialized = String::from_utf8(ser.into_inner()).unwrap();
        serialized.push('\n');

        let mut file = File::create(file_path).unwrap();
        file.write_all(serialized.as_bytes()).unwrap();
    }
}

impl MandosGenerator {
    pub fn set_account(&mut self, address: &Address, acc: &AccountData) {
        let mut accounts_raw = BTreeMap::new();

        let addr_as_str = bytes_to_hex(address.as_bytes());
        let acc_raw = account_as_raw(&acc);
        accounts_raw.insert(addr_as_str, acc_raw);

        let step = StepRaw::SetState {
            accounts: accounts_raw,
            block_hashes: Vec::new(),
            comment: None,
            current_block_info: None,
            new_addresses: Vec::new(),
            previous_block_info: None,
        };
        self.scenario.steps.push(step);
    }
}
