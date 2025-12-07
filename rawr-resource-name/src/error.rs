use thiserror::Error;

/// these are the possible errors that might occur when parsing or constructing
/// a `ResourceName`.
// TODO(nick): at some point, we should probably make these a little less "yo
// dawg" and a little more sad, er - professional. at some point, eventually.
#[derive(Debug, Error, PartialEq)]
pub enum ResourceNameError {
    #[error("yo dawg, you just straight up forgot to provide a resource name")]
    Empty,

    #[error(
        "yo dawg, you provided an invalid number of resource name segments; expected {expected} segments, found {found}"
    )]
    InvalidSegmentCount { expected: usize, found: usize },

    #[error("yo dawg, you forgot to provide a valid prefix")]
    EmptyPrefix,

    #[error("yo dawg, you forgot to provide a valid partition")]
    EmptyPartition,

    #[error("yo dawg, you forgot to provide a valid service")]
    EmptyService,

    #[error("yo dawg, you forgot to provide a valid qualified resource path")]
    EmptyQualifiedResourcePath,

    #[error("yo dawg, you forgot to provide a valid resource type")]
    EmptyResourceType,

    #[error("yo dawg, you forgot to provide a valid resource path")]
    EmptyResourcePath,
}
