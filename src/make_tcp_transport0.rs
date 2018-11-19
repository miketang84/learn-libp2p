
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
    let socket = tcp_transport.dial(addr).unwrap();
    let action = socket.then(|sock| -> Result<(), ()> {
        sock.unwrap().write(b"hello, libp2p").unwrap();
        Ok(())
    });

    tokio::run(action);
}
