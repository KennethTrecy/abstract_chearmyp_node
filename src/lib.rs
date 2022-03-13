#![cfg_attr(feature = "no_std", no_std)]

//! # Abstract Chearmyp Node
//! Please read the README.md for more information.
//!
//! ## Features available
//! - `no_std`: Uses the `core` crate instead of `std` crate.
#![cfg_attr(
	feature = "no_std",
	doc = "- `vecdeque_node_queue`: Implements [AbstractNodeQueue]
												for [alloc::collections::VecDeque].",
)]
#![cfg_attr(
	not(feature = "no_std"),
	doc = "- `vecdeque_node_queue`: Implements [AbstractNodeQueue]
												for [std::collections::VecDeque].",
)]
#![cfg_attr(
	feature = "no_std",
	doc = "- `vecdeque_attacher_collection`: Implements [AbstractAttacherCollection]
												for [alloc::collections::VecDeque].",
)]
#![cfg_attr(
	not(feature = "no_std"),
	doc = "- `vecdeque_attacher_collection`: Implements [AbstractAttacherCollection]
												for [std::collections::VecDeque].",
)]

#[cfg(feature = "no_std")]
extern crate alloc;

mod native {
	#[cfg(feature = "no_std")]
	pub use alloc::{
		vec::Vec,
		collections::VecDeque
	};

	#[cfg(feature = "no_std")]
	pub use core::{
		ops::Fn
	};

	#[cfg(not(feature = "no_std"))]
	pub use std::{
		vec::Vec,
		collections::VecDeque,
		ops::Fn
	};
}

mod abstracts {
	pub use abstract_chearmyp_boundary::{AbstractBoundary, AbstractBoundaryCollection};
}

mod node_kind;
mod abstract_node;
mod specific_abstract_nodes;
mod abstract_node_queue;
mod dynamic_abstract_node;
mod abstract_attacher_collection;

pub use node_kind::NodeKind;
pub use abstract_node::{AbstractNode, SimpleAbstractNode};
pub use abstract_node_queue::AbstractNodeQueue;
pub use dynamic_abstract_node::DynamicAbstractNode;
pub use abstract_attacher_collection::AbstractAttacherCollection;
pub use specific_abstract_nodes::{
	AbstractSimplexNode,
	AbstractComplexNode,
	AbstractAttacherNode,
	AbstractLineCommentNode,
	AbstractBlockCommentNode,
	AbstractLineOthertongueNode,
	AbstractBlockOthertongueNode
};
