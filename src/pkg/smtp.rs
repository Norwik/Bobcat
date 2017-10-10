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

struct Listener {
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

pub struct Server {
    stream: tokio::net::TcpStream,
    state_machine: Listener,
}

impl Server {
    /// Creates a new server from a connected stream
    pub async fn new(
        domain: impl AsRef<str>,
        stream: tokio::net::TcpStream,
    ) -> Result<Self> {
        Ok(Self {
            stream,
            state_machine: Listener::new(domain),
            db: Arc::new(Mutex::new(database::Client::new().await?)),
        })
    }

    /// Runs the server loop, accepting and handling SMTP commands
    pub async fn serve(mut self) -> Result<()> {
        self.greet().await?;

        let mut buf = vec![0; 65536];
        loop {
            let n = self.stream.read(&mut buf).await?;

            if n == 0 {
                tracing::info!("Received EOF");
                self.state_machine.handle_smtp("quit").ok();
                break;
            }
            let msg = std::str::from_utf8(&buf[0..n])?;
            let response = self.state_machine.handle_smtp(msg)?;
            if response != Listener::HOLD_YOUR_HORSES {
                self.stream.write_all(response).await?;
            } else {
                tracing::debug!("Not responding, awaiting more data");
            }
            if response == Listener::KTHXBYE {
                break;
            }
        }
        match self.state_machine.state {
            State::Received(mail) => {
                self.db.lock().await.replicate(mail).await?;
            }
            State::ReceivingData(mail) => {
                tracing::info!("Received EOF before receiving QUIT");
                self.db.lock().await.replicate(mail).await?;
            }
            _ => {}
        }
        Ok(())
    }

    /// Sends the initial SMTP greeting
    async fn greet(&mut self) -> Result<()> {
        self.stream
            .write_all(Listener::OH_HAI)
            .await
            .map_err(|e| e.into())
    }
}
