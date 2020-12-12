// Copyright 2022 Clivern. All rights reserved.
// Use of this source code is governed by the MIT
// license that can be found in the LICENSE file.

extern crate dotenv;

mod module;
mod util;

use tokio::io::{AsyncReadExt, AsyncWriteExt};
use tokio::net::TcpListener;

use dotenv::dotenv;
use module::command::{Command, Type};
use std::env;
use std::error::Error;
use util::environ::get_config;

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    // Load .env file
    dotenv().ok();

    // Set up our TCP listener.
    let addr = env::args()
        .nth(1)
        .unwrap_or_else(|| get_config("HOSTNAME", "127.0.0.1:8080"));

    // We create a TCP listener which will listen for incoming requests
    let listener = TcpListener::bind(&addr).await?;

    println!("Listening on: {}", addr);

    loop {
        let (mut socket, _) = listener.accept().await?;

        tokio::spawn(async move {
            let mut buf = vec![0; 1024];

            // In a loop, read data from the socket and write the data back.
            loop {
                let n = socket
                    .read(&mut buf)
                    .await
                    .expect("failed to read data from socket");

                if n == 0 {
                    return;
                }

                let command = &buf[0..n];
                let out;
                let mut cmd: Command = Command::new("", "", 0, Type::Unknown);

                let _s = match std::str::from_utf8(command) {
                    Ok(v) => {
                        match Command::from_str(v) {
                            Ok(v) => {
                                cmd = v;

                                match *cmd.get_name() {
                                    Type::Ping => {
                                        out = String::from("PONG\n");
                                    }
                                    Type::Exit => {
                                        out = String::from("OK\n");
                                    }
                                    Type::Get => {
                                        out = String::from("OK\n");
                                    }
                                    Type::Set => {
                                        out = String::from("OK\n");
                                    }
                                    Type::Update => {
                                        out = String::from("OK\n");
                                    }
                                    Type::Delete => {
                                        out = String::from("OK\n");
                                    }
                                    _ => {
                                        out = String::from("UNKNOWN\n");
                                    }
                                }
                            }
                            Err(e) => out = format!("{}\n", e),
                        }

                        println!("{:?}", cmd);
                    }
                    Err(e) => panic!("Invalid UTF-8 sequence: {}", e),
                };

                socket
                    .write_all(&out.as_str().as_bytes())
                    .await
                    .expect("failed to write data to socket");
            }
        });
    }
}
