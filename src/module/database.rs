// Copyright 2022 Clivern. All rights reserved.
// Use of this source code is governed by the MIT
// license that can be found in the LICENSE file.

use bincode;
use bincode::serialize_into;
use serde::{Deserialize, Serialize};

use std::collections::HashMap;
use std::fs::File;
use std::sync::RwLock;

#[derive(Serialize, Debug, Deserialize)]
// Database type
struct Database {
    // the sync lock
    lock: RwLock<()>,
    // The database path
    path: String,
    // The k/v data
    data: HashMap<String, String>,
}

// Database type methods
impl Database {
    ///
    /// Returns a database object with the provided path
    ///
    /// # Arguments
    ///
    /// * `path` - A string that holds the path to the database
    ///
    /// # Examples
    ///
    /// ```
    /// let mut db: Database = Database::new("/etc/langmore/langmore.db");
    /// ```
    ///
    pub fn new<S: Into<String>>(path: S) -> Database {
        Database {
            lock: RwLock::new(()),
            path: path.into(),
            data: HashMap::new(),
        }
    }

    ///
    /// Updates the database path
    ///
    /// # Arguments
    ///
    /// * `path` - A string that holds the path to the database
    ///
    pub fn set_path<S: Into<String>>(&mut self, path: S) {
        let _lock = self.lock.write().expect("Lock is used");
        self.path = path.into();
    }

    ///
    /// Gets the database path
    ///
    /// # Returns
    ///
    /// * `path` - A string that holds the path to the database
    ///
    pub fn get_path(&self) -> String {
        format!("{}", self.path)
    }

    ///
    /// Store or Update a Key Value
    ///
    /// # Arguments
    ///
    /// * `key` - A string that holds the key
    /// * `value` - A string that holds the value
    ///
    /// # Returns
    ///
    /// * The number of records in the database
    ///
    pub fn set<S: Into<String>>(&mut self, key: S, value: S) -> usize {
        let _lock = self.lock.write().expect("Lock is used");
        self.data.insert(key.into(), value.into());
        self.data.len()
    }

    ///
    /// Removes a Key from the database
    ///
    /// # Arguments
    ///
    /// * `key` - A string that holds the key
    ///
    /// # Returns
    ///
    /// * The number of records in the database
    ///
    pub fn remove<S: Into<String>>(&mut self, key: S) -> usize {
        let _lock = self.lock.write().expect("Lock is used");
        self.data.remove(&key.into());
        self.data.len()
    }

    ///
    /// Get the Value of a Key
    ///
    /// # Arguments
    ///
    /// * `key` - A string that holds the key
    ///
    /// # Returns
    ///
    /// * The Value of the Key
    ///
    pub fn get<S: Into<String>>(&self, key: S) -> String {
        match self.data.get(&key.into()) {
            Some(value) => value.to_string(),
            _ => String::new(),
        }
    }

    ///
    /// Loads the database from the path and return Database object
    ///
    /// # Returns
    ///
    /// *  An instance of the database object
    ///
    pub fn load(&self) -> Database {
        let _lock = self.lock.write().expect("Lock is used");
        let f = File::open(self.path.as_str()).unwrap();
        let mut db: Database = Database::new(self.path.as_str());
        db.data = bincode::deserialize_from(&f).unwrap();
        db
    }

    ///
    /// Serialize the data and store them in the database path
    ///
    pub fn flush(&self) {
        let _lock = self.lock.write().expect("Lock is used");
        let mut f = File::create(self.path.as_str()).unwrap();
        serialize_into(&mut f, &self.data).unwrap();
        drop(f)
    }
}

#[test]
fn test_database_methods() {
    let mut db: Database = Database::new("./cache/langmore.db");

    db.set_path("./cache/langmore.db");

    assert_eq!(db.get_path(), "./cache/langmore.db".to_string());

    assert_eq!(db.set("key1", "value1"), 1);
    assert_eq!(db.set("key2", "value2"), 2);
    assert_eq!(db.set("key3", "value3"), 3);

    assert_eq!(db.get("key1"), "value1".to_string());
    assert_eq!(db.get("key2"), "value2".to_string());
    assert_eq!(db.get("key3"), "value3".to_string());

    assert_eq!(db.remove("key1"), 2);
    assert_eq!(db.remove("key2"), 1);
    assert_eq!(db.remove("key3"), 0);

    assert_eq!(db.set("key1", "value1"), 1);
    assert_eq!(db.set("key2", "value2"), 2);
    assert_eq!(db.set("key3", "value3"), 3);

    db.flush();

    let db2: Database = db.load();

    assert_eq!(db2.get("key1"), "value1".to_string());
    assert_eq!(db2.get("key2"), "value2".to_string());
    assert_eq!(db2.get("key3"), "value3".to_string());
}
