// Copyright 2022 Clivern. All rights reserved.
// Use of this source code is governed by the MIT
// license that can be found in the LICENSE file.

mod pkg;

use std::error::Error;
use tokio::net::TcpListener;

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    let addr = "127.0.0.1:1025";
    let listener = TcpListener::bind(addr).await?;

    println!("Listening on address: {}", addr);

    pkg::worker::worker();

    //    loop {
    //        let (mut stream, _) = listener.accept().await?;
    //        tokio::spawn(async move {
    //            let server = pkg::server::Server::new(addr, stream).await?;
    //            server.serve().await
    //        });
    //   }
}
