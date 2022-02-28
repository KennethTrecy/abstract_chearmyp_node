/// Possible kinds of Chearmyp nodes.
#[cfg_attr(feature = "assertable_node_kind", derive(Debug, PartialEq))]
pub enum NodeKind {
	Complex,
	Simplex,
	Attacher,
	LineComment,
	BlockComment,
	LineOthertongue,
	BlockOthertongue
}
