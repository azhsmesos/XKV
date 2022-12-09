use std::collections::HashMap;
use std::fs::{File, OpenOptions};
use std::io::{BufReader, BufWriter, Read, Seek, SeekFrom, Write};
use std::ops::Index;
use std::path::PathBuf;
use crate::kv::entry::{Entry, State};

use super::errors::{XKVError, Result};

const STORAGE_FILE_PREFIX: &str = "xdb";
const COMPACTION_THRESHOLD: u64 = 1 << 16;

pub trait Storage {
    fn get(&mut self, key: String) -> Result<Option<String>>;
    fn set(&mut self, key: String, val: String) -> Result<()>;
    fn remove(&mut self, key: String) -> Result<()>;
}

pub struct BitcaskStorage {

    data_path_buf: PathBuf,

    reader: BufReaderWithPos<File>,

    writer: BufWriterWithPos<File>,

    index: HashMap<String, u64>,

    pending_compact: u64,
}

impl Storage for BitcaskStorage {

    fn get(&mut self, key: String) -> Result<Option<String>> {
       todo!()
    }

    fn set(&mut self, key: String, val: String) -> Result<()> {
        todo!()
    }

    fn remove(&mut self, key: String) -> Result<()> {
        todo!()
    }
}

impl BitcaskStorage {
    pub fn open(path_buf: PathBuf) -> Result<BitcaskStorage> {
        let data_path_buf = path_buf.join(STORAGE_FILE_PREFIX.to_string() + ".data");
        let writer = BufWriterWithPos::new(
            OpenOptions::new()
                .create(true)
                .write(true)
                .append(true)
                .open(data_path_buf.as_path())?,
        )?;
        let reader = BufReaderWithPos::new(File::open(data_path_buf.as_path())?)?;
        let mut instance = BitcaskStorage {
            data_path_buf,
            reader,
            writer,
            index: HashMap::new(),
            pending_compact: 0,
        };

        Ok(instance)
    }

    fn write(&mut self, entry: Entry) -> Result<()> {

        Ok(())
    }

    fn read(&mut self, key: &str) -> Result<Entry> {

        Err(XKVError::KeyNotFound)
    }

    fn read_at(&mut self, offset: u64) -> Result<Entry> {


        Err(XKVError::KeyNotFound)
    }

    // load index
    fn load_index(&mut self) -> Result<()> {
        let mut position = 0;
        loop {
            match self.read_at(position) {
                Ok(entry) => {
                    if entry.state == State::DEL {
                        self.index.remove(&entry.key);
                        continue
                    }
                    let size = entry.size() as u64;
                    self.index.insert(entry.key, position);
                    position += size;
                },
                Err(XKVError::EOF) => {
                    self.writer.pos = position;
                    return Ok(());
                },
                Err(err) => {
                    return Err(err);
                },
            }
        }
        Ok(())
    }

    fn merge(&mut self) -> Result<()> {

        Ok(())
    }

}

struct BufReaderWithPos<R: Read + Seek> {
    reader: BufReader<R>,
    pos: u64,
}

impl <R: Read + Seek> BufReaderWithPos<R> {
    fn new(mut inner: R) -> Result<Self> {
        let pos = inner.seek(SeekFrom::Current(0))?;
        Ok(BufReaderWithPos {
            reader: BufReader::new(inner),
            pos,
        })
    }
}

impl <R: Read + Seek> Read for BufReaderWithPos<R> {
    fn read(&mut self, buf: &mut [u8]) -> std::io::Result<usize> {
        let len = self.reader.read(buf)?;
        self.pos += len as u64;
        Ok(len)
    }
}

impl <R: Read + Seek> Seek for BufReaderWithPos<R> {
    fn seek(&mut self, pos: SeekFrom) -> std::io::Result<u64> {
        self.pos = self.reader.seek(pos)?;
        Ok(self.pos)
    }
}

struct BufWriterWithPos<W: Write + Seek> {
    writer: BufWriter<W>,
    pos: u64,
}

impl <W: Write + Seek> BufWriterWithPos<W> {
    fn new(mut inner: W) -> Result<Self> {
        let pos = inner.seek(SeekFrom::Current(0))?;
        Ok(BufWriterWithPos {
            writer: BufWriter::new(inner),
            pos,
        })
    }
}

impl <W: Write + Seek> Write for BufWriterWithPos<W> {
    fn write(&mut self, buf: &[u8]) -> std::io::Result<usize> {
        let len = self.writer.write(buf)?;
        self.pos += len as u64;
        Ok(len)
    }

    fn flush(&mut self) -> std::io::Result<()> {
        self.writer.flush()
    }
}

impl <W: Write + Seek> Seek for BufWriterWithPos<W> {
    fn seek(&mut self, pos: SeekFrom) -> std::io::Result<u64> {
        self.pos = self.writer.seek(pos)?;
        Ok(self.pos)
    }
}