// Copyright 2022 Clivern. All rights reserved.
// Use of this source code is governed by the MIT
// license that can be found in the LICENSE file.


#[derive(Clone, Debug, Default, PartialEq, Eq)]
pub struct Mail {
    pub id: String,
    pub from: String,
    pub to: Vec<String>,
    pub data: String,
}
