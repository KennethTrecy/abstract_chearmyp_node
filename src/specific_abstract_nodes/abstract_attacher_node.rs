/// An abstraction of attacher node.
pub trait AbstractAttacherNode {
	/// The type of label in the attacher node.
	type Label;

	/// The type of content in the attacher node.
	type Content;

	/// The type of comments in the attacher node.
	type Comments;

	/// Returns the label of the attacher.
	fn label(&self) -> &Self::Label;

	/// Returns the content of the attacher.
	fn content(&self) -> &Self::Content;

	/// Returns the comments to the attacher.
	fn comments(&self) -> &Self::Comments;

	/// Consumes the attacher node into tuple.
	fn consume(self) -> (Self::Label, Self::Content, Self::Comments);
}
