use std::sync::mpsc::Sender;
use crate::NodeRef;

pub(super) enum NodeUpdateTask {
    UpdateSuccessor(NodeRef, Sender<()>),
    UpdatePredecessor(NodeRef, Sender<()>),
    UnsetPredecessor(Sender<()>),
}
