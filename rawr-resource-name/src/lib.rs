//! rawr-resource-name
//!
//! provides parsing, validation, and formatting of rawr-style resource names.
//!
//! format:
//! `{prefix}:{partition}:{service}:{region}:{account_id}:{resource_type}/{resource_path}`
//!
//! example:
//! `mrn:tycho:opa:sol-belt-1:36UeVtK7fIxhHyD9Dd5gc1XSd77:member/anderson-dawes`
//!
//! resource name rules:
//! - the {prefix} must be exactly 3 lowercaseunicode alphabetic characters.
//! - the {partition}, {service}, {region}, {account_id}, and {resource_type}
//!   must be lowercase unicode alphanumeric strings.
//! - the {resource_type} and {resource_path} must be separated by a forward
//!   slash.
//! - the {resource_path} must be a valid unicode string.
//! - the combination of {resource_type} and {resource_path} are known as the
//!   "qualified resource path".
mod error;
mod resource_name;

pub use error::ResourceNameError;
pub use resource_name::ResourceName;
