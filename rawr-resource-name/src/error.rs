use thiserror::Error;

/// these are the possible errors that might occur when parsing or constructing
/// a `ResourceName`.
// TODO(nick): at some point, we should probably make these a little less "yo
// dawg" and a little more sad, er - professional. at some point, eventually.
#[derive(Debug, Error, PartialEq)]
pub enum ResourceNameError {
    #[error("resource name is empty")]
    Empty,

    #[error("invalid resource name segment count: expected {expected} segments, found {found}")]
    InvalidSegmentCount { expected: usize, found: usize },

    #[error("missing resource name prefix")]
    EmptyPrefix,

    #[error("missing partition in resource name")]
    EmptyPartition,

    #[error("missing service in resource name")]
    EmptyService,

    #[error("missing qualified resource path")]
    EmptyQualifiedResourcePath,

    #[error("missing resource type")]
    EmptyResourceType,

    #[error("missing resource path")]
    EmptyResourcePath,
}
