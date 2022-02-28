/// An abstraction of Chearmyp node queue.
pub trait AbstractNodeQueue<T> {
	/// Creates an empty node queue.
	fn new() -> Self;

	/// Puts the latest node into the node queue.
	fn push_node(&mut self, _: T);

	/// Moves out the earlieast node in the node queue.
	fn shift_node(&mut self) -> Option<T>;
}
