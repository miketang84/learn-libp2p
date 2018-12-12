
extern crate futures;
extern crate libp2p;
extern crate tokio;

use futures::prelude::*;
use libp2p::{
    Transport,
    core::upgrade::{self, OutboundUpgradeExt},
    secio,
    mplex,
    tokio_codec::{FramedRead, LinesCodec}
};

fn main() {
    // Create a random PeerId
    let local_key = secio::SecioKeyPair::ed25519_generated().unwrap();
    let local_peer_id = local_key.to_peer_id();
    println!("Local peer id: {:?}", local_peer_id);

    // Set up a an encrypted DNS-enabled TCP Transport over the Mplex protocol
    let transport = libp2p::CommonTransport::new()
        .with_upgrade(secio::SecioConfig::new(local_key))
        .and_then(move |out, _| {
            let kad_upgrade = libp2p::kad::KadConnecConfig::new();
            upgrade::apply_inbound(out.stream, kad_upgrade).map_err(|e| e.into_io_error())
        })
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

    // Kick it off
    tokio::run(futures::future::poll_fn(move || -> Result<_, ()> {
        loop {
            match framed_stdin.poll().expect("Error while polling stdin") {
                Async::Ready(Some(line)) => swarm.publish(&floodsub_topic, line.as_bytes()),
                Async::Ready(None) => panic!("Stdin closed"),
                Async::NotReady => break,
            };
        }

        loop {
            match swarm.poll().expect("Error while polling swarm") {
                Async::Ready(Some(message)) => {
                    println!("Received: {:?}", String::from_utf8_lossy(&message.data));
                },
                Async::Ready(None) | Async::NotReady => break,
            }
        }

        Ok(Async::NotReady)
    }));
}
