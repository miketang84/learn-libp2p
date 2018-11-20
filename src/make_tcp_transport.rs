
use tokio::prelude::*;
use tokio::io;
use libp2p::{
    Multiaddr,
    Transport,
    tcp::TcpConfig
};

use tokio::runtime::current_thread::Runtime;

fn main() {

    let addr = "/ip4/127.0.0.1/tcp/8080".parse::<Multiaddr>().unwrap();
    let tcp = TcpConfig::new();
    // Obtain a future socket through dialing
    let socket = tcp.dial(addr.clone()).unwrap();
    // Define what to do with the socket once it's obtained
    let action = socket.then(|sock| -> Result<(), ()> {
        sock.unwrap().write(&[0x1, 0x2, 0x3]).unwrap();
        Ok(())
    });
    // Execute the future in our event loop
    let mut rt = Runtime::new().unwrap();
    let _ = rt.block_on(action).unwrap();

}
