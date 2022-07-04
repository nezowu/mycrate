pub fn type_of_ptr<T>(_: &T) -> &'static str {
  std::any::type_name::<T>()
}

pub fn type_of<T>(_: T) -> &'static str {
  std::any::type_name::<T>()
}
