use svix_ksuid::Ksuid;

/// unique identifier for principals (users, groups, roles).
/// we're using strings here for flexibility, as different systems
/// may have different formats for principal identifiers.
pub type PrincipalId = String;

/// tenant identifier type. None indicates a global or single-tenant context.
/// we want to be able to support multi-tenant use-cases without forcing it onto
/// single-tenant implementations. there's like, no rules here dude.
pub type TenantId = Option<String>;

/// unique identifier for rawr resources.
/// globally unique and sorted strings is a cornerstone of many rawr data
/// models. ksuids are impecable for this use-case. using ksuids allows us to
/// balance highly-performant, globally unique identifiers without forcing too
/// many implementation specific contraints onto callers.
pub type RawrId = Ksuid;
