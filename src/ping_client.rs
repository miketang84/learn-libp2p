extern crate tokio;
extern crate libp2p;

use tokio::net::TcpListener;
use tokio::net::TcpStream;
use libp2p::ping::protocol::Ping;
use futures::{Future, Stream};
use libp2p::core::upgrade::{InboundUpgrade, OutboundUpgrade};


fn main () {
    let client = TcpStream::connect(&"127.0.0.1:38067".parse().unwrap())
        .map_err(|e| e.into())
        .and_then(|c| {
            Ping::<()>::default().upgrade_outbound(c, b"/ipfs/ping/1.0.0")
        })
    .and_then(|mut pinger| {
/*
        for n in 0..100 {
            pinger.ping(());
        }

        pinger.take(100)
            .map(|_| println!("received pong"))
            .collect()
            //.map(|val| { assert_eq!(val, (0..1000).collect::<Vec<_>>()); })
            .map_err(|e| panic!())
*/
        pinger.ping(());

        pinger.into_future()
            .map(|_| println!("received pong"))
            .map_err(|e| panic!())
    })
    .map(|_| ());

    let mut runtime = tokio::runtime::Runtime::new().unwrap();
    runtime.block_on(client).unwrap();
}


