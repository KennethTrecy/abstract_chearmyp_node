/// An abstraction of simplex node.
pub trait AbstractSimplexNode {
	/// The type that represents the simplex' name.
	type Simplex;

	/// The type that represents the simplex' attacher(s).
	type Attachers;

	/// Returns the name of the simplex node.
	fn name(&self) -> &Self::Simplex;

	/// Returns the attachers of the simplex node.
	fn attachers(&self) -> &Self::Attachers;
}
