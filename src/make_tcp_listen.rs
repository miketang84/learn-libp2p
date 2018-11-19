
use tokio::prelude::*;
use tokio::io;
use libp2p::{
    Multiaddr,
    Transport,
    tcp::TcpConfig
};


fn main() {

    let tcp_transport = TcpConfig::new();
    let addr: Multiaddr = "/ip4/127.0.0.1/tcp/8088".parse().expect("invalid multiaddr");

    //let _outgoing_connec = tcp_transport.dial(addr)
    let _outgoing_connec = tcp_transport.listen_on(addr).unwrap().0
        .map_err(|e| println!("err={:?}", e))
        .for_each(|(sock, _)| {
            println!("{:?}", sock);
            // No split here now.
            //let (reader, writer) = socket.split();
            //let amt = io::copy(reader, writer);

            //let msg = amt.then(move |result| {
            //    match result {
            //        Ok((amt, _, _)) => println!("wrote {} bytes", amt),
            //    Err(e) => println!("error: {}", e),
            //    }

            //    Ok(())
            //});

            //tokio::spawn(msg);

            //sock.and_then(|sock| {
            //    let handle_conn = io::read_exact(sock, [0; 3])
            //    .map(|(_, buf)| println!("{:?}", buf))
            //    .map_err(|err| panic!("IO error {:?}", err));

            //    tokio::spawn(handle_conn).map_err(|_| ())

            //})
            //.map_err(|_| ())
            Ok(())
        });


    tokio::run(_outgoing_connec.map_err(|e| println!("{:?}", e)));

    //println!("Hello, world!");
}
