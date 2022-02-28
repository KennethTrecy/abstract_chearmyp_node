use crate::AbstractBlockOthertongueNode;
use super::DynamicAbstractNode;

impl<T, U> AbstractBlockOthertongueNode for U
where
	U: DynamicAbstractNode<Block = T> {
	type Block = T;

	fn block(&self) -> &Self::Block { DynamicAbstractNode::block(self) }
}
