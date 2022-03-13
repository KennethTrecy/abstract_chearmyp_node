use crate::AbstractAttacherNode;
use super::DynamicAbstractNode;

impl<T, U, V, W> AbstractAttacherNode for W
where
	W: DynamicAbstractNode<Label = T, Content = U, Comments = V> {
	type Label = T;
	type Content = U;
	type Comments = V;

	fn label(&self) -> &Self::Label { DynamicAbstractNode::label(self) }

	fn content(&self) -> &Self::Content { DynamicAbstractNode::content(self) }

	fn comments(&self) -> &Self::Comments { DynamicAbstractNode::comments(self) }

	fn consume(self) -> (Self::Label, Self::Content, Self::Comments) {
		DynamicAbstractNode::consume_attacher(self)
	}
}
