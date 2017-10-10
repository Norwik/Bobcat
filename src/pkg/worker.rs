// Copyright 2022 Clivern. All rights reserved.
// Use of this source code is governed by the MIT
// license that can be found in the LICENSE file.

use std::{thread, time};
use tokio::task;

pub fn worker() {
    // Launch Background Worker
    task::spawn(async {
        loop {
            println!("{:?}", "Hello");
            thread::sleep(time::Duration::from_millis(1000));
        }
    });
}
