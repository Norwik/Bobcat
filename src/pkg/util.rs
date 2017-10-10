// Copyright 2022 Clivern. All rights reserved.
// Use of this source code is governed by the MIT
// license that can be found in the LICENSE file.

use uuid::Uuid;

pub fn get_v4_uuid() -> Uuid {
    Uuid::new_v4()
}

#[test]
fn test_get_uuid() {
    assert_eq!(get_v4_uuid().to_string() != "", true);
    assert_eq!(get_v4_uuid().to_string() != "", true);
    assert_eq!(get_v4_uuid().to_string() != "", true);
    assert_eq!(get_v4_uuid().to_string() != "", true);
    assert_eq!(get_v4_uuid().to_string() != "", true);
    assert_eq!(get_v4_uuid().to_string() != "", true);
}
