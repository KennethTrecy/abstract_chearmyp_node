use crate::native::VecDeque;
use crate::{AbstractAttacherNode, AbstractAttacherCollection};

/// This is only available if `vecdeque_attacher_collection` feature has been activated.
///
/// It implements [AbstractAttacherCollection] for [VecDeque].
impl<T, U> AbstractAttacherCollection<T, U> for VecDeque<U>
where
	U: AbstractAttacherNode<Label = T>,
	T: PartialEq {
	fn new() -> Self { VecDeque::new() }

	fn get(&self, label: T) -> Option<&U> {
		for attacher in self {
			if attacher.label() == &label {
				return Some(attacher);
			}
		}

		None
	}

	fn get_mut(&mut self, label: T) -> Option<&mut U> {
		for attacher in self {
			if attacher.label() == &label {
				return Some(attacher);
			}
		}

		None
	}

	fn attach(&mut self, node: U) {
		self.push_back(node);
	}

	fn deattach(&mut self, label: T) -> Option<U> {
		let mut i = 0;
		for attacher in self.iter() {
			if attacher.label() == &label {
				break;
			} else {
				i += 1;
			}
		}

		self.remove(i)
	}
}

#[cfg(test)]
mod t {
	use crate::native::Vec;
	use super::{AbstractAttacherNode, AbstractAttacherCollection, VecDeque};

	#[derive(Debug, PartialEq)]
	struct Attacher<'a> { _data: &'a str, comments: Vec<&'a str> }
	impl<'a> Attacher<'a> {
		fn new(_data: &'a str) -> Self { Attacher { _data, comments: Vec::new() } }
	}

	// Just a dummy node implementation
	impl<'a> AbstractAttacherNode for Attacher<'a> {
		type Label = &'a str;
		type Content = &'a str;
		type Comments = Vec<&'a str>;

		fn label(&self) -> &Self::Label { &self._data }

		fn content(&self) -> &Self::Content { &self._data }

		fn comments(&self) -> &Self::Comments { &self.comments }

		fn consume(self) -> (Self::Label, Self::Content, Self::Comments) {
			(self._data, self._data, self.comments)
		}
	}

	#[test]
	fn should_attach() {
		let mut queue = <VecDeque<Attacher> as AbstractAttacherCollection<&str, Attacher>>::new();
		let mut expected_queue = VecDeque::new();
		expected_queue.push_back(Attacher::new("a"));

		queue.attach(Attacher::new("a"));

		assert_eq!(queue, expected_queue);
	}

	#[test]
	fn should_deattach_none() {
		let mut queue = <VecDeque<Attacher> as AbstractAttacherCollection<&str, Attacher>>::new();
		let expected_node = None;

		let node = queue.deattach("a");

		assert_eq!(node, expected_node);
	}

	#[test]
	fn should_deattach_some() {
		let mut queue = <VecDeque<Attacher> as AbstractAttacherCollection<&str, Attacher>>::new();
		queue.push_back(Attacher::new("a"));

		let node = queue.deattach("a");

		assert_eq!(node, Some(Attacher::new("a")));
	}
}
