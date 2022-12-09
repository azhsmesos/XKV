use serde::{Deserialize, Serialize};
use serde_repr::{Deserialize_repr, Serialize_repr};

use super::errors::{XKVError, Result};

const USIZE_LEN: usize = std::mem::size_of::<usize>();
const ENTRY_HEAD_LEN: usize = USIZE_LEN * 2 + 1;

#[derive(Serialize_repr, Deserialize_repr, PartialEq, Debug)]
#[repr(u8)]
pub enum State {
    PUT = 1,
    DEL = 2,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Entry {
    pub key_len: usize,

    pub value_len: usize,

    pub key: String,

    pub value: String,

    pub state: State,
}

impl Entry {
    pub fn new(key: String, value: String, state: State) -> Self {
        Self {
            key_len: key.as_bytes().len(),
            value_len: value.as_bytes().len(),
            key,
            value,
            state,
        }
    }

    pub fn size(&self) -> usize {
        ENTRY_HEAD_LEN + self.key_len + self.value_len
    }


    pub fn encode(&self) -> Vec<u8> {

        vec![]
    }

    pub fn decode(b: &[u8; ENTRY_HEAD_LEN]) -> Result<Entry> {

        Err(XKVError::EOF)
    }
}

