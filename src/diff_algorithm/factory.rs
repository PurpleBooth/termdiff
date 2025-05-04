use super::{Algorithm, DiffAlgorithm};

#[cfg(feature = "myers")]
use super::MyersDiff;
#[cfg(feature = "similar")]
use super::SimilarDiff;

/// Factory for creating diff algorithm instances
#[derive(Debug)]
pub struct DiffAlgorithmFactory;

impl DiffAlgorithmFactory {
    /// Creates a new diff algorithm instance based on the specified algorithm
    ///
    /// # Panics
    ///
    /// Panics if the requested algorithm is not available due to being disabled
    /// via feature flags.
    pub fn create(algorithm: Algorithm) -> Box<dyn DiffAlgorithm> {
        match algorithm {
            #[cfg(feature = "similar")]
            Algorithm::Similar => Box::new(SimilarDiff),
            #[cfg(feature = "myers")]
            Algorithm::Myers => Box::new(MyersDiff),
            #[cfg(not(feature = "similar"))]
            Algorithm::Similar => panic!(
                "Similar algorithm is not available. Enable the 'similar' feature to use it."
            ),
            #[cfg(not(feature = "myers"))]
            Algorithm::Myers => {
                panic!("Myers algorithm is not available. Enable the 'myers' feature to use it.")
            }
        }
    }
}
