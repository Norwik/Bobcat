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

#[derive(Clone, Debug, PartialEq, Eq)]
enum State {
    Fresh,
    Greeted,
    ReceivingRcpt(Mail),
    ReceivingData(Mail),
    Received(Mail),
}

pub struct Listener {
    state: State,
    ehlo_greeting: String,
}

impl Listener {
    const HAY: &[u8] = b"220 bobcat\n";
    const OK: &[u8] = b"250 Ok\n";
    const AUTH_OK: &[u8] = b"235 Ok\n";
    const END: &[u8] = b"354 End data with <CR><LF>.<CR><LF>\n";
    const BYE: &[u8] = b"221 Bye\n";
    const HOLD_ON: &[u8] = &[];

    pub fn new(domain: impl AsRef<str>) -> Self {
        let domain = domain.as_ref();
        let ehlo_greeting =
            format!("250-{domain} Hello {domain}\n250 AUTH PLAIN LOGIN\n");

        Self {
            state: State::Fresh,
            ehlo_greeting,
        }
    }

    pub fn handle_smtp(&mut self, raw_msg: &str) -> Result<&[u8]> {
        // let mut msg = raw_msg.split_whitespace();
        // let command = msg.next().context("received empty command")?.to_lowercase();
        // let state = std::mem::replace(&mut self.state, State::Fresh);
    }

    fn legal_recipient(to: &str) -> bool {
        let to = to.to_lowercase();
        !to.contains("admin") && !to.contains("postmaster") && !to.contains("hostmaster")
    }
}
