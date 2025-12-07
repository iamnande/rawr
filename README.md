# 🦖 rawr

> righteous authorization with rust, like the hipster gods intended

**rawr** is a blazing-fast, zero-nonsense (alloc), production-ready rust
library for fine-grained access control. with rawr, your access control is
tight, your policies are lean, and your system is flexing access like a boss.

it's built on a trie-based pattern matching system with glob support, this bad
boy handles your allow / deny policies like a champ at any scale.


## ⚡ features

* **zero-allocation** - zero memory allocations where it counts.
* **high-performance** - millions of parses or checks per second (~12M/s)
* **flexible model** - plug in your own RBAC/ABAC model you wish, or use the 
default
* **storage agnostic** - works with in-memory, Redis, Postgres, DynamoDB, or 
custom stores.


## 🔧 status

⚠️ under active development 🚧

we are actively developing. expect new features and breaking changes until 
v1.0.0.


## 🤖 LLMs (a.k.a. clankers)

note: this project is built using human hands and brains. any issues or PRs
raised that make use of an LLM will be unceremoniously rejected and closed.