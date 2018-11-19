
use tokio::prelude::*;
use tokio::io;
use libp2p::{
    Multiaddr,
    Transport,
    tcp::TcpConfig
};

fn main() {

    let tcp_transport = TcpConfig::new();
    let addr: Multiaddr = "/ip4/127.0.0.1/tcp/8080".parse().expect("invalid multiaddr");

    //let _outgoing_connec = tcp_transport.dial(addr)
    let _outgoing_connec = tcp_transport.listen_on(addr).unwrap().0
        .map_err(|e| println!("err={:?}", e))
        .for_each(|(sock, _)| {
            println!("{:?}", sock);


            Ok(())
        });


    tokio::run(_outgoing_connec.map_err(|e| println!("{:?}", e)));

    //println!("Hello, world!");
}
