use petgraph;
use std::cell;
use std::collections::HashMap;

// core types
pub use common::*;
pub use noria::internal::*;
pub use ops::NodeOperator;
pub use petgraph::graph::NodeIndex;
pub use processing::Ingredient;
pub(crate) use processing::{Miss, ProcessingResult, RawProcessingResult, ReplayContext};

// graph types
pub use node::Node;
pub type Edge = ();
pub type Graph = petgraph::Graph<Node, Edge>;

// dataflow types
pub use noria::debug::trace::{Event, PacketEvent, Tracer};
pub use noria::Input;
pub use payload::{ExternalId, Packet, PacketId, ReplayPathSegment, SourceChannelIdentifier};
pub use Sharding;

// domain local state
pub use state::{LookupResult, MemoryState, PersistentState, RecordResult, Row, State};
pub type StateMap = Map<Box<State>>;
pub type DomainNodes = Map<cell::RefCell<Node>>;
pub type ReplicaAddr = (DomainIndex, usize);

// persistence configuration
pub use DurabilityMode;
pub use PersistenceParameters;

use fnv::FnvHashMap;
use std::collections::VecDeque;
pub type EnqueuedSends = FnvHashMap<ReplicaAddr, VecDeque<Box<Packet>>>;

/// Channel coordinator type specialized for domains
pub type ChannelCoordinator = noria::channel::ChannelCoordinator<(DomainIndex, usize), Box<Packet>>;
pub trait Executor {
    fn ack(&mut self, tag: SourceChannelIdentifier);
    fn create_universe(&mut self, req: HashMap<String, DataType>);
}
