use authz::{ACM, Effect, Policy, Role};

mod acm_tests {
    use super::*;

    #[test]
    fn test_empty_acm_denies_all() {
        let acm = ACM::new();
        assert!(!acm.authorized("any:action", "any/resource"));
        assert!(!acm.authorized("networks:GetVLAN", "VLAN-1"));
    }

    #[test]
    fn test_allow_basic() {
        let mut acm = ACM::new();
        acm.allow("networks:GetVLAN", "VLAN-1");
        assert!(acm.authorized("networks:GetVLAN", "VLAN-1"));
        assert!(!acm.authorized("networks:GetVLAN", "VLAN-2"));
        assert!(!acm.authorized("networks:UpdateVLAN", "VLAN-1"));
    }

    #[test]
    fn test_deny_basic() {
        let mut acm = ACM::new();
        acm.allow("networks:*", "VLAN-1");
        acm.deny("networks:DeleteVLAN", "VLAN-1");
        assert!(acm.authorized("networks:GetVLAN", "VLAN-1"));
        assert!(!acm.authorized("networks:DeleteVLAN", "VLAN-1"));
    }

    #[test]
    fn test_deny_precedence_over_allow() {
        let mut acm = ACM::new();
        acm.allow("networks:*", "VLAN-1");
        acm.deny("networks:DeleteVLAN", "VLAN-1");

        assert!(acm.authorized("networks:GetVLAN", "VLAN-1"));
        assert!(acm.authorized("networks:UpdateVLAN", "VLAN-1"));
        assert!(!acm.authorized("networks:DeleteVLAN", "VLAN-1"));
    }

    #[test]
    fn test_wildcard_action_patterns() {
        let mut acm = ACM::new();
        acm.allow("networks:*", "VLAN-1");
        acm.allow("calendar:Get*", "laura/*");

        assert!(acm.authorized("networks:GetVLAN", "VLAN-1"));
        assert!(acm.authorized("networks:UpdateVLAN", "VLAN-1"));
        assert!(acm.authorized("networks:DeleteVLAN", "VLAN-1"));
        assert!(acm.authorized("calendar:GetCalendar", "laura/shared"));
        assert!(acm.authorized("calendar:GetEvent", "laura/events"));
        assert!(!acm.authorized("calendar:CreateEvent", "laura/events"));
    }

    #[test]
    fn test_wildcard_resource_patterns() {
        let mut acm = ACM::new();
        acm.allow("networks:GetVLAN", "*");
        acm.allow("networks:UpdateVLAN", "VLAN-*");
        acm.allow("networks:DeleteVLAN", "nick/lab/*");

        assert!(acm.authorized("networks:GetVLAN", "VLAN-1"));
        assert!(acm.authorized("networks:GetVLAN", "VLAN-999"));
        assert!(acm.authorized("networks:GetVLAN", "anything"));
        assert!(acm.authorized("networks:UpdateVLAN", "VLAN-1"));
        assert!(acm.authorized("networks:UpdateVLAN", "VLAN-999"));
        assert!(!acm.authorized("networks:UpdateVLAN", "other-1"));
        assert!(acm.authorized("networks:DeleteVLAN", "nick/lab/VLAN-1"));
        assert!(acm.authorized("networks:DeleteVLAN", "nick/lab/anything"));
        assert!(!acm.authorized("networks:DeleteVLAN", "nick/other/VLAN-1"));
    }

    #[test]
    fn test_complex_resource_paths() {
        let mut acm = ACM::new();
        acm.allow("storage:Read", "projects/*/buckets/*/objects/*");
        acm.allow("storage:Write", "projects/myproject/buckets/*");

        assert!(acm.authorized("storage:Read", "projects/p1/buckets/b1/objects/o1"));
        assert!(acm.authorized("storage:Read", "projects/p2/buckets/b2/objects/o2"));
        assert!(!acm.authorized("storage:Read", "projects/p1/buckets/b1"));
        assert!(acm.authorized("storage:Write", "projects/myproject/buckets/b1"));
        assert!(!acm.authorized("storage:Write", "projects/other/buckets/b1"));
    }

    #[test]
    fn test_single_segment_resources() {
        let mut acm = ACM::new();
        acm.allow("admin:*", "root");
        acm.allow("user:Read", "profile");

        assert!(acm.authorized("admin:DoAnything", "root"));
        assert!(acm.authorized("user:Read", "profile"));
        assert!(!acm.authorized("user:Write", "profile"));
    }
}

mod role_tests {
    use super::*;

    #[test]
    fn test_role_creation() {
        let role = Role::new("TestRole", "Test description");
        assert_eq!(role.name, "TestRole");
        assert_eq!(role.description, "Test description");
        assert_eq!(role.policies.len(), 0);
    }

    #[test]
    fn test_role_with_policies() {
        let role = Role::new("TestRole", "Test")
            .with_policy(Policy::allow(
                vec!["action1".into()],
                vec!["resource1".into()],
            ))
            .with_policy(Policy::deny(
                vec!["action2".into()],
                vec!["resource2".into()],
            ));

        assert_eq!(role.policies.len(), 2);
        matches!(role.policies[0].effect, Effect::Allow);
        matches!(role.policies[1].effect, Effect::Deny);
    }

    #[test]
    fn test_role_with_empty_policies() {
        let role = Role::new("EmptyRole", "No policies");
        let mut acm = ACM::new();
        acm.apply_role(&role);
        assert!(!acm.authorized("any:action", "any/resource"));
    }

    #[test]
    fn test_role_with_multiple_actions_and_resources() {
        let role = Role::new("MultiRole", "Test").with_policy(Policy {
            effect: Effect::Allow,
            actions: vec!["action1".into(), "action2".into()],
            resources: vec!["res1".into(), "res2".into()],
        });

        let mut acm = ACM::new();
        acm.apply_role(&role);

        assert!(acm.authorized("action1", "res1"));
        assert!(acm.authorized("action1", "res2"));
        assert!(acm.authorized("action2", "res1"));
        assert!(acm.authorized("action2", "res2"));
    }
}

mod policy_tests {
    use super::*;

    #[test]
    fn test_policy_allow_constructor() {
        let policy = Policy::allow(
            vec!["action1".into(), "action2".into()],
            vec!["resource1".into()],
        );
        matches!(policy.effect, Effect::Allow);
        assert_eq!(policy.actions.len(), 2);
        assert_eq!(policy.resources.len(), 1);
    }

    #[test]
    fn test_policy_deny_constructor() {
        let policy = Policy::deny(
            vec!["action1".into()],
            vec!["resource1".into(), "resource2".into()],
        );
        matches!(policy.effect, Effect::Deny);
        assert_eq!(policy.actions.len(), 1);
        assert_eq!(policy.resources.len(), 2);
    }
}

mod integration_tests {
    use super::*;

    #[test]
    fn test_role_policy_application() {
        let mut acm = ACM::new();
        let role = Role::new("MHQ.NetworkAdmin", "Lorem Ipsum.")
            .with_policy(Policy {
                effect: Effect::Deny,
                actions: vec!["networks:*".into()],
                resources: vec!["VLAN-1".into()],
            })
            .with_policy(Policy {
                effect: Effect::Allow,
                actions: vec!["networks:GetVLAN".into()],
                resources: vec!["*".into()],
            })
            .with_policy(Policy {
                effect: Effect::Allow,
                actions: vec!["networks:UpdateVLAN".into()],
                resources: vec!["VLAN-20".into()],
            })
            .with_policy(Policy {
                effect: Effect::Allow,
                actions: vec!["networks:*".into()],
                resources: vec!["VLAN-70".into()],
            })
            .with_policy(Policy {
                effect: Effect::Allow,
                actions: vec!["networks:AddVLANTag".into()],
                resources: vec!["nick/lab/*".into()],
            })
            .with_policy(Policy {
                effect: Effect::Allow,
                actions: vec!["calendar:Get*".into()],
                resources: vec!["laura/*".into()],
            });

        acm.apply_role(&role);

        let cases = vec![
            ("networks:GetVLAN", "VLAN-50", true),
            ("networks:UpdateVLAN", "VLAN-20", true),
            ("networks:UpdateVLAN", "VLAN-30", false),
            ("networks:GetVLAN", "VLAN-1", false),
            ("networks:DeleteVLAN", "VLAN-70", true),
            ("networks:AddVLANTag", "nick/lab/VLAN-70", true),
            ("calendar:GetCalendar", "laura/shared-family-calendar", true),
        ];

        for (action, resource, expected) in cases {
            let granted = acm.authorized(action, resource);
            assert_eq!(
                granted, expected,
                "action: {}, resource: {}, expected: {}, granted: {}",
                action, resource, expected, granted
            );
        }
    }

    #[test]
    fn test_multiple_roles() {
        let mut acm = ACM::new();

        let network_admin = Role::new("NetworkAdmin", "Network admin").with_policy(Policy::allow(
            vec!["networks:*".into()],
            vec!["VLAN-*".into()],
        ));

        let storage_admin = Role::new("StorageAdmin", "Storage admin")
            .with_policy(Policy::allow(
                vec!["storage:*".into()],
                vec!["buckets/*".into()],
            ))
            .with_policy(Policy::deny(
                vec!["storage:Delete".into()],
                vec!["buckets/production/*".into()],
            ));

        acm.apply_role(&network_admin);
        acm.apply_role(&storage_admin);

        assert!(acm.authorized("networks:GetVLAN", "VLAN-1"));
        assert!(acm.authorized("networks:UpdateVLAN", "VLAN-999"));

        assert!(acm.authorized("storage:Read", "buckets/test"));
        assert!(acm.authorized("storage:Write", "buckets/test"));
        assert!(acm.authorized("storage:Delete", "buckets/test"));
        assert!(!acm.authorized("storage:Delete", "buckets/production/test"));
        assert!(!acm.authorized("storage:Read", "buckets/production/test"));

        assert!(!acm.authorized("networks:GetVLAN", "buckets/test"));
        assert!(!acm.authorized("storage:Read", "VLAN-1"));
    }

    #[test]
    fn test_conflicting_policies_same_role() {
        let mut acm = ACM::new();
        let role = Role::new("ConflictingRole", "Test")
            .with_policy(Policy::allow(
                vec!["action:*".into()],
                vec!["resource".into()],
            ))
            .with_policy(Policy::deny(
                vec!["action:Specific".into()],
                vec!["resource".into()],
            ));

        acm.apply_role(&role);

        assert!(!acm.authorized("action:Specific", "resource"));
        assert!(acm.authorized("action:Other", "resource"));
    }

    #[test]
    fn test_wildcard_edge_cases() {
        let mut acm = ACM::new();
        acm.allow("*", "*");
        acm.deny("admin:*", "sensitive/*");

        assert!(acm.authorized("any", "resource"));
        assert!(acm.authorized("user", "profile"));

        acm.allow("*:*", "*");
        assert!(acm.authorized("user:Read", "profile"));

        assert!(acm.authorized("admin:Delete", "sensitive"));
        assert!(!acm.authorized("admin:Delete", "sensitive/data"));
        assert!(!acm.authorized("admin:Read", "sensitive/secret"));
        assert!(acm.authorized("admin:Read", "public"));

        acm.allow("*:*", "*/*");
        assert!(acm.authorized("any:action", "any/resource"));
        assert!(!acm.authorized("any:action", "any/nested/resource/path"));
    }
}

mod json_tests {
    use super::*;

    #[test]
    fn test_load_acm_from_valid_json() {
        let json_data = include_str!("./testdata/valid.json");
        let acm = ACM::from_json(json_data).expect("failed to load ACM from valid.json");

        let cases = vec![
            ("networks:GetVLAN", "VLAN-50", true),
            ("networks:UpdateVLAN", "VLAN-20", true),
            ("networks:UpdateVLAN", "VLAN-30", false),
            ("networks:GetVLAN", "VLAN-1", false),
            ("networks:DeleteVLAN", "VLAN-70", true),
            ("networks:AddVLANTag", "nick/lab/VLAN-70", true),
            ("calendar:GetCalendar", "laura/shared-family-calendar", true),
        ];

        for (action, resource, expected) in cases {
            let granted = acm.authorized(action, resource);
            assert_eq!(
                granted, expected,
                "action: {}, resource: {}, expected: {}, granted: {}",
                action, resource, expected, granted
            );
        }
    }

    #[test]
    fn test_load_acm_from_invalid_json() {
        let json_data = include_str!("./testdata/invalid.json");
        let result = ACM::from_json(json_data);
        assert!(result.is_err(), "expected error when loading invalid.json");
    }

    #[test]
    fn test_load_acm_from_empty_json() {
        let json_data = r#"{"roles": []}"#;
        let acm = ACM::from_json(json_data).expect("should parse empty roles");
        assert!(!acm.authorized("any:action", "any/resource"));
    }

    #[test]
    fn test_load_acm_from_malformed_json() {
        let test_cases = vec![
            r#"{"roles": [{"name": "test"}]}"#,
            r#"{"roles": [{"name": "test", "description": "test"}]}"#,
            r#"{"roles": [{"name": "test", "description": "test", "policies": [{"effect": "invalid"}]}]}"#,
            r#"not json at all"#,
            r#"{"roles": null}"#,
        ];

        for json_data in test_cases {
            let result = ACM::from_json(json_data);
            assert!(result.is_err(), "expected error for: {}", json_data);
        }
    }

    #[test]
    fn test_json_with_multiple_roles() {
        let json_data = r#"
        {
          "roles": [
            {
              "name": "Role1",
              "description": "First role",
              "policies": [
                {
                  "effect": "allow",
                  "actions": ["action1"],
                  "resources": ["resource1"]
                }
              ]
            },
            {
              "name": "Role2",
              "description": "Second role",
              "policies": [
                {
                  "effect": "allow",
                  "actions": ["action2"],
                  "resources": ["resource2"]
                }
              ]
            }
          ]
        }
        "#;

        let acm = ACM::from_json(json_data).expect("should parse multiple roles");
        assert!(acm.authorized("action1", "resource1"));
        assert!(acm.authorized("action2", "resource2"));
        assert!(!acm.authorized("action1", "resource2"));
        assert!(!acm.authorized("action2", "resource1"));
    }
}
