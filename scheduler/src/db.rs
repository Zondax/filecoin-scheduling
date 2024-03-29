use crate::{Error, Result};
use serde::{de::DeserializeOwned, Serialize};
use sled::{Config, Db};
use std::iter::DoubleEndedIterator;
use std::iter::Iterator;
use std::ops::RangeBounds;
use std::path::Path;

pub struct Database {
    db: Db,
}

impl Database {
    pub fn open<P: AsRef<Path>>(path: P, temporary: bool) -> Result<Self> {
        if !path.as_ref().exists() {
            std::fs::create_dir_all(&path).map_err(|e| {
                Error::Database(format!(
                    "cannot create database in {:?} err: {}",
                    path.as_ref(),
                    e.to_string()
                ))
            })?;
        }
        let config = Config::default()
            .path(path)
            .temporary(temporary)
            .print_profile_on_drop(true)
            .flush_every_ms(Some(1000));
        let db = config.open().map_err(Error::from)?;
        tracing::debug!(
            "db size {:?} - was recovered {:?}",
            db.size_on_disk(),
            db.was_recovered()
        );
        Ok(Self { db })
    }

    pub fn insert<K, V>(&self, key: K, value: V) -> Result<()>
    where
        K: Serialize + DeserializeOwned,
        V: Serialize + DeserializeOwned,
    {
        let key = bincode::serialize(&key).map_err(|e| Error::Other(e.to_string()))?;
        let value_bytes = bincode::serialize(&value).map_err(|e| Error::Other(e.to_string()))?;
        let _ = self.db.insert(key, value_bytes)?;
        Ok(())
    }

    pub fn remove<K, V>(&self, key: K) -> Result<Option<V>>
    where
        K: Serialize,
        V: DeserializeOwned,
    {
        let key = bincode::serialize(&key).map_err(|e| Error::Other(e.to_string()))?;
        self.db
            .remove(key)
            .map(|res| {
                res.and_then(|o| {
                    let value: V = bincode::deserialize(o.as_ref()).ok()?;
                    Some(value)
                })
            })
            .map_err(Error::from)
    }

    pub fn _flush(&self) -> Result<usize> {
        self.db.flush().map_err(Error::from)
    }

    fn range<K, R, V>(
        &self,
        range: R,
    ) -> impl DoubleEndedIterator<Item = Result<Result<(K, V)>>> + Send + Sync
    where
        K: DeserializeOwned,
        V: DeserializeOwned,
        R: RangeBounds<Vec<u8>>,
    {
        self.db.range(range).map(|res| {
            res.map_err(Error::from).map(|(k, v)| {
                let value = bincode::deserialize(&v).map_err(|e| Error::Other(e.to_string()))?;
                let key = bincode::deserialize(&k).map_err(|e| Error::Other(e.to_string()))?;
                Ok((key, value))
            })
        })
    }

    pub fn iter<K, V>(
        &self,
    ) -> impl DoubleEndedIterator<Item = Result<Result<(K, V)>>> + Send + Sync
    where
        K: DeserializeOwned,
        V: DeserializeOwned,
    {
        self.range::<K, _, V>(..)
    }
}

impl From<sled::Error> for Error {
    fn from(e: sled::Error) -> Self {
        Self::Database(e.to_string())
    }
}
