// Copyright 2022 Clivern. All rights reserved.
// Use of this source code is governed by the MIT
// license that can be found in the LICENSE file.

use std::error::Error;

use std::{thread, time};
use tokio::io::{AsyncReadExt, AsyncWriteExt};
use tokio::net::TcpListener;
use tokio::task;

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    let addr = "127.0.0.1:1025";
    let listener = TcpListener::bind(addr).await?;

    println!("Listening on address: {}", addr);

    // Launch Background Worker
    task::spawn(async {
        loop {
            println!("{:?}", "Hello");
            thread::sleep(time::Duration::from_millis(1000));
        }
    });

    loop {
        let (mut socket, _) = listener.accept().await?;

        tokio::spawn(async move {
            let mut buf = vec![0; 1024];

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

                let _s = match std::str::from_utf8(command) {
                    Ok(v) => {
                        out = String::from(v);

                        if out == "Bye" {
                            break;
                        }
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
