/// An abstraction of line othertongue node.
pub trait AbstractLineOthertongueNode {
	/// The line in line othertongue node which could be a string, array of bytes, or other type.
	type Line;

	/// Returns the line in line othertongue node.
	fn line(&self) -> &Self::Line;

	/// Consumes the line othertongue node into line.
	fn consume(self) -> Self::Line;
}
