use std::fmt::Display;

pub trait Output<T>: Display + From<T> {}
