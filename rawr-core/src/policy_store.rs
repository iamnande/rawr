use async_trait::async_trait;
use std::sync::Arc;

use crate::{Policy, PolicyId, PrincipalId, TenantId};

pub struct ListPoliciesRequest {
    pub tenant_id: TenantId,
    pub principal_id: PrincipalId,
}

pub struct ListPoliciesResponse {
    pub policies: Vec<Arc<dyn Policy>>,
}

pub struct GetPolicyRequest {
    pub tenant_id: TenantId,
    pub policy_id: PolicyId,
}

pub struct GetPolicyResponse {
    pub policy: Option<Arc<dyn Policy>>,
}

pub struct AddPolicyRequest {
    pub tenant_id: TenantId,
    pub principal_id: PrincipalId,
    pub policy: Arc<dyn Policy>,
}

pub struct AddPolicyResponse {}

pub struct DeletePolicyRequest {
    pub tenant_id: TenantId,
    pub principal_id: PrincipalId,
    pub policy_id: PolicyId,
}

pub struct DeletePolicyResponse {}

/// NOTE: we're trying to support multi-tenant use-cases, without outright
/// baking multi-tenancy into every interface. hence the optional
/// tenant_id. if you'd like to use this in a single-tenant context, you
/// can simply pass None for tenant_id.
///
/// NOTE: if you do intend to use this in a multi-tenant context, please
/// ensure that your implementation provides tenant_id innformation in
/// all relevant locations. we'll work towards making tenent_id requirements
/// available at the type-level in future iterations to help enforce this for
/// those who do wish for a guaranteed multi-tenant interface.
#[async_trait]
pub trait PolicyStore: Send + Sync {
    type Error: std::error::Error + Send + Sync + 'static;

    /// list policies for a given principal_id, and optionally filter by a
    /// tenant_id. if tenant_id is None, list policies across all tenants for
    /// the principal.
    async fn list_policies(
        &self,
        request: ListPoliciesRequest,
    ) -> Result<ListPoliciesResponse, Self::Error>;

    /// get a specific policy by its id, optionally filtered by tenant_id.
    async fn get_policy(&self, request: GetPolicyRequest)
    -> Result<GetPolicyResponse, Self::Error>;

    /// add a policy to a principal, optionally scoped to a tenant.
    async fn add_policy(&self, request: AddPolicyRequest)
    -> Result<AddPolicyResponse, Self::Error>;

    /// remove a policy from a principal, optionally scoped to a tenant.
    async fn delete_policy(
        &self,
        request: DeletePolicyRequest,
    ) -> Result<DeletePolicyResponse, Self::Error>;
}
