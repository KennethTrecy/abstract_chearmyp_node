use crate::native::VecDeque;
use crate::AbstractNodeQueue;

/// This is only available if `vecdeque_node_queue` feature has been activated.
///
/// It implements [AbstractNodeQueue] for [VecDeque].
impl<T> AbstractNodeQueue<T> for VecDeque<T> {
	fn new() -> Self { VecDeque::new() }

	fn push_node(&mut self, node: T) {
		self.push_back(node)
	}

	fn shift_node(&mut self) -> Option<T> {
		self.pop_front()
	}
}

#[cfg(test)]
mod t {
	use super::{VecDeque, AbstractNodeQueue};

	#[derive(Debug, PartialEq)]
	struct Node<'a> { _data: &'a str }
	impl<'a> Node<'a> {
		fn new() -> Self { Node { _data: "" } }
	}

	#[test]
	fn should_push_node() {
		let mut queue = <VecDeque<Node> as AbstractNodeQueue<Node>>::new();
		let mut expected_queue = VecDeque::new();
		expected_queue.push_back(Node::new());

		queue.push_node(Node::new());

		assert_eq!(queue, expected_queue);
	}

	#[test]
	fn should_pop_none() {
		let mut queue = <VecDeque<Node> as AbstractNodeQueue<Node>>::new();
		let expected_node = None;

		let node = queue.shift_node();

		assert_eq!(node, expected_node);
	}

	#[test]
	fn should_pop_some() {
		let node = Node::new();
		let mut queue = <VecDeque<Node> as AbstractNodeQueue<Node>>::new();
		queue.push_back(node);

		let node = queue.shift_node();

		assert_eq!(node, Some(Node::new()));
	}
}
