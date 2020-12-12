// Copyright 2022 Clivern. All rights reserved.
// Use of this source code is governed by the MIT
// license that can be found in the LICENSE file.

/// Returns the value of an environment variable
///
/// # Arguments
///
/// * `key` - environment variable name
///
/// # Examples
///
/// ```
/// mod util;
///
/// println!("CARGO_PKG_NAME = {}!", util::environ::get_config("CARGO_PKG_NAME"));
/// ```
pub fn get_config(key: &str, def: &str) -> String {
    match std::env::var(key) {
        Ok(val) => val,
        Err(_) => def.to_string(),
    }
}

#[test]
fn test_get_config() {
    assert_eq!(get_config("CARGO_PKG_NAME", ""), "elk");
    assert_eq!(get_config("CARGO__PKG_NAME", "default"), "default");
}
