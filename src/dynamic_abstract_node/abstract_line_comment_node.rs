use crate::AbstractLineCommentNode;
use super::DynamicAbstractNode;

impl<T, U> AbstractLineCommentNode for U
where
	U: DynamicAbstractNode<Line = T> {
	type Line = T;

	fn line(&self) -> &Self::Line { DynamicAbstractNode::line(self) }

	fn consume(self) -> Self::Line {
		DynamicAbstractNode::consume_line(self)
	}
}
