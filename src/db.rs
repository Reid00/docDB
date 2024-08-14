use serde::de::DeserializeOwned;
use serde::Serialize;

use crate::error::{DocError, Result};
use crate::iterator::DocDbIterator;
use crate::serialization::{SerializationMethod, Serializer};
use std::fs;
use std::time::{SystemTime, UNIX_EPOCH};
use std::{
    collections::HashMap,
    path::{Path, PathBuf},
    time::{Duration, Instant},
};

/// An enum that determines the policy of dumping DocDb changes into the file
pub enum DumpPolicy {
    /// Never dump any change, file will always remain read-only
    NeverDump,
    AutoDump,
    DumpRelyRequest,
    PeriodicDump(Duration),
}

pub struct DocDb {
    ///
    map: HashMap<String, Vec<u8>>,
    serializer: Serializer,
    db_file_path: PathBuf,
    dump_policy: DumpPolicy,
    last_dump: Instant,
}

impl DocDb {
    pub fn new<P: AsRef<Path>>(
        db_path: P,
        dump_policy: DumpPolicy,
        serialize_method: SerializationMethod,
    ) -> Self {
        let mut path_buf = PathBuf::new();
        path_buf.push(db_path);

        Self {
            map: HashMap::new(),
            serializer: Serializer::new(serialize_method),
            db_file_path: path_buf,
            dump_policy: dump_policy,
            last_dump: Instant::now(),
        }
    }

    #[cfg(feature = "json")]
    pub fn new_json<P: AsRef<Path>>(db_path: P, dump_policy: DumpPolicy) -> Self {
        DocDb::new(db_path, dump_policy, SerializationMethod::Json)
    }

    #[cfg(feature = "yaml")]
    pub fn new_yaml<P: AsRef<Path>>(db_path: P, dump_policy: DumpPolicy) -> Self {
        DocDb::new(db_path, dump_policy, SerializationMethod::Yaml)
    }

    #[cfg(feature = "bincode")]
    pub fn new_bincode<P: AsRef<Path>>(db_path: P, dump_policy: DumpPolicy) -> Self {
        DocDb::new(db_path, dump_policy, SerializationMethod::Bin)
    }

    pub fn load<P: AsRef<Path>>(
        db_path: P,
        dump_policy: DumpPolicy,
        ser_method: SerializationMethod,
    ) -> Result<DocDb> {
        let content = match fs::read(db_path.as_ref()) {
            Ok(file_content) => file_content,
            Err(err) => return Err(DocError::IO(err)),
        };

        let serializer = Serializer::new(ser_method);

        let maps_from_file = match serializer.deserialize_db(&content) {
            Ok(maps) => maps,
            Err(err) => return Err(err),
        };

        let mut db_path_buf = PathBuf::new();
        db_path_buf.push(db_path);

        Ok(DocDb {
            map: maps_from_file,
            serializer,
            db_file_path: db_path_buf,
            dump_policy,
            last_dump: Instant::now(),
        })
    }

    pub fn load_read_only<P: AsRef<Path>>(
        db_path: P,
        serialization_method: SerializationMethod,
    ) -> Result<DocDb> {
        DocDb::load(db_path, DumpPolicy::NeverDump, serialization_method)
    }

    #[cfg(feature = "json")]
    pub fn load_json<P: AsRef<Path>>(db_path: P, dump_policy: DumpPolicy) -> Result<Self> {
        DocDb::load(db_path, dump_policy, SerializationMethod::Json)
    }

    #[cfg(feature = "yaml")]
    pub fn load_yaml<P: AsRef<Path>>(db_path: P, dump_policy: DumpPolicy) -> Result<Self> {
        Self::load(db_path, dump_policy, SerializationMethod::Yaml)
    }

    #[cfg(feature = "bincode")]
    pub fn load_bin<P: AsRef<Path>>(db_path: P, dump_policy: DumpPolicy) -> Result<Self> {
        Self::load(db_path, dump_policy, SerializationMethod::Bin)
    }

    pub fn dump(&mut self) -> Result<()> {
        if let DumpPolicy::NeverDump = self.dump_policy {
            return Ok(());
        }

        match self.serializer.serialize_db(&self.map) {
            Ok(ser_data) => {
                let temp_file_path = format!(
                    "{}.temp.{}",
                    self.db_file_path.to_str().unwrap(),
                    SystemTime::now()
                        .duration_since(UNIX_EPOCH)
                        .unwrap()
                        .as_secs()
                );

                fs::write(&temp_file_path, ser_data)?;
                // match fs::write(&temp_file_path, ser_data) {
                //     Ok(_) => (),
                //     Err(err) => return Err(DocError::IO(err)),
                // }

                fs::rename(temp_file_path, &self.db_file_path)?;
                // match fs::rename(temp_file_path, &self.db_file_path) {
                //     Ok(_) => (),
                //     Err(err) => return Err(DocError::IO(err)),
                // }

                if let DumpPolicy::PeriodicDump(_dur) = self.dump_policy {
                    self.last_dump = Instant::now();
                }
                Ok(())
            }
            Err(err) => Err(err),
        }
    }

    pub fn dump_now(&mut self) -> Result<()> {
        match self.dump_policy {
            DumpPolicy::AutoDump => self.dump(),
            DumpPolicy::PeriodicDump(duration) => {
                //
                if Instant::now().duration_since(self.last_dump) > duration {
                    self.last_dump = Instant::now();
                    self.dump()?;
                }
                Ok(())
            }
            _ => Ok(()),
        }
    }

    pub fn set<T: Serialize>(&mut self, key: &str, val: &T) -> Result<()> {
        let ser_data = match self.serializer.serialize_data(val) {
            Ok(data) => data,
            Err(err) => return Err(err),
        };

        let original_val = self.map.insert(key.to_string(), ser_data);

        match self.dump_now() {
            Ok(_) => Ok(()),
            // set value failed, need to roll back
            Err(err) => {
                match original_val {
                    // not exist before insert
                    None => self.map.remove(key),
                    // exist and reset old val
                    Some(orig_val) => self.map.insert(String::from(key), orig_val.to_vec()),
                };

                Err(err)
            }
        }
    }

    pub fn get<T: DeserializeOwned>(&self, key: &str) -> Option<T> {
        match self.map.get(key) {
            Some(v) => self.serializer.deserialize_data(v),
            None => None,
        }
    }

    pub fn exist(&self, key: &str) -> bool {
        self.map.get(key).is_some()
    }

    /// Get a vector of all the keys in the DB.
    ///
    /// The keys returned in the vector are not references to the actual key string
    /// objects but rather a clone of them.
    pub fn get_all_keys(&self) -> Vec<String> {
        self.map.keys().cloned().collect()
    }

    /// Get the total number of keys in the DB.
    pub fn total_nums(&self) -> usize {
        self.map.len()
    }

    pub fn rem(&mut self, key: &str) -> Result<bool> {
        let remove_map = match self.map.remove(key) {
            // exists key, return old value and dump db now
            Some(v) => match self.dump_now() {
                // dump successfully, return some(v)
                Ok(_) => Some(v),
                // dump failed, restore key in map
                Err(err) => {
                    self.map.insert(key.to_string(), v);
                    return Err(err);
                }
            },
            None => None,
        };

        Ok(remove_map.is_some())
    }

    pub fn iter(&self) -> DocDbIterator {
        DocDbIterator {
            map_iter: self.map.iter(),
            serializer: &self.serializer,
        }
    }
}

impl Drop for DocDb {
    fn drop(&mut self) {
        if !matches!(
            self.dump_policy,
            DumpPolicy::NeverDump | DumpPolicy::DumpRelyRequest
        ) {
            let _ = self.dump();
        }
    }
}
