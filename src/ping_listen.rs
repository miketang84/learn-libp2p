extern crate tokio;
extern crate libp2p;

use tokio::net::TcpListener;
use tokio::net::TcpStream;
use libp2p::ping::protocol::Ping;
use futures::{Future, Stream};
use libp2p::core::upgrade::{InboundUpgrade, OutboundUpgrade};

// TODO: rewrite tests with the MemoryTransport

fn main () {
    let listener = TcpListener::bind(&"127.0.0.1:8080".parse().unwrap()).unwrap();
    let listener_addr = listener.local_addr().unwrap();
    println!("{:?}", listener_addr);

    let server = listener
        .incoming()
        .into_future()
        .map_err(|(e, _)| e.into())
        .and_then(|(c, _)| {
            Ping::<()>::default().upgrade_inbound(c.unwrap(), b"/ipfs/ping/1.0.0")
        })
    .flatten();
/*
    let client = TcpStream::connect(&listener_addr)
        .map_err(|e| e.into())
        .and_then(|c| {
            Ping::<()>::default().upgrade_outbound(c, b"/ipfs/ping/1.0.0")
        })
    .and_then(|mut pinger| {
        pinger.ping(());
        pinger.into_future().map(|_| ()).map_err(|_| panic!())
    })
    .map(|_| ());
*/
    let mut runtime = tokio::runtime::Runtime::new().unwrap();
//    runtime.block_on(server.select(client).map_err(|_| panic!())).unwrap();
    runtime.block_on(server).unwrap();
}


