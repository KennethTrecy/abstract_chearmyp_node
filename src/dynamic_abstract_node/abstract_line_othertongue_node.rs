use crate::AbstractLineOthertongueNode;
use super::DynamicAbstractNode;

impl<T, U> AbstractLineOthertongueNode for U
where
	U: DynamicAbstractNode<Line = T> {
	type Line = T;

	fn line(&self) -> &Self::Line { DynamicAbstractNode::line(self) }
}
