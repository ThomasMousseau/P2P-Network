use libp2p::{
    // swarm::{NetworkBehaviourEventProcess, Swarm, SwarmBuilder},
            tcp::TokioTcpConfig, NetworkBehaviour, PeerId, Transport, identity
};
use serde::{Deserialize, Serialize};
use tokio::{fs, io::AsyncBufReadExt, sync::mpsc};
use once_cell::sync::Lazy;
use log::{error, info};

static KEYS: Lazy<identity::Keypair> = Lazy::new(|| identity::Keypair::generate_ed25519());
static PEER_ID: Lazy<PeerId> = Lazy::new(|| PeerId::from(KEYS.public()));

#[derive(Debug, Serialize, Deserialize)]
struct Wizard {
    id: u32,
    name: String,
    magic_type: String,
    has_grimoire: bool
}


#[tokio::main]
async fn main() {
    pretty_env_logger::init();
    println!("START OF NETWORK!");
    info!("Peer Id: {}", PEER_ID.clone()); 

    // let auth_keys = Keypair::<X25519Spec>::new()
    //     .into_authentic(&KEYS)
    //     .expect("can create auth keys");
}