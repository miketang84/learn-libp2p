
use tokio::prelude::*;
use tokio::io;
use libp2p::{
    Multiaddr,
    Transport,
    tcp::TcpConfig
};
use tokio::codec::BytesCodec;
use tokio::codec::Decoder;

fn main() {

    let tcp_transport = TcpConfig::new();
    let addr: Multiaddr = "/ip4/127.0.0.1/tcp/8080".parse().expect("invalid multiaddr");

    let listener = tcp_transport.listen_on(addr).unwrap().0.for_each(|(sock, _)| {
            println!("{:?}", sock);
            sock.and_then(|sock| {
                let framed = BytesCodec::new().framed(sock);
                let (sink, stream) = framed.split();

                let processor = stream 
                .for_each(|bytes| {
                    println!("bytes: {:?}", bytes);
                    Ok(())
                })
                .and_then(|()| {
                    println!("Socket received FIN packet and closed connection");
                    Ok(())
                })
                .or_else(|err| {
                    println!("Socket closed with error: {:?}", err);
                    Err(err)
                })
                .then(|result| {
                    println!("Socket closed with result: {:?}", result);
                    Ok(())
                });

                tokio::spawn(processor);

                Ok(())
            })

        });


    tokio::run(listener.map_err(|e|println!("{:?}", e)));
}

