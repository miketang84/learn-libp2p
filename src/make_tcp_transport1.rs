
use tokio::prelude::*;
use tokio::io;
use libp2p::{
    Multiaddr,
    Transport,
    tcp::TcpConfig
};

use tokio::timer::Interval;
use std::time::{Duration, Instant};


fn main() {


    let task = Interval::new(Instant::now(), Duration::from_millis(1000))
        .for_each(move |instant| {
            println!("fire; instant={:?}", instant);
            let tcp_transport = TcpConfig::new();
            let addr: Multiaddr = "/ip4/127.0.0.1/tcp/8080".parse().expect("invalid multiaddr");
            let socket = tcp_transport.dial(addr).unwrap();
            let action = socket.then(|sock| -> Result<(), ()> {
                sock.unwrap().write(b"hello, libp2p").unwrap();
                Ok(())
            });

            tokio::spawn(action);

            Ok(())
        })
        .map_err(|e| panic!("interval errored; err={:?}", e));


    tokio::run(task);
}
