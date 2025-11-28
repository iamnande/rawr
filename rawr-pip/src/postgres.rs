use svix_ksuid::Ksuid;

use crate::{AcmLoader, PipError, apply_roles};
use rawr_acm::Acm;
use rawr_db::DbPool;
use rawr_pap::{Effect, Policy, Role};

pub struct PostgresPolicyLoader {
    pool: DbPool,
}

impl PostgresPolicyLoader {
    pub fn new(pool: DbPool) -> Self {
        PostgresPolicyLoader { pool }
    }
}

#[derive(sqlx::FromRow)]
struct RoleRow {
    id: String,
    account_ksuid: String,
    name: String,
    description: Option<String>,
}

#[derive(sqlx::FromRow)]
struct PolicyRow {
    id: String,
    account_ksuid: String,
    role_ksuid: String,
    effect: String,
    actions: sqlx::types::Json<Vec<String>>,
    resources: sqlx::types::Json<Vec<String>>,
}

impl AcmLoader for PostgresPolicyLoader {
    async fn load(&self, account_ksuid: &Ksuid, principal_ksuid: &Ksuid) -> Result<Acm, PipError> {
        let pool = self.pool.clone();
        let account_ksuid_str = account_ksuid.to_string();
        let principal_ksuid_str = principal_ksuid.to_string();

        // fetch them rolls
        let roles: Vec<RoleRow> = sqlx::query_as(
            r#"
            SELECT 
                r.id,
                r.account_ksuid,
                r.name,
                r.description
            FROM roles r
            INNER JOIN role_assignments ra
                ON r.id = ra.role_ksuid
               AND r.account_ksuid = ra.account_ksuid
            WHERE ra.account_ksuid = $1
              AND ra.principal_ksuid = $2
              AND r.deleted_at IS NULL
              AND ra.deleted_at IS NULL
            ORDER BY r.name
            "#,
        )
        .bind(&account_ksuid_str)
        .bind(&principal_ksuid_str)
        .fetch_all(&pool)
        .await
        .map_err(|e| {
            PipError::AcmNotFound(format!(
                "strong sad for principal, failed to fetch roles: {}",
                e
            ))
        })?;

        if roles.is_empty() {
            return Err(PipError::AcmNotFound(format!(
                "oh brother, this guy stinks! they've got no roles: {}",
                principal_ksuid_str
            )));
        }

        let mut role_objects = Vec::new();
        for role_row in roles {
            let role_id_str = role_row.id;
            let policies: Vec<PolicyRow> = sqlx::query_as(
                r#"
                        SELECT id, account_ksuid, role_ksuid, effect, actions, resources
                        FROM role_policies
                        WHERE account_ksuid = $1
                          AND role_ksuid = $2
                          AND deleted_at IS NULL
                        ORDER BY created_at
                        "#,
            )
            .bind(&account_ksuid_str)
            .bind(&role_id_str)
            .fetch_all(&pool)
            .await
            .map_err(|e| {
                PipError::AcmNotFound(format!(
                    "broken, broken, broken. we got a broken role {}: {}",
                    role_row.name, e
                ))
            })?;

            let policy_objects: Vec<Policy> = policies
                .into_iter()
                .map(|p| {
                    let effect = match p.effect.as_str() {
                        "allow" => Effect::Allow,
                        "deny" => Effect::Deny,
                        _ => {
                            return Err(PipError::InvalidAcmFormat(format!(
                                "Invalid policy effect: {}",
                                p.effect
                            )));
                        }
                    };
                    Ok(Policy {
                        effect,
                        actions: p.actions.0,
                        resources: p.resources.0,
                    })
                })
                .collect::<Result<Vec<_>, _>>()?;

            role_objects.push(Role {
                name: role_row.name,
                description: role_row.description.unwrap_or_default(),
                policies: policy_objects,
            });
        }

        let mut acm = Acm::new();
        apply_roles(&mut acm, &role_objects);

        Ok(acm)
    }
}
