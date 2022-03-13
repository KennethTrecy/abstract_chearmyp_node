use crate::native::Fn;

/// An abstraction of Chearmyp attacher collection.
pub trait AbstractAttacherCollection<T> {
	/// Creates an empty attacher collection.
	fn new() -> Self;

	/// Finds the mutable attacher with the help of closure.
	fn get<U: Fn(&T) -> bool>(&self, _: U) -> Option<&T>;

	/// Finds the mutable attacher with the help of closure..
	fn get_mut<U: Fn(&T) -> bool>(&mut self, _: U) -> Option<&mut T>;

	/// Puts the attacher into the attacher collection.
	fn attach(&mut self, _: T);

	/// Takes the attacher in the attacher collection with the help of closure..
	fn deattach<U: Fn(&T) -> bool>(&mut self, _: U) -> Option<T>;
}

#[cfg(any(feature = "vecdeque_attacher_collection", test))]
mod vec_deque;
