use crate::gossip::NodeId;

/// Base trait for the services that use node identifiers to
/// manage subscriptions and coordinate propagation in the P2P network.
pub trait P2pService {
    /// Network node identifier.
    type NodeId: NodeId + Send + 'static;

    /// Returns the identifier of this node, or None if the node
    /// does not have a public address advertised via gossip.
    fn node_id(&self) -> Option<Self::NodeId>;
}
