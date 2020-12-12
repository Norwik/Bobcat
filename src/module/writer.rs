// Copyright 2022 Clivern. All rights reserved.
// Use of this source code is governed by the MIT
// license that can be found in the LICENSE file.

use std::fs::metadata;
use std::fs::read_to_string;
use std::fs::File;
use std::fs::OpenOptions;
use std::io::Write;
use std::path::Path;

// Writer type
pub struct Writer {}

// Writer type methods
impl Writer {
    ///
    /// Returns a writer object
    ///
    /// # Returns
    ///
    /// * An instance of the writer object
    ///
    pub fn new() -> Writer {
        Writer {}
    }

    ///
    /// Check if file exists
    ///
    /// # Arguments
    ///
    /// * `path` - A string that holds the path to the file
    ///
    /// # Returns
    ///
    /// * A boolean whether file exists or not
    ///
    pub fn file_exists(&self, path: String) -> bool {
        let path = Path::new(path.as_str());

        if path.exists() {
            true
        } else {
            false
        }
    }

    ///
    /// Write file content
    ///
    /// # Arguments
    ///
    /// * `path` - A string that holds the path to the file
    /// * `content` - The file content
    ///
    /// # Returns
    ///
    /// * Error raised
    ///
    pub fn overwrite(&self, path: String, content: String) -> Result<(), String> {
        let file = File::create(path.as_str());

        match file {
            Ok(mut fi) => match fi.write_all(content.as_str().as_bytes()) {
                Err(e) => Err(format!("Error raised: {}", e.to_string())),
                Ok(_) => Ok(()),
            },
            Err(e) => Err(format!("Error raised: {}", e.to_string())),
        }
    }

    ///
    /// Append to file content
    ///
    /// # Arguments
    ///
    /// * `path` - A string that holds the path to the file
    /// * `line` - The line content to append
    ///
    /// # Returns
    ///
    /// * Error raised
    ///
    pub fn append(&self, path: String, line: String) -> Result<(), String> {
        let file = OpenOptions::new()
            .write(true)
            .append(true)
            .create(true)
            .open(path.to_string());

        match file {
            Ok(mut fi) => match fi.write_all(line.as_str().as_bytes()) {
                Err(e) => Err(format!("Error raised: {}", e.to_string())),
                Ok(_) => Ok(()),
            },
            Err(e) => Err(format!("Error raised: {}", e.to_string())),
        }
    }

    ///
    /// Reads the file content
    ///
    /// # Arguments
    ///
    /// * `path` - A string that holds the path to the file
    ///
    /// # Returns
    ///
    /// * The file content
    /// * Error raised
    ///
    pub fn read(&self, path: String) -> Result<String, String> {
        let fi = read_to_string(path.to_string());

        match fi {
            Ok(content) => Ok(content),
            Err(err) => Err(format!("Error raised: {}", err.to_string())),
        }
    }

    ///
    /// Get the file size in bytes
    ///
    /// # Arguments
    ///
    /// * `path` - A string that holds the path to the file
    ///
    /// # Returns
    ///
    /// * The file size in bytes and error
    /// * Error raised
    ///
    fn filesize(&self, path: String) -> Result<u64, String> {
        let fi = metadata(path.to_string());

        match fi {
            Ok(file) => Ok(file.len()),
            Err(err) => Err(format!("Error raised: {}", err.to_string())),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    /// test file_exists method
    fn test_file_exists() {
        let wt: Writer = Writer::new();
        assert_eq!(wt.file_exists("cache/.gitignore".to_string()), true);
        assert_eq!(wt.file_exists("cache/gitignore".to_string()), false);
    }

    #[test]
    /// test overwrite method
    fn test_overwrite() {
        let wt: Writer = Writer::new();
        let result =
            wt.overwrite("cache/test1.log".to_string(), "Hello World".to_string());

        match result {
            Ok(_) => {
                assert!(true, "overwrite operation succeeded!");
            }
            Err(_) => {
                assert!(false, "overwrite operation failed!");
            }
        }
    }

    #[test]
    /// test append method
    fn test_append() {
        let wt: Writer = Writer::new();
        let result = wt.append("cache/test2.log".to_string(), "Hello World".to_string());

        match result {
            Ok(_) => {
                assert!(true, "append operation succeeded!");
            }
            Err(_) => {
                assert!(false, "append operation failed!");
            }
        }
    }

    #[test]
    /// test read method
    fn test_read() {
        let wt: Writer = Writer::new();
        let _ = wt.overwrite("cache/test4.log".to_string(), "Hello World".to_string());
        let result = wt.read("cache/test4.log".to_string());

        match result {
            Ok(data) => {
                assert_eq!(data, "Hello World".to_string());
                assert!(true, "read operation succeeded!");
            }
            Err(_) => {
                assert!(false, "read operation failed!");
            }
        }
    }

    #[test]
    /// test filesize method
    fn test_filesize() {
        let wt: Writer = Writer::new();
        let _ = wt.overwrite("cache/test3.log".to_string(), "Hello World".to_string());
        let result = wt.filesize("cache/test3.log".to_string());

        match result {
            Ok(size) => {
                assert!(size > 1, "filesize operation succeeded!");
            }
            Err(_) => {
                assert!(false, "filesize operation failed!");
            }
        }
    }
}
