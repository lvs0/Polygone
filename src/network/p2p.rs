use libp2p::{identity, PeerId, Swarm, swarm::SwarmEvent, kad::Kademlia, kad::KademliaEvent, kad::store::MemoryStore}; use std::error::Error; use futures::StreamExt; use std::collections::HashSet; use std::time::Duration; /// Represents a Polygone network node with DHT capabilities.
/// Implements proper node discovery and DHT functionality.
use libp2p::{identity, PeerId, Swarm, swarm::SwarmEvent, kad::Kademlia, kad::KademliaEvent, kad::store::MemoryStore};
use std::error::Error;
use futures::StreamExt;
use std::collections::HashSet;
use std::time::Duration;

/// Represents a Polygone network node with DHT capabilities.
pub struct PolygoneNode {
    swarm: Swarm<Kademlia<MemoryStore>>,
    peer_id: PeerId,
    known_peers: HashSet<PeerId>,
}

impl PolygoneNode {
    /// Creates a new PolygoneNode with the given identity.
    pub async fn new(identity: identity::Keypair) -> Result<Self, Box<dyn Error>> {
        let peer_id = PeerId::from(identity.public());
        let store = MemoryStore::new(peer_id);
        let kademlia = Kademlia::new(peer_id, store);
        let transport = libp2p::development_transport(identity).await?;
        let behaviour = kademlia;
        let mut swarm = Swarm::new(transport, behaviour, peer_id);
        
        // Add bootstrap nodes here
        Ok(PolygoneNode {
            swarm,
            peer_id,
            known_peers: HashSet::new(),
        })
    }
    
    /// Starts the node and runs the event loop.
    pub async fn run(&mut self) {
        loop {
            match self.swarm.select_next_some().await {
                SwarmEvent::Behaviour(KademliaEvent::OutboundQueryProgressed { result, .. }) => {
                    if let Ok(Ok(peer)) = result {
                        self.known_peers.insert(peer);
                    }
                }
                _ => {}
            }
        }
    }
    
    /// Gets the current list of known peers.
    pub fn known_peers(&self) -> Vec<PeerId> {
        self.known_peers.iter().cloned().collect()
    }
} pub struct PolygoneNode {     swarm: Swarm<Kademlia<MemoryStore>>,     peer_id: PeerId,     known_peers: HashSet<PeerId>, } impl PolygoneNode {     /// Creates a new PolygoneNode with the given identity.     pub async fn new(identity: identity::Keypair) -> Result<Self, Box<dyn Error>> {         let peer_id = PeerId::from(identity.public());         let store = MemoryStore::new(peer_id);         let kademlia = Kademlia::new(peer_id, store);         let transport = libp2p::development_transport(identity).await?;         let behaviour = kademlia;         let mut swarm = Swarm::new(transport, behaviour, peer_id);         // Add bootstrap nodes here         Ok(PolygoneNode {             swarm,             peer_id,             known_peers: HashSet::new(),         })     }     /// Starts the node and runs the event loop.     pub async fn run(&mut self) {         loop {             match self.swarm.select_next_some().await {                 SwarmEvent::Behaviour(KademliaEvent::OutboundQueryProgressed { result, .. }) => {                     if let Ok(Ok(peer)) = result {                         self.known_peers.insert(peer);                     }                 }                 _ => {}             }         }     }     /// Gets the current list of known peers.     pub fn known_peers(&self) -> Vec<PeerId> {         self.known_peers.iter().cloned().collect()     } }