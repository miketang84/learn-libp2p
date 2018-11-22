// Copyright 2018 Parity Technologies (UK) Ltd. // // Permission is hereby granted, free of charge, to any person obtaining a // copy of this software and associated documentation files (the "Software"), 
// to deal in the Software without restriction, including without limitation // the rights to use, copy, modify, merge, publish, distribute, sublicense, // and/or sell copies of the Software, and to 
permit persons to whom the // Software is furnished to do so, subject to the following conditions: // // The above copyright notice and this permission notice shall be included in // all copies or 
substantial portions of the Software. // // THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF ANY KIND, EXPRESS // OR IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF MERCHANTABILITY, // 
FITNESS FOR A PARTICULAR PURPOSE AND NONINFRINGEMENT. IN NO EVENT SHALL THE // AUTHORS OR COPYRIGHT HOLDERS BE LIABLE FOR ANY CLAIM, DAMAGES OR OTHER // LIABILITY, WHETHER IN AN ACTION OF CONTRACT, TORT 
OR OTHERWISE, ARISING // FROM, OUT OF OR IN CONNECTION WITH THE SOFTWARE OR THE USE OR OTHER // DEALINGS IN THE SOFTWARE.

//! A basic chat application demonstrating libp2p and the Floodsub protocol. //! //! Using two terminal windows, start two instances. Take note of the listening //! address of the first instance and start 
the second with this address as the //! first argument. In the first terminal window, run: //! ```text //! cargo run --example chat //! ``` //! It will print the PeerId and the listening address, e.g. 
`Listening on //! "/ip4/0.0.0.0/tcp/24915"` //! //! In the second terminal window, start a new instance of the example with: //! ```text //! cargo run --example chat -- /ip4/127.0.0.1/tcp/24915 //! ``` 
//! The two nodes connect. Type a message in either terminal and hit return: the //! message is sent and printed in the other terminal.Close with Ctrl-c. //! //! You can of course open more terminal 
windows and add more participants. //! Dialing any of the other peers will propagate the new participant to all //! chat members and everyone will receive all messages.

use futures::prelude::*; use libp2p::{
    Transport,
    core::upgrade::{self, OutboundUpgradeExt},
    secio,
    mplex,
};
use libp2p::floodsub::FloodsubBehaviour; use libp2p::core::nodes::swarm::NetworkBehaviour; use tokio::codec::{FramedRead, LinesCodec}; use libp2p::CommonTransport;

struct MyChat<A, B, D, E>
    where
    D: tokio::io::AsyncRead,
    D: tokio::io::AsyncWrite, {
    floodsub_topic: libp2p::floodsub::Topic,
    framed_stdin: FramedRead<A, B>,
    swarm: libp2p::Swarm<CommonTransport, FloodsubBehaviour<D>, E>,
}

impl<A, B, D, E> Future for MyChat<A, B, D, E>
    where
    D: tokio::io::AsyncRead,
    D: tokio::io::AsyncWrite, {
    type Item = ();
    type Error = ();

    fn poll(&mut self) -> Poll<(), ()> {
        loop {
            match self.framed_stdin.poll().expect("Error while polling stdin") {
                Async::Ready(Some(line)) => self.swarm.publish(&self.floodsub_topic, line.as_bytes()),
                    Async::Ready(None) => panic!("Stdin closed"),
                    Async::NotReady => break,
            };
        }

        loop {
            match self.swarm.poll().expect("Error while polling swarm") {
                Async::Ready(Some(message)) => {
                    println!("Received: {:?}", String::from_utf8_lossy(&message.data));
                },
                Async::Ready(None) | Async::NotReady => break,
            }
        }

        Ok(Async::NotReady)
    }
}


fn main() {
    // Create a random PeerId
    let local_key = secio::SecioKeyPair::ed25519_generated().unwrap();
    let local_peer_id = local_key.to_peer_id();
    println!("Local peer id: {:?}", local_peer_id);

    // Set up a an encrypted DNS-enabled TCP Transport over the Mplex protocol
    let transport = libp2p::CommonTransport::new()
        .with_upgrade(secio::SecioConfig::new(local_key))
        .and_then(move |out, _| {
            let peer_id = out.remote_key.into_peer_id();
            let upgrade = mplex::MplexConfig::new().map_outbound(move |muxer| (peer_id, muxer) );
            upgrade::apply_outbound(out.stream, upgrade).map_err(|e| e.into_io_error())
        });

    // Create a Floodsub topic
    let floodsub_topic = libp2p::floodsub::TopicBuilder::new("chat").build();

    // Create a Swarm to manage peers and events
    let mut swarm = {
        let mut behaviour = libp2p::floodsub::FloodsubBehaviour::new(local_peer_id);
        behaviour.subscribe(floodsub_topic.clone());
        libp2p::Swarm::new(transport, behaviour, libp2p::core::topology::MemoryTopology::empty())
    };

    // Listen on all interfaces and whatever port the OS assigns
    let addr = libp2p::Swarm::listen_on(&mut swarm, "/ip4/0.0.0.0/tcp/0".parse().unwrap()).unwrap();
    println!("Listening on {:?}", addr);

    // Reach out to another node
    if let Some(to_dial) = std::env::args().nth(1) {
        let dialing = to_dial.clone();
        match to_dial.parse() {
            Ok(to_dial) => {
                match libp2p::Swarm::dial_addr(&mut swarm, to_dial) {
                    Ok(_) => println!("Dialed {:?}", dialing),
                    Err(e) => println!("Dial {:?} failed: {:?}", dialing, e)
                }
            },
            Err(err) => println!("Failed to parse address to dial: {:?}", err),
        }
    }

    // Read full lines from stdin
    let stdin = tokio_stdin_stdout::stdin(0);
    let mut framed_stdin = FramedRead::new(stdin, LinesCodec::new());

    let my_chat = MyChat {
        framed_stdin: framed_stdin,
        floodsub_topic: floodsub_topic,
        swarm: swarm,
    };

    tokio::run(my_chat);
    
}

/*

mike@spirit:~/works/learn-libp2p$ cargo build
   Compiling learn-libp2p v0.1.0 (/home/mike/works/learn-libp2p) error[E0599]: no method named `poll` found for type `tokio_io::_tokio_codec::framed_read::FramedRead<A, B>` in the current scope
  --> src/chat.rs:75:37
   |                                                                                                                                                                                                                
75 | match self.framed_stdin.poll().expect("Error while polling stdin") {
   |                                     ^^^^
   |                                                                                                                                                                                                                
   = note: the method `poll` exists but the following trait bounds were not satisfied:
           `tokio_io::_tokio_codec::framed_read::FramedRead<A, B> : futures::stream::Stream`
                                                                                                                                                                                                                    
error[E0599]: no method named `expect` found for type `futures::poll::Async<libp2p_core::nodes::swarm::NetworkBehaviourAction<libp2p_floodsub::protocol::FloodsubRpc, 
libp2p_floodsub::protocol::FloodsubMessage>>` in the current scope
  --> src/chat.rs:83:37
   |                                                                                                                                                                                                                
83 | match self.swarm.poll().expect("Error while polling swarm") {
   |                                     ^^^^^^
                                                                                                                                                                                                                    
error[E0308]: mismatched types
   --> src/chat.rs:146:16
    |                                                                                                                                                                                                               
146 | swarm: swarm,
    |                ^^^^^ expected struct `libp2p::CommonTransport`, found struct `libp2p_core::transport::and_then::AndThen`
    |                                                                                                                                                                                                               
    = note: expected type `libp2p_core::nodes::swarm::Swarm<libp2p::CommonTransport, libp2p_floodsub::layer::FloodsubBehaviour<_>, _>`
               found type `libp2p_core::nodes::swarm::Swarm<libp2p_core::transport::and_then::AndThen<libp2p_core::transport::upgrade::Upgrade<libp2p::CommonTransport, libp2p_secio::SecioConfig>, 
[closure@src/chat.rs:105:19: 109:10]>, 
libp2p_floodsub::layer::FloodsubBehaviour<libp2p_core::muxing::SubstreamRef<std::sync::Arc<libp2p_mplex::Multiplex<rw_stream_sink::RwStreamSink<futures::stream::map_err::MapErr<libp2p_secio::SecioMiddleware<libp2p_core::either::EitherOutput<libp2p_tcp_transport::TcpTransStream, 
std::boxed::Box<(dyn websocket::stream::async::Stream + std::marker::Send + 'static)>>>, fn(libp2p_secio::error::SecioError) -> std::io::Error>>>>>>, libp2p_core::topology::MemoryTopology>`
                                                                                                                                                                                                                    
error: aborting due to 3 previous errors
                                                                                                                                                                                                                    
Some errors occurred: E0308, E0599.  For more information about an error, try `rustc --explain E0308`.  error: Could not compile `learn-libp2p`.

To learn more, run the command again with --verbose.



*/
