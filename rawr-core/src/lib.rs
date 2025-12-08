//! rawr-core
//!
//! provides core primitives for the rawr ecosystem.

// internal modules
mod ids;
mod pap;
mod pdp;
mod policy;

// re-exports which allows for UX such as `use rawr_core::PolicyStore;` instead
// of `use rawr_core::policy_store::PolicyStore;`
pub use ids::*;
pub use pap::*;
pub use pdp::*;
pub use policy::*;
