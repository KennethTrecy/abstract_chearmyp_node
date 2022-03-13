use crate::AbstractSimplexNode;
use super::DynamicAbstractNode;

impl<T, U, V> AbstractSimplexNode for V
where
	V: DynamicAbstractNode<Name = T, Attachers = U> {
	type Simplex = T;
	type Attachers = U;

	fn name(&self) -> &Self::Simplex { DynamicAbstractNode::name(self) }

	fn attachers(&self) -> &Self::Attachers { DynamicAbstractNode::attachers(self) }

	fn consume(self) -> (Self::Simplex, Self::Attachers) {
		DynamicAbstractNode::consume_simplex(self)
	}
}
