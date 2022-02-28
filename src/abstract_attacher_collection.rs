/// An abstraction of Chearmyp attacher collection.
pub trait AbstractAttacherCollection<T, U> {
	/// Creates an empty attacher collection.
	fn new() -> Self;

	/// Finds the mutable attacher.
	fn get(&self, _: T) -> Option<&U>;

	/// Finds the mutable attacher.
	fn get_mut(&mut self, _: T) -> Option<&mut U>;

	/// Puts the attacher into the attacher collection.
	fn attach(&mut self, _: U);

	/// Takes the attacher in the attacher collection.
	fn deattach(&mut self, _: T) -> Option<U>;
}

#[cfg(any(feature = "vecdeque_attacher_collection", test))]
mod vec_deque;
