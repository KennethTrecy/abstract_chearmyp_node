/// An abstraction of dynamic Chearmyp node.
///
/// Assumes that the node can represent any specific node. The methods can panic if the they trying
/// to do an invalid operation. For example, getting the label from a simplex node must panic.
/// Getting the label can only be done for an attacher node.
///
/// Automatically implements [AbstractComplexNode], [AbstractSimplexNode], [AbstractAttacherNode],
/// [AbstractLineCommentNode], [AbstractBlockCommentNode], [AbstractLineOthertongueNode], and
/// [AbstractBlockOthertongueNode].
///
/// [AbstractComplexNode]: crate::AbstractComplexNode
/// [AbstractSimplexNode]: crate::AbstractSimplexNode
/// [AbstractAttacherNode]: crate::AbstractAttacherNode
/// [AbstractLineCommentNode]: crate::AbstractLineCommentNode
/// [AbstractBlockCommentNode]: crate::AbstractBlockCommentNode
/// [AbstractLineOthertongueNode]: crate::AbstractLineOthertongueNode
/// [AbstractBlockOthertongueNode]: crate::AbstractBlockOthertongueNode
pub trait DynamicAbstractNode {
	/// The type that represents the simplex' name or complex' name.
	type Name;

	/// The type that represents the simplex' or complex' attacher(s).
	type Attachers;

	/// The type that represents the complex' nodes(s).
	type Nodes;

	/// The type of line in the line comment node or line othertongue node.
	type Line;

	/// The type of block in the block comment node or block othertongue node.
	type Block;

	/// The type of label in the attacher node.
	type Label;

	/// The type of content in the attacher node.
	type Content;

	/// The type of comments in the attacher node.
	type Comments;

	/// Returns the name of the simplex node or complex node.
	fn name(&self) -> &Self::Name;

	/// Returns the attachers of the simplex node or complex node.
	fn attachers(&self) -> &Self::Attachers;

	/// Returns the nodes in the complex node.
	fn nodes(&self) -> &Self::Nodes;

	/// Returns the line in line comment node or line othertongue node.
	fn line(&self) -> &Self::Line;

	/// Returns the block in the block comment node or block othertongue node.
	fn block(&self) -> &Self::Block;

	/// Returns the label of the attacher node.
	fn label(&self) -> &Self::Label;

	/// Returns the content of the attacher node.
	fn content(&self) -> &Self::Content;

	/// Returns the comments to the attacher.
	fn comments(&self) -> &Self::Comments;

	/// Consumes the attacher node into tuple.
	fn consume_attacher(self) -> (Self::Label, Self::Content, Self::Comments);

	/// Consumes the block comment or othertongue node into block.
	fn consume_block(self) -> Self::Block;

	/// Consumes the simplex node into tuple.
	fn consume_simplex(self) -> (Self::Name, Self::Attachers);

	/// Consumes the complex node into tuple.
	fn consume_complex(self) -> (Self::Name, Self::Attachers, Self::Nodes);

	/// Consumes the line comment or othertongue node into line.
	fn consume_line(self) -> Self::Line;
}

mod abstract_complex_node;
mod abstract_simplex_node;
mod abstract_attacher_node;
mod abstract_line_comment_node;
mod abstract_block_comment_node;
mod abstract_line_othertongue_node;
mod abstract_block_othertongue_node;
