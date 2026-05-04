// Comportements P2P (Kademlia, gossipsub, etc.) - bouchon temporaire pour compilation.
// TODO: réactiver libp2p correctement lors de l'Étape 2.

#[derive(Debug)]
pub struct PolygoneBehaviour;

impl PolygoneBehaviour {
    pub fn new(_local_key: &libp2p::identity::Keypair) -> Self {
        Self
    }
}
