use crate::AbstractComplexNode;
use super::DynamicAbstractNode;

impl<T, U, V, W> AbstractComplexNode for W
where
	W: DynamicAbstractNode<Name = T, Attachers = U, Nodes = V> {
	type Complex = T;
	type Attachers = U;
	type Nodes = V;

	fn name(&self) -> &Self::Complex { DynamicAbstractNode::name(self) }

	fn attachers(&self) -> &Self::Attachers { DynamicAbstractNode::attachers(self) }

	fn nodes(&self) -> &Self::Nodes { DynamicAbstractNode::nodes(self) }

	fn consume(self) -> (Self::Complex, Self::Attachers, Self::Nodes) {
		DynamicAbstractNode::consume_complex(self)
	}
}
