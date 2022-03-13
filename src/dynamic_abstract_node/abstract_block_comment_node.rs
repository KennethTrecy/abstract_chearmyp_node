use crate::AbstractBlockCommentNode;
use super::DynamicAbstractNode;

impl<T, U> AbstractBlockCommentNode for U
where
	U: DynamicAbstractNode<Block = T> {
	type Block = T;

	fn block(&self) -> &Self::Block { DynamicAbstractNode::block(self) }

	fn consume(self) -> Self::Block {
		DynamicAbstractNode::consume_block(self)
	}
}
