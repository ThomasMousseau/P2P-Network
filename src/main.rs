use libp2p::{
    core::upgrade,
    swarm::{NetworkBehaviourEventProcess, Swarm, SwarmBuilder},
    tcp::{TokioTcpConfig}, 
    NetworkBehaviour, 
    PeerId, 
    Transport,
    identity,
    mdns::{Mdns, MdnsEvent},
    mplex,
    noise::{Keypair, NoiseConfig, X25519Spec},
    floodsub::{Floodsub, FloodsubEvent, Topic},
};

use serde::{Deserialize, Serialize};
use tokio::{fs, io::AsyncBufReadExt, sync::mpsc};
use once_cell::sync::Lazy;
use log::{error, info};

static KEYS: Lazy<identity::Keypair> = Lazy::new(|| identity::Keypair::generate_ed25519());
static PEER_ID: Lazy<PeerId> = Lazy::new(|| PeerId::from(KEYS.public()));
static TOPIC: Lazy<Topic> = Lazy::new(|| Topic::new("wizards"));

#[derive(Debug, Serialize, Deserialize)]
struct Wizard {
    id: u32,
    name: String,
    magic_type: String,
    has_grimoire: bool
}

#[derive(NetworkBehaviour)]
struct NodeBehaviour {
    floodsub: Floodsub,
    mdns: Mdns,
    #[behaviour(ignore)]
    response_sender: mpsc::UnboundedSender<ListResponse>,
}

#[derive(Debug, Serialize, Deserialize)]
enum ListMode {
    ALL,
    One(String),
}

#[derive(Debug, Serialize, Deserialize)]
struct ListRequest {
    mode: ListMode,
}

#[derive(Debug, Serialize, Deserialize)]
struct ListResponse {
    mode: ListMode,
    data: Wizard,
    receiver: String,
}



#[tokio::main]
async fn main() {
    pretty_env_logger::init();

    info!("Peer Id: {}", PEER_ID.clone());
    let (response_sender, mut response_rcv) = mpsc::unbounded_channel();

    println!("START OF NETWORK!");
    info!("Peer Id: {}", PEER_ID.clone()); 

    //Noise protocle, used to secre the connection https://noiseprotocol.org/
    let auth_keys = Keypair::<X25519Spec>::new()
        .into_authentic(&KEYS)
        .expect("can create auth keys");

    //Transport layer
    let transport = TokioTcpConfig::new()
        .nodelay(true)
        .upgrade(upgrade::Version::V1)
        .authenticate(NoiseConfig::xx(auth_keys).into_authenticated())
        .multiplex(mplex::MplexConfig::new())
        .boxed();

    // let transport_v2 = TokioTcpConfig::new()
    //     .upgrade(upgrade::Version::V1)
    //     .authenticate(NoiseConfig::xx(auth_keys).into_authenticated())
    //     .multiplex(mplex::MplexConfig::new())
    //     .boxed();

    // NetworkBehaviour, used to define the behaviour of the nodes and peers
    let mut behaviour = NodeBehaviour {
        floodsub: Floodsub::new(PEER_ID.clone()),
        mdns: TokioMdns::new().expect("can create mdns"),
        response_sender,
    };

    behaviour.floodsub.subscribe(TOPIC.clone());

    // Swarm, used to manage the network https://docs.rs/libp2p/latest/libp2p/swarm/index.html
    let mut swarm = SwarmBuilder::new(transport, behaviour, PEER_ID.clone())
        .executor(Box::new(|fut| {
            tokio::spawn(fut);
        })).build();

    Swarm::listen_on(
        &mut swarm,
        "/ip4/0.0.0.0/tcp/0"
            .parse()
            .expect("can get a local socket"),
    )
    .expect("swarm can be started");

}