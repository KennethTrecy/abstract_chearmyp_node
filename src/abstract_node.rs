use crate::abstracts::{AbstractBoundary, AbstractBoundaryCollection};
use crate::{NodeKind, AbstractNodeQueue, AbstractAttacherCollection};

/// An abstraction of Chearmyp node.
pub trait AbstractNode<T, U, V, W, X, Y, Z, A, B, C>
where
	U: AbstractBoundary<T>,
	W: AbstractBoundary<V>,
	X: AbstractBoundaryCollection<V, W>,
	A: AbstractAttacherCollection<Y, Z>,
	C: AbstractNodeQueue<B> {
	/// Returns the kind of the node it holds.
	fn kind(&self) -> NodeKind;

	/// Creates new line comment node.
	///
	/// First parameter is the boundary of line comment node.
	fn new_line_comment(_: U) -> Self;

	/// Creates new line othertongue node.
	///
	/// First parameter is the boundary of line othertongue node.
	fn new_line_othertongue(_: U) -> Self;

	/// Creates new block comment node.
	///
	/// First parameter is the source collection for block comment node.
	fn new_block_comment(_: X) -> Self;

	/// Creates new attacher node.
	///
	/// First parameter is the label; the second parameter is the content; and third parameter are
	/// the comments to the attacher.
	fn new_attacher(_: U, _: U, _: X) -> Self;

	/// Creates new block othertongue node.
	///
	/// First parameter is the source collection for block othertongue node.
	fn new_block_othertongue(_: X) -> Self;

	/// Creates new simplex node.
	///
	/// First parameter is the concept name, and the second parameter contains the attachers for the
	/// concept.
	fn new_simplex(_: U, _: A) -> Self;

	/// Creates new complex node.
	///
	/// First parameter is the concept name; second parameter contains the attachers for the concept;
	/// and third parameter contains other subconcepts that makes the main concept.
	fn new_complex(_: U, _: A, _: C) -> Self;
}
