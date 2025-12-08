use thiserror::Error;

use rawr_resource_name::ResourceNameError;

/// top-level error enum for rawr.
#[derive(Debug, Error)]
pub enum RawrError {
    /// errors of the "not found" variety
    #[error("not found")]
    NotFound,

    /// errors of the "this is an our bad" variety
    #[error("internal error: {0}")]
    Internal(String),

    /// errors of the ResourceName variety
    #[error("resource name error: {0}")]
    ResourceName(#[from] ResourceNameError),
}

#[cfg(test)]
mod tests {
    use super::*;
    use rawr_resource_name::ResourceNameError;

    #[test]
    fn error_wrapping() {
        // create a rawr-{crate} specific error
        let underlying_error = ResourceNameError::InvalidSegmentCount {
            expected: 4,
            found: 20,
        };

        // coerce our underlying error into the top-level rawr error
        let user_facing_error: RawrError = underlying_error.into();

        // verify the wrapping worked as expected
        assert!(matches!(
            user_facing_error,
            RawrError::ResourceName(ResourceNameError::InvalidSegmentCount { .. })
        ));

        // verify the internals can be extracted for more specific error
        // handling, (e.g. if we want to handle underlying errors differently).
        if let RawrError::ResourceName(ResourceNameError::InvalidSegmentCount { expected, found }) =
            user_facing_error
        {
            assert_eq!(expected, 4);
            assert_eq!(found, 20);
        }
    }
}
