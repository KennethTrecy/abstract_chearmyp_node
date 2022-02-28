/// An abstraction of block comment node.
pub trait AbstractBlockCommentNode {
	/// The type of block in the block comment node which could be a string, array of strings, or
	/// other type.
	type Block;

	/// Returns the block in the block comment node.
	fn block(&self) -> &Self::Block;
}
