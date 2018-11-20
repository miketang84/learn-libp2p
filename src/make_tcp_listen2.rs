
use tokio::prelude::*;
use tokio::io;
use libp2p::{
    Multiaddr,
    Transport,
    tcp::TcpConfig
};

use tokio::runtime::current_thread::Runtime;


fn main() {

    let tcp_transport = TcpConfig::new();
    let addr: Multiaddr = "/ip4/127.0.0.1/tcp/8080".parse().expect("invalid multiaddr");
    let mut rt = Runtime::new().unwrap();
    let handle = rt.handle();


    let listener = tcp_transport.listen_on(addr).unwrap().0.for_each(|(sock, _)| {
        println!("{:?}", sock);
        sock.and_then(|sock| {
            let handle_conn = io::read_exact(sock, [0; 3])
            .map(|(_, buf)| {
                assert_eq!(buf, [1, 2, 3]);
                println!("{:?}", buf);
            })
            .map_err(|err| panic!("IO error {:?}", err));

            handle.spawn(handle_conn).unwrap();

            Ok(())
        })
    });

    rt.block_on(listener).unwrap();
    rt.run().unwrap();
}
