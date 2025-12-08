use std::sync::Arc;

use crate::PolicyId;

/// the effect of a policy statement determines whether the statement
/// explicitly allows or denies the requested operation.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Effect {
    Allow,
    Deny,
}

/// a policy defines a set of permissions that can be attached to
/// principals (users, groups, roles).
pub trait Policy: Send + Sync {
    /// unique identifier for the policy
    fn id(&self) -> PolicyId;

    /// name of the policy
    fn name(&self) -> &str;

    /// optional description of the policy
    fn description(&self) -> Option<&str>;

    /// statements defined in the policy
    fn statements(&self) -> &[Arc<dyn PolicyStatement>];
}

/// a policy statement defines a single permission within a policy.
/// it specifies the effect (allow or deny), the actions, and the resources
/// to which the statement applies.
pub trait PolicyStatement: Send + Sync {
    /// effect of the policy statement (allow or deny)
    fn effect(&self) -> Effect;

    /// actions to which the policy statement applies
    fn actions(&self) -> &[String];

    /// resources to which the policy statement applies
    fn resources(&self) -> &[String];
}
