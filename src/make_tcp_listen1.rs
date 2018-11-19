
use tokio::prelude::*;
use tokio::io;
use libp2p::{
    Multiaddr,
    Transport,
    tcp::TcpConfig
};
use tokio_codec::BytesCodec;
use tokio::codec::Decoder;

fn main() {

    let tcp_transport = TcpConfig::new();
    let addr: Multiaddr = "/ip4/127.0.0.1/tcp/8080".parse().expect("invalid multiaddr");

    //let _outgoing_connec = tcp_transport.dial(addr)
    let _outgoing_connec = tcp_transport.listen_on(addr).unwrap().0
        .map_err(|e| println!("err={:?}", e))
        .for_each(|(sock, _)| {
            println!("{:?}", sock);
            // No split here now.
            sock.and_then(|sock| {

                let framed = BytesCodec::new().framed(sock);
                let (_writer, reader) = framed.split();

                let processor = reader
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

                //let amt = io::copy(reader, writer);
                //let msg = amt.then(move |result| {
                //    match result {
                //        Ok((amt, _, _)) => println!("wrote {} bytes", amt),
                //        Err(e) => println!("error: {}", e),
                //    }

                //    Ok(())
                //});

                //tokio::spawn(writer);

                Ok(())
            });

            Ok(())
        });


    tokio::run(_outgoing_connec.map_err(|e| println!("{:?}", e)));

    //println!("Hello, world!");
}
