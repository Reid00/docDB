use std::collections::HashMap;
use std::fmt;

use crate::error::{DocError, Result};
use serde::de::DeserializeOwned;
use serde::Serialize;

pub struct DocSerializer {}

impl DocSerializer {
    fn serialize_data<V>(&self, data: &V) -> Result<Vec<u8>>
    where
        V: Serialize,
    {
        let v = serde_json::to_string(data)?;
        Ok(v.into_bytes())
    }

    fn deserialize_data<V>(&self, data: &[u8]) -> Result<V>
    where
        V: DeserializeOwned,
    {
        let v = serde_json::from_str(std::str::from_utf8(data)?)?;
        Ok(v)
    }

    fn serialize_db(&self, map: &HashMap<String, Vec<u8>>) -> Result<Vec<u8>> {
        let mut db_map = HashMap::new();

        db_map = map
            .iter()
            .map(|(k, v)| (k, std::str::from_utf8(v).unwrap()))
            .collect();

        let ret = serde_json::to_string(&db_map)?;
        Ok(ret.into_bytes())
    }

    fn deserialize_db(&self, db: &[u8]) -> Result<HashMap<String, Vec<u8>>> {
        let db_data = std::str::from_utf8(db)?;
        let data = serde_json::from_str::<HashMap<String, String>>(db_data)?;

        let mut hmap = HashMap::new();

        hmap = data
            .iter()
            .map(|(k, v)| (k.to_string(), v.as_bytes().to_vec()))
            .collect();
        Ok(hmap)
    }
}

type DbMap = HashMap<String, Vec<u8>>;

/// An enum for specifying the serialization method to use when creating a new PickleDB database
/// or loading one from a file
#[derive(Debug)]
pub enum SerializationMethod {
    /// [JSON serialization](https://crates.io/crates/serde_json)
    Json,

    /// [Bincode serialization](https://crates.io/crates/bincode)
    Bin,

    /// [YAML serialization](https://crates.io/crates/serde_yaml)
    Yaml,

    /// [CBOR serialization](https://crates.io/crates/serde_cbor)
    Cbor,
}

impl From<i32> for SerializationMethod {
    fn from(value: i32) -> Self {
        match value {
            0 => SerializationMethod::Json,
            1 => SerializationMethod::Bin,
            2 => SerializationMethod::Yaml,
            3 => SerializationMethod::Cbor,
            _ => SerializationMethod::Json,
        }
    }
}

impl fmt::Display for SerializationMethod {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{:?}", self)
    }
}

/// serde json for db
#[cfg(feature = "json")]
struct JsonSerializer {}

#[cfg(feature = "json")]
impl JsonSerializer {
    fn new() -> Self {
        JsonSerializer {}
    }

    fn serialize_data<V>(&self, v: &V) -> Result<Vec<u8>>
    where
        V: Serialize,
    {
        match serde_json::to_string(&v) {
            Ok(serd_data) => Ok(serd_data.into_bytes()),
            Err(err) => Err(DocError::Serialization(err.to_string())),
        }
    }

    fn deserialize_data<V>(&self, ser_data: &[u8]) -> Option<V>
    where
        V: DeserializeOwned,
    {
        match serde_json::from_str(std::str::from_utf8(ser_data).unwrap()) {
            Ok(v) => Some(v),
            Err(_err) => None,
        }
    }

    fn serialize_db(&self, map: &DbMap) -> Result<Vec<u8>> {
        let json_map: HashMap<&String, &str> = map
            .iter()
            .map(|(k, v)| (k, std::str::from_utf8(v).unwrap()))
            .collect();

        match serde_json::to_string(&json_map) {
            Ok(v) => Ok(v.into_bytes()),
            Err(err) => Err(DocError::Serialization(err.to_string())),
        }
    }

    pub fn deserialize_db(&self, ser_data: &[u8]) -> Result<DbMap> {
        match serde_json::from_str::<HashMap<String, String>>(
            std::str::from_utf8(ser_data).unwrap(),
        ) {
            Ok(json_map) => {
                // let mut db_map = DbMap::new();
                let db_map = json_map
                    .iter()
                    .map(|(k, v)| (k.to_string(), v.as_bytes().to_vec()))
                    .collect();
                Ok(db_map)
            }
            Err(err) => Err(DocError::Deserialization(err.to_string())),
        }
    }
}

/// serde yaml for db
#[cfg(feature = "yaml")]
struct YamlSerializer {}

#[cfg(feature = "yaml")]
impl YamlSerializer {
    fn new() -> Self {
        Self {}
    }

    fn serialize_data<V>(&self, v: &V) -> Result<Vec<u8>>
    where
        V: Serialize,
    {
        match serde_yaml::to_string(v) {
            Ok(ser_data) => Ok(ser_data.into_bytes()),
            Err(err) => Err(DocError::Serialization(err.to_string())),
        }
    }

    fn deserialize_data<V>(&self, ser_data: &[u8]) -> Option<V>
    where
        V: DeserializeOwned,
    {
        match serde_yaml::from_str(std::str::from_utf8(ser_data).unwrap()) {
            Ok(data) => Some(data),
            Err(_err) => None,
        }
    }

    fn serialize_db(&self, map: &DbMap) -> Result<Vec<u8>> {
        let hmap: HashMap<&String, &str> = map
            .iter()
            .map(|(k, v)| (k, std::str::from_utf8(v).unwrap()))
            .collect();

        match serde_yaml::to_string(&hmap) {
            Ok(d) => Ok(d.into_bytes()),
            Err(err) => Err(DocError::Serialization(err.to_string())),
        }
    }

    fn deserialize_db(&self, db: &[u8]) -> Result<DbMap> {
        match serde_yaml::from_str::<HashMap<String, String>>(std::str::from_utf8(db).unwrap()) {
            Ok(data) => {
                let db_map = data
                    .iter()
                    .map(|(k, v)| (k.to_string(), v.as_bytes().to_vec()))
                    .collect();
                Ok(db_map)
            }
            Err(err) => Err(DocError::Deserialization(err.to_string())),
        }
    }
}

#[cfg(feature = "cbor")]
struct CborSerializer {}
#[cfg(feature = "cbor")]
impl CborSerializer {
    fn new() -> Self {
        Self {}
    }

    fn serialize_data<V>(&self, v: &V) -> Result<Vec<u8>>
    where
        V: Serialize,
    {
        todo!()
    }

    fn deserialize_data<V>(&self, v: &[u8]) -> Option<V>
    where
        V: DeserializeOwned,
    {
        todo!()
    }

    fn serialize_db(&self, map: &DbMap) -> Result<Vec<u8>> {
        todo!()
    }

    fn deserialize_db(&self, db: &[u8]) -> Result<DbMap> {
        todo!()
    }
}

#[cfg(feature = "bincode")]
struct BincodeSerializer {}
#[cfg(feature = "bincode")]
impl BincodeSerializer {
    fn new() -> Self {
        Self {}
    }

    fn serialize_data<V>(&self, v: &V) -> Result<Vec<u8>>
    where
        V: Serialize,
    {
        match bincode::serialize(v) {
            Ok(ser_data) => Ok(ser_data),
            Err(err) => Err(DocError::Serialization(err.to_string())),
        }
    }

    fn deserialize_data<V>(&self, v: &[u8]) -> Option<V>
    where
        V: DeserializeOwned,
    {
        match bincode::deserialize(v) {
            Ok(v) => Some(v),
            Err(err) => {
                println!("deserialize err: {}", err);
                None
            }
        }
    }

    fn serialize_db(&self, map: &DbMap) -> Result<Vec<u8>> {
        self.serialize_data(map)
    }

    fn deserialize_db(&self, db: &[u8]) -> Result<DbMap> {
        match self.deserialize_data(db) {
            Some(map) => Ok(map),
            None => Err(DocError::Deserialization(
                "cannot deserialize from db".to_string(),
            )),
        }
    }
}

pub(crate) struct Serializer {
    ser_method: SerializationMethod,

    #[cfg(feature = "json")]
    json_serializer: JsonSerializer,
    #[cfg(feature = "yaml")]
    yaml_serializer: YamlSerializer,
    #[cfg(feature = "cbor")]
    cbor_serializer: CborSerializer,
    #[cfg(feature = "bincode")]
    bin_serializer: BincodeSerializer,
}

impl Serializer {
    pub(crate) fn new(ser_method: SerializationMethod) -> Self {
        Self {
            ser_method: ser_method,
            #[cfg(feature = "json")]
            json_serializer: JsonSerializer::new(),
            #[cfg(feature = "yaml")]
            yaml_serializer: YamlSerializer::new(),
            #[cfg(feature = "cbor")]
            cbor_serializer: CborSerializer::new(),
            #[cfg(feature = "bincode")]
            bin_serializer: BincodeSerializer::new(),
        }
    }

    pub fn serialize_data<V>(&self, v: &V) -> Result<Vec<u8>>
    where
        V: Serialize,
    {
        #[allow(unreachable_patterns)]
        match self.ser_method {
            #[cfg(feature = "json")]
            SerializationMethod::Json => self.json_serializer.serialize_data(v),
            #[cfg(feature = "yaml")]
            SerializationMethod::Yaml => self.yaml_serializer.serialize_data(v),
            #[cfg(feature = "cbor")]
            SerializationMethod::Cbor => self.cbor_serializer.serialize_data(v),
            #[cfg(feature = "bincode")]
            SerializationMethod::Bin => self.bin_serializer.serialize_data(v),
            #[cfg(feature = "json")]
            _ => self.json_serializer.serialize_data(v),
            // #[cfg(feature = "yaml")]
            // _ => self.yaml_serializer.serialize_data(v),
            // #[cfg(feature = "cbor")]
            // _ => self.cbor_serializer.serialize_data(v),
            // #[cfg(feature = "bincode")]
            // _ => self.bin_serializer.serialize_data(v),
        }
    }

    pub(crate) fn deserialize_data<T>(&self, ser_data: &[u8]) -> Option<T>
    where
        T: DeserializeOwned,
    {
        #[allow(unreachable_patterns)]
        match self.ser_method {
            #[cfg(feature = "json")]
            SerializationMethod::Json => self.json_serializer.deserialize_data(ser_data),
            #[cfg(feature = "yaml")]
            SerializationMethod::Yaml => self.yaml_serializer.deserialize_data(ser_data),
            #[cfg(feature = "cbor")]
            SerializationMethod::Cbor => self.cbor_serializer.deserialize_data(ser_data),
            #[cfg(feature = "bincode")]
            SerializationMethod::Bin => self.bin_serializer.deserialize_data(ser_data),
            #[cfg(feature = "json")]
            _ => self.json_serializer.deserialize_data(ser_data),
        }
    }

    pub(crate) fn serialize_db(&self, map: &DbMap) -> Result<Vec<u8>> {
        #[allow(unreachable_patterns)]
        match self.ser_method {
            #[cfg(feature = "json")]
            SerializationMethod::Json => self.json_serializer.serialize_db(map),
            #[cfg(feature = "yaml")]
            SerializationMethod::Yaml => self.yaml_serializer.serialize_db(map),
            #[cfg(feature = "cbor")]
            SerializationMethod::Cbor => self.cbor_serializer.serialize_db(map),
            #[cfg(feature = "bincode")]
            SerializationMethod::Bin => self.bin_serializer.serialize_db(map),
            _ => self.json_serializer.serialize_db(map),
        }
    }
    pub(crate) fn deserialize_db(&self, v: &[u8]) -> Result<DbMap> {
        #[allow(unreachable_patterns)]
        match self.ser_method {
            #[cfg(feature = "json")]
            SerializationMethod::Json => self.json_serializer.deserialize_db(v),
            #[cfg(feature = "yaml")]
            SerializationMethod::Yaml => self.yaml_serializer.deserialize_db(v),
            #[cfg(feature = "cbor")]
            SerializationMethod::Cbor => self.cbor_serializer.deserialize_db(v),
            #[cfg(feature = "bincode")]
            SerializationMethod::Bin => self.bin_serializer.deserialize_db(v),
            #[cfg(feature = "json")]
            _ => self.json_serializer.deserialize_db(v),
        }
    }
}
