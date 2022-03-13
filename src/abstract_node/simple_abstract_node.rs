use crate::abstracts::{AbstractBoundary, AbstractBoundaryCollection};
use crate::{NodeKind, AbstractNodeQueue, AbstractAttacherCollection};

/// An simple abstraction of Chearmyp node.
///
/// Unlike [AbstractNode], [SimpleAbstractNode] has fewer template arguments. It assumes that the
/// innermost contents of [AbstractBoundaryCollection] will have the same type as the
/// [AbstractBoundary]. Implementing this trait automatically implements the [AbstractNode].
///
/// [AbstractNode]: crate::AbstractNode
pub trait SimpleAbstractNode<T, U, V, W, X, Y>
where
	U: AbstractBoundary<T>,
	V: AbstractBoundaryCollection<T, U>,
	X: AbstractAttacherCollection<W>,
	Y: AbstractNodeQueue<W> {
	/// Returns the kind of the node it holds.
	fn kind(&self) -> NodeKind;

	/// Creates new line comment node.
	fn new_line_comment(_: U) -> Self;

	/// Creates new line othertongue node.
	fn new_line_othertongue(_: U) -> Self;

	/// Creates new block comment node.
	fn new_block_comment(_: V) -> Self;

	/// Creates new block othertongue node.
	fn new_block_othertongue(_: V) -> Self;

	/// Creates new attacher node.
	fn new_attacher(_: U, _: U, _: V) -> Self;

	/// Creates new simplex node.
	fn new_simplex(_: U, _: X) -> Self;

	/// Creates new complex node.
	fn new_complex(_: U, _: X, _: Y) -> Self;
}

use super::AbstractNode;

impl<T, U, V, W, X, Y, Z> AbstractNode<T, U, T, U, V, W, X, W, Y> for Z
where
	U: AbstractBoundary<T>,
	V: AbstractBoundaryCollection<T, U>,
	X: AbstractAttacherCollection<W>,
	Y: AbstractNodeQueue<W>,
	Z: SimpleAbstractNode<T, U, V, W, X, Y> {
	fn kind(&self) -> NodeKind { SimpleAbstractNode::kind(self) }

	fn new_line_comment(line: U) -> Self { Self::new_line_comment(line) }

	fn new_line_othertongue(line: U) -> Self { Self::new_line_othertongue(line) }

	fn new_block_comment(block: V) -> Self { Self::new_block_comment(block) }

	fn new_block_othertongue(block: V) -> Self { Self::new_block_othertongue(block) }

	fn new_attacher(label: U, content: U, comments: V) -> Self {
		Self::new_attacher(label, content, comments)
	}

	fn new_simplex(concept: U, attachers: X) -> Self { Self::new_simplex(concept, attachers) }

	fn new_complex(concept: U, attachers: X, nodes: Y) -> Self {
		Self::new_complex(concept, attachers, nodes)
	}
}
