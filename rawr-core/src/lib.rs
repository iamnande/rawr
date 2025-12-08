//! rawr-core
//!
//! provides core primitives for the rawr ecosystem.

// internal modules
mod ids;
mod policy;
mod policy_store;

// re-exports which allows for UX such as `use rawr_core::PolicyStore;` instead
// of `use rawr_core::policy_store::PolicyStore;`
pub use ids::*;
pub use policy::*;
pub use policy_store::*;
