/// An abstraction of block othertongue node.
pub trait AbstractBlockOthertongueNode {
	/// The type of block in the block othertongue node which could be a string, array of strings,
	/// or other type.
	type Block;

	/// Returns the block in the block othertongue node.
	fn block(&self) -> &Self::Block;

	/// Consumes the block othertongue node into block.
	fn consume(self) -> Self::Block;
}
