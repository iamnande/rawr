# rawr 🦖

> righteous authorization with rust, like the hipster gods intended

**rawr** is a blazing-fast, zero-nonsense authorization library for Rust
that will leave your access control game **dripping** with security rizz.

It's built on a trie-based pattern matching system with glob support, this bad
boy handles your Allow/Deny policies like a champ.

## peep the stats

rawr provides:
- ⚡ **lightning-fast authorization checks** using a trie data structure (~3.6 
million per second throughput)
- 🎯 **glob pattern matching** for actions and resources
- 🛡️ **role-based access control** with `allow`/`deny` policies

coming soon:
- **attribute-level conditions**

## rtfm

1. Add this to your `Cargo.toml`:

```toml
[dependencies]
rawr-acm = { path = "./rawr-acm" }
rawr-pap = { path = "./rawr-pap" }
rawr-pip = { path = "./rawr-pip" }
rawr-pdp = { path = "./rawr-pdp" }
# or from crates.io when we're famous
```

2. Create a `Role` with some `Policies` (PAP vibes)

```rust
use rawr_pap::{Role, Policy};

let role = Role::new("MHQ.NetworkAdmin", "The network whisperer")
    .with_policy(Policy::deny(
        vec!["networks:*".to_string()],
        vec!["VLAN-1".to_string()],
    ))
    .with_policy(Policy::allow(
        vec!["networks:GetVLAN".to_string()],
        vec!["*".to_string()],
    ))
    .with_policy(Policy::allow(
        vec!["networks:UpdateVLAN".to_string()],
        vec!["VLAN-20".to_string()],
    ))
    .with_policy(Policy::allow(
        vec!["networks:*".to_string()],
        vec!["VLAN-70".to_string()],
    ))
    .with_policy(Policy::allow(
        vec!["networks:AddVLANTag".to_string()],
        vec!["nick/lab/*".to_string()],
    ));
```

3. Apply the `Role` to an `Acm` and enforce that authz. PIP problems, amirite?

```rust
use rawr_acm::Acm;
use rawr_pip::apply_role;

let mut acm = Acm::new();
apply_role(&mut acm, &role);
```

4. Enforce authorization against all the actions and resources you could ever desire.

```rust
// true
assert!(acm.enforce("networks:GetVLAN", "VLAN-50"));

// false
assert!(!acm.enforce("networks:DeleteVLAN", "VLAN-1"));

// true
assert!(acm.enforce("networks:UpdateVLAN", "VLAN-70"));

// true
assert!(acm.enforce("networks:AddVLANTag", "nick/lab/VLAN-70"));
```

5. Or go full PDP mode with a `Decider` (the real way to do it)

```rust
use rawr_pdp::{Decider, RawrDecider};
use rawr_pip::JsonPolicyLoader;
use svix_ksuid::Ksuid;
use std::str::FromStr;

let loader = JsonPolicyLoader::default();
let decider = RawrDecider::new(Box::new(loader));
let principal_ksuid = Ksuid::from_str("35zt2PT8xuykGfG83dEp5cZY4AM").unwrap();

// this bad boy caches ACMs for you, so it's fast AF boi
let granted = decider.decide(&principal_ksuid, "networks:GetVLAN", "VLAN-50")?;
```

## lez incompétentz

Here's the full workflow in one epic example:

```rust
use rawr_pdp::{Decider, RawrDecider};
use rawr_pip::JsonPolicyLoader;
use svix_ksuid::Ksuid;
use std::str::FromStr;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Step 1: Set up your policy loader (loads from JSON files)
    let loader = JsonPolicyLoader::default();
    
    // Step 2: Create a decider (this caches ACMs for you, so it's fast)
    let decider = RawrDecider::new(Box::new(loader));
    
    // Step 3: Get your principal's KSUID
    let principal_ksuid = Ksuid::from_str("35zt2PT8xuykGfG83dEp5cZY4AM")?;
    
    // Step 4: Make decisions (this is where the magic happens)
    println!("Can read code? {}", decider.decide(&principal_ksuid, "code:Read", "repos/my-app")?);
    println!("Can deploy to staging? {}", decider.decide(&principal_ksuid, "deploy:Rollout", "staging/web")?);
    println!("Can deploy to prod? {}", decider.decide(&principal_ksuid, "deploy:Rollout", "production/web")?);
    
    Ok(())
}
```

## fit check

rawr uses a **trie data structure** to store and match action/resource
patterns. When you call `enforce()` or `decide()`, it:

1. Splits the action by `:` and the resource by `/` to create segments
2. Checks the deny trie first (because explicit denies win)
3. If not denied, checks the allow trie
4. Uses glob pattern matching for each segment (so `*` matches anything,
`foo/*` matches `foo/bar`, etc.)

This means your authorization checks are **O(n)** where n is the depth of your
action/resource path, not the number of policies. That's what we call
**efficient**, my dude.

The PDP (`RawrDecider`) also caches ACMs per principal, so subsequent
decisions are even faster. It's like having a turbo button for your authz.

## API Reference

rawr is split into four crates following the XACML architecture (but way cooler):

### `rawr-acm` - Access Control Model

The core authorization engine. This is where the magic happens.

#### `Acm`

An access control model with `allow`/`deny` policies stored in tries.

- `Acm::new()` - Create a new ACM
- `acm.allow(action, resource)` - Directly allow an action/resource (low-level)
- `acm.deny(action, resource)` - Directly deny an action/resource (low-level)
- `acm.enforce(action, resource)` - Check if an action on a resource is authorized (returns `bool`)
- `Acm::split_action(action)` - Split an action by `:` (utility method)
- `Acm::split_resource_path(resource_path)` - Split a resource path by `/` (utility method)

### `rawr-pap` - Policy Administration Point

Where you define your roles and policies. The admin interface, basically.

#### `Role`

A role with a name, description, and policies.

- `Role::new(name, description)` - Create a new role
- `role.with_policy(policy)` - Add a policy to the role (returns self for chaining)

#### `Policy`

A policy with an effect (`allow`/`deny`), actions, and resources.

- `Policy::allow(actions, resources)` - Create an `allow` policy
- `Policy::deny(actions, resources)` - Create a `deny` policy

#### `Effect`

An enum representing the policy effect.

- `Effect::Allow` - Allow the action/resource combo
- `Effect::Deny` - Deny the action/resource combo (overrides allows)

### `rawr-pip` - Policy Information Point

Loads policies and builds ACMs. The data fetcher.

#### `AcmLoader`

A trait for loading ACMs for a given principal.

- `load(principal_ksuid)` - Load an ACM for a principal (returns `Result<Acm, PipError>`)

#### `JsonPolicyLoader`

A concrete implementation that loads policies from JSON files.

- `JsonPolicyLoader::new(base_dir)` - Create a loader that reads from a directory
- `JsonPolicyLoader::default()` - Create a loader that reads from `testdata/` directory

#### Utility Functions

- `apply_role(acm, role)` - Apply a single role's policies to an ACM
- `apply_roles(acm, roles)` - Apply multiple roles' policies to an ACM

### `rawr-pdp` - Policy Decision Point

The decision maker. This is what you use in production (probably).

#### `Decider`

A trait for making authorization decisions.

- `decide(principal_ksuid, action, resource_path)` - Make an authorization decision (returns `Result<bool, PdpError>`)

#### `RawrDecider`

A concrete implementation with ACM caching for maximum speed.

- `RawrDecider::new(acm_loader)` - Create a decider with a policy loader
- `decide(...)` - Make a decision (caches ACMs per principal for performance)

### JSON Derulo

When loading from JSON, the format should be:

```json
{
  "roles": [
    {
      "name": "RoleName",
      "description": "Role description",
      "policies": [
        {
          "effect": "allow",
          "actions": ["action:pattern", "*"],
          "resources": ["resource/path", "*"]
        },
        {
          "effect": "deny",
          "actions": ["action:*"],
          "resources": ["resource/*"]
        }
      ]
    }
  ]
}
```

Files should be named `{principal_ksuid}.json` and placed in your loader's base directory.

## code community

Want to make rawr even more righteous? PRs welcome, but keep the vibes high 
and the code clean. No cap.

## for the suits

Unlicensed. There's like, no rules here dude.
