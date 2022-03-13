/// An abstraction of complex node.
pub trait AbstractComplexNode {
	/// The type that represents the complex' name.
	type Complex;

	/// The type that represents the complex' attacher(s).
	type Attachers;

	/// The type that represents the complex' nodes(s).
	type Nodes;

	/// Returns the name of the complex node.
	fn name(&self) -> &Self::Complex;

	/// Returns the attachers of the complex node.
	fn attachers(&self) -> &Self::Attachers;

	/// Returns the nodes in the complex node.
	fn nodes(&self) -> &Self::Nodes;

	/// Consumes the complex node into tuple.
	fn consume(self) -> (Self::Complex, Self::Attachers, Self::Nodes);
}
