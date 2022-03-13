/// An abstraction of line comment node.
pub trait AbstractLineCommentNode {
	/// The line in line comment node which could be a string, array of bytes, or other type.
	type Line;

	/// Returns the line in line comment node.
	fn line(&self) -> &Self::Line;

	/// Consumes the line comment node into line.
	fn consume(self) -> Self::Line;
}
