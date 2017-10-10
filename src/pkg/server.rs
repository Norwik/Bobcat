// Copyright 2022 Clivern. All rights reserved.
// Use of this source code is governed by the MIT
// license that can be found in the LICENSE file.

use crate::pkg::smtp::Listener;

/// SMTP server, which handles user connections
/// and replicates received messages to the database.
pub struct Server {
    stream: tokio::net::TcpStream,
    listener: Listener,
}

impl Server {
    /// Creates a new server from a connected stream
    pub async fn new(
        domain: impl AsRef<str>,
        stream: tokio::net::TcpStream,
    ) -> Result<Self> {
        Ok(Self {
            stream,
            listener: Listener::new(domain),
        })
    }

    /// Runs the server loop, accepting and handling SMTP commands
    pub async fn serve(mut self) -> Result<()> {
        self.hello().await?;

        let mut buf = vec![0; 65536];

        loop {
            let n = self.stream.read(&mut buf).await?;

            if n == 0 {
                self.listener.handle_smtp("quit").ok();
                break;
            }

            let msg = std::str::from_utf8(&buf[0..n])?;
            let response = self.listener.handle_smtp(msg)?;

            if response != Listener::HOLD_ON {
                self.stream.write_all(response).await?;
            } else {
                // Not responding, awaiting more data
            }

            if response == Listener::BYE {
                break;
            }
        }

        Ok(())
    }

    /// Sends the initial SMTP greeting
    async fn hello(&mut self) -> Result<()> {
        self.stream
            .write_all(Listener::HAY)
            .await
            .map_err(|e| e.into())
    }
}
