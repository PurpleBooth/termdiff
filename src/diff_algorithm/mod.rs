//! Module for diff algorithm implementations
//!
//! This module contains the implementations of different diff algorithms,
//! as well as the common types and traits used by them.

pub mod common;
pub mod factory;
#[cfg(feature = "myers")]
pub mod myers;
#[cfg(feature = "similar")]
pub mod similar;

// Re-export the common types and traits
pub use common::{Algorithm, ChangeTag, DiffAlgorithm};

// Re-export the algorithm implementations
#[cfg(feature = "myers")]
pub use myers::MyersDiff;
#[cfg(feature = "similar")]
pub use similar::SimilarDiff;

// Re-export the factory
pub use factory::DiffAlgorithmFactory;
