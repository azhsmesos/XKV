use serde::{Deserialize, Serialize};
use serde_repr::*;

use super::errors::Result;

pub const USIZE_LEN: usize = std::mem::size_of::<usize>();
pub const ENTRY_HEAD_LEN: usize = USIZE_LEN * 2 + 1;

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
        let mut buf = vec![0; self.size()];
        // key len
        buf[0..USIZE_LEN].copy_from_slice(&self.key_len.to_be_bytes());
        // value len
        buf[USIZE_LEN..USIZE_LEN * 2].copy_from_slice(&self.value_len.to_be_bytes());
        // state if delete
        buf[USIZE_LEN * 2..ENTRY_HEAD_LEN].copy_from_slice(bincode::serialize(&self.state).unwrap().as_slice());
        // key
        buf[ENTRY_HEAD_LEN..ENTRY_HEAD_LEN + self.key_len].copy_from_slice(self.key.as_bytes());
        // value
        buf[ENTRY_HEAD_LEN + self.key_len..].copy_from_slice(self.value.as_bytes());
        buf
    }

    pub fn decode(b: &[u8; ENTRY_HEAD_LEN]) -> Result<Entry> {
        let key_len = usize::from_be_bytes(b[0..USIZE_LEN].try_into()?);
        let value_len = usize::from_be_bytes(b[USIZE_LEN..USIZE_LEN * 2].try_into()?);
        let state: State = bincode::deserialize(&b[USIZE_LEN * 2..ENTRY_HEAD_LEN])?;
        Ok(Entry {
            key_len,
            value_len,
            state,
            key: String::new(),
            value: String::new(),
        })
    }
}

