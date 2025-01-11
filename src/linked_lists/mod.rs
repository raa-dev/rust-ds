// mod double;
mod errors;
mod node;
mod singly;
mod traits;

// pub(crate) use singly::Singly;
// pub(crate) use double::Double;

pub(super) use errors::{Error, Result};
pub(super) use node::SNode;
pub(super) use traits::{List, SinglyNode};
