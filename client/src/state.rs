use anyhow::Result;
use serde_derive::{Deserialize, Serialize};
use std::{
    collections::BTreeMap,
    fs::File,
    io::{Read, Write},
    path::PathBuf,
};

pub const STATE_PATH: &str = "STATE_PATH";
pub const DEFAULT_STATE_PATH: &str = "state.json";

#[derive(Default, Debug, Serialize, Deserialize)]
pub(crate) struct State {
    pub batch_root_map: BTreeMap<common::BatchId, Vec<u8>>,
}

impl State {
    pub(crate) fn load_state() -> Result<State> {
        let path = PathBuf::from(
            std::env::var(STATE_PATH)
                .ok()
                .unwrap_or(DEFAULT_STATE_PATH.to_string()),
        );

        let mut file = match File::open(path) {
            Ok(file) => file,
            Err(ref err) if err.kind() == std::io::ErrorKind::NotFound => {
                // If the file does not exist, create it with default state
                let default_state = State::default();
                default_state.save_state()?;
                return Ok(default_state);
            }
            Err(err) => return Err(err.into()),
        };

        let mut contents = String::new();
        file.read_to_string(&mut contents)?;
        if contents.is_empty() {
            // If the file is empty, initialize it with default state
            let default_state = State::default();
            default_state.save_state()?;
            return Ok(default_state);
        }
        let state: State = serde_json::from_str(&contents)?;
        Ok(state)
    }

    pub(crate) fn save_state(&self) -> Result<()> {
        let path = PathBuf::from(
            std::env::var(STATE_PATH)
                .ok()
                .unwrap_or(DEFAULT_STATE_PATH.to_string()),
        );
        let serialized = serde_json::to_string(self)?;
        let mut file = File::create(path)?;
        file.write_all(serialized.as_bytes())?;
        Ok(())
    }
}
