-- roles table; stores role definitions
CREATE TABLE IF NOT EXISTS roles (
    id CHAR(27) NOT NULL, 
    account_ksuid CHAR(27) NOT NULL, 
    name TEXT NOT NULL CHECK (
    char_length(name) <= 255
    ), 
    description TEXT CHECK (
    char_length(description) <= 1000
    ), 
    created_at TIMESTAMPTZ NOT NULL DEFAULT now(), 
    updated_at TIMESTAMPTZ NOT NULL DEFAULT now(), 
    deleted_at TIMESTAMPTZ NULL, 
    PRIMARY KEY (id, account_ksuid)
);
CREATE INDEX IF NOT EXISTS idx_active_roles_by_account_ksuid ON roles(account_ksuid) 
WHERE 
deleted_at IS NULL;
CREATE INDEX IF NOT EXISTS idx_active_roles_by_account_ksuid_and_name ON roles(account_ksuid, name) 
WHERE 
deleted_at IS NULL;
CREATE INDEX IF NOT EXISTS idx_deleted_roles ON roles(id) 
WHERE 
deleted_at IS NOT NULL;

-- role_policies; stores role policies
DO $$ BEGIN IF NOT EXISTS (
    SELECT 
        1 
    FROM 
    pg_type 
    WHERE 
    typname = 'policy_effect'
    ) THEN CREATE TYPE policy_effect AS ENUM ('allow', 'deny');
END IF;
END $$;

CREATE TABLE IF NOT EXISTS role_policies (
    id CHAR(27) NOT NULL, 
    account_ksuid CHAR(27) NOT NULL, 
    role_ksuid CHAR(27) NOT NULL, 
    effect policy_effect NOT NULL, 
    actions JSONB NOT NULL DEFAULT '[]' :: jsonb, 
    resources JSONB NOT NULL DEFAULT '[]' :: jsonb, 
    created_at TIMESTAMPTZ NOT NULL DEFAULT now(), 
    updated_at TIMESTAMPTZ NOT NULL DEFAULT now(), 
    deleted_at TIMESTAMPTZ NULL, 
    PRIMARY KEY (id, account_ksuid), 
    FOREIGN KEY (role_ksuid, account_ksuid) REFERENCES roles(id, account_ksuid) ON DELETE CASCADE
);
CREATE INDEX IF NOT EXISTS idx_active_role_policies_by_account_ksuid_and_role_ksuid ON role_policies(account_ksuid, role_ksuid) 
WHERE 
deleted_at IS NULL;

-- role_assignments; stores principal role associations
CREATE TABLE IF NOT EXISTS role_assignments (
    id CHAR(27) NOT NULL, 
    account_ksuid CHAR(27) NOT NULL, 
    principal_ksuid CHAR(27) NOT NULL, 
    role_ksuid CHAR(27) NOT NULL, 
    created_at TIMESTAMPTZ NOT NULL DEFAULT now(), 
    deleted_at TIMESTAMPTZ NULL, 
    PRIMARY KEY (
    account_ksuid, principal_ksuid, role_ksuid
    ), 
    FOREIGN KEY (role_ksuid, account_ksuid) REFERENCES roles(id, account_ksuid) ON DELETE CASCADE
);
CREATE INDEX IF NOT EXISTS idx_active_role_assignments ON role_assignments(account_ksuid, principal_ksuid, role_ksuid)
WHERE 
deleted_at IS NULL;
CREATE INDEX IF NOT EXISTS idx_active_role_assignments_by_account_ksuid ON role_assignments(account_ksuid) 
WHERE 
deleted_at IS NULL;
CREATE INDEX IF NOT EXISTS idx_active_role_assignments_by_account_ksuid_and_principal_ksuid ON role_assignments(account_ksuid, principal_ksuid) 
WHERE 
deleted_at IS NULL;













