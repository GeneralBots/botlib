# BotLib Development Prompt Guide

**Version:** 6.1.0  
**Purpose:** LLM context for BotLib shared library development

---

## Version Management - CRITICAL

**Current version is 6.1.0 - DO NOT CHANGE without explicit approval!**

### Rules

1. **Version is 6.1.0 across ALL workspace crates**
2. **NEVER change version without explicit user approval**
3. **All workspace crates share version 6.1.0**
4. **BotLib does not have migrations - all migrations are in botserver/**

---

## Official Icons - Reference

**BotLib does not contain icons.** Icons are managed in:
- `botui/ui/suite/assets/icons/` - Runtime UI icons
- `botbook/src/assets/icons/` - Documentation icons

When documenting or referencing UI elements in BotLib:
- Reference icons by name (e.g., `gb-chat.svg`, `gb-drive.svg`)
- Never generate or embed icon content
- See `botui/PROMPT.md` for the complete icon list

---

## Weekly Maintenance - EVERY MONDAY

### Package Review Checklist

**Every Monday, review the following:**

1. **Dependency Updates**
   ```bash
   cargo outdated
   cargo audit
   ```

2. **Package Consolidation Opportunities**
   - Check if new crates can replace custom code
   - Look for crates that combine multiple dependencies
   - Review `Cargo.toml` for redundant dependencies

3. **Code Reduction Candidates**
   - Custom implementations that now have crate equivalents
   - Boilerplate that can be replaced with derive macros
   - Re-exports that can simplify downstream usage

4. **Feature Flag Review**
   - Check if optional features are still needed
   - Consolidate similar features
   - Remove unused feature gates

### Packages to Watch

| Area | Potential Packages | Purpose |
|------|-------------------|---------|
| Error Handling | `anyhow`, `thiserror` | Consolidate error types |
| Validation | `validator` | Replace manual validation |
| Serialization | `serde` derives | Reduce boilerplate |
| UUID | `uuid` | Consistent ID generation |

---

## Project Overview

BotLib is the shared foundation library for the General Bots workspace. It provides common types, utilities, error handling, and optional integrations that are consumed by botserver, botui, and botapp.

### Workspace Position

```
botlib/        # THIS PROJECT - Shared library
botserver/     # Main server (depends on botlib)
botui/         # Web/Desktop UI (depends on botlib)
botapp/        # Desktop app (depends on botlib)
botbook/       # Documentation
```

### What BotLib Provides

- **Error Types**: Common error handling with anyhow/thiserror
- **Models**: Shared data structures and types
- **HTTP Client**: Optional reqwest wrapper
- **Database**: Optional diesel integration
- **Validation**: Optional input validation
- **Branding**: Version and branding constants

---

## Feature Flags

```toml
[features]
default = []
full = ["database", "http-client", "validation"]
database = ["dep:diesel"]
http-client = ["dep:reqwest"]
validation = ["dep:validator"]
```

### Usage in Dependent Crates

```toml
# botserver/Cargo.toml
[dependencies.botlib]
path = "../botlib"
features = ["database"]

# botui/Cargo.toml
[dependencies.botlib]
path = "../botlib"
features = ["http-client"]
```

---

## Code Generation Rules

### CRITICAL REQUIREMENTS

```
- Library code must be generic and reusable
- No hardcoded values or project-specific logic
- All public APIs must be well-documented
- Feature gates for optional dependencies
- Zero warnings - clean compilation required
```

### Module Structure

```
src/
├── lib.rs           # Public exports, feature gates
├── error.rs         # Error types (thiserror)
├── models.rs        # Shared data models
├── message_types.rs # Message type definitions
├── http_client.rs   # HTTP client wrapper (feature-gated)
├── branding.rs      # Version, branding constants
└── version.rs       # Version information
```

---

## Adding New Features

### Adding a Shared Type

```rust
// src/models.rs
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SharedEntity {
    pub id: Uuid,
    pub name: String,
    pub created_at: DateTime<Utc>,
}
```

### Adding a Feature-Gated Module

```rust
// src/lib.rs
#[cfg(feature = "my-feature")]
pub mod my_module;

#[cfg(feature = "my-feature")]
pub use my_module::MyType;
```

```toml
# Cargo.toml
[features]
my-feature = ["dep:some-crate"]

[dependencies]
some-crate = { version = "1.0", optional = true }
```

### Adding Error Types

```rust
// src/error.rs
use thiserror::Error;

#[derive(Error, Debug)]
pub enum BotLibError {
    #[error("Configuration error: {0}")]
    Config(String),
    
    #[error("HTTP error: {0}")]
    Http(#[from] reqwest::Error),
    
    #[error("Database error: {0}")]
    Database(String),
}

pub type Result<T> = std::result::Result<T, BotLibError>;
```

---

## Re-exports Strategy

BotLib should re-export common dependencies to ensure version consistency:

```rust
// src/lib.rs
pub use anyhow;
pub use chrono;
pub use serde;
pub use serde_json;
pub use thiserror;
pub use uuid;

#[cfg(feature = "database")]
pub use diesel;

#[cfg(feature = "http-client")]
pub use reqwest;
```

Consumers then use:

```rust
use botlib::uuid::Uuid;
use botlib::chrono::Utc;
```

---

## Dependencies

| Library | Version | Purpose | Optional |
|---------|---------|---------|----------|
| anyhow | 1.0 | Error handling | No |
| thiserror | 2.0 | Error derive | No |
| log | 0.4 | Logging facade | No |
| chrono | 0.4 | Date/time | No |
| serde | 1.0 | Serialization | No |
| serde_json | 1.0 | JSON | No |
| uuid | 1.11 | UUIDs | No |
| toml | 0.8 | Config parsing | No |
| diesel | 2.1 | Database ORM | Yes |
| reqwest | 0.12 | HTTP client | Yes |
| validator | 0.18 | Validation | Yes |

---

## Testing

```bash
# Test all features
cargo test --all-features

# Test specific feature
cargo test --features database

# Test without optional features
cargo test
```

---

## Final Checks Before Commit

```bash
# Verify version is 6.1.0
grep "^version" Cargo.toml | grep "6.1.0"

# Build with all features
cargo build --all-features

# Check for warnings
cargo check --all-features 2>&1 | grep warning

# Run tests
cargo test --all-features
```

---

## Rules

- Keep botlib minimal and focused
- No business logic - only utilities and types
- Feature gate all optional dependencies
- Maintain backward compatibility
- Document all public APIs
- Target zero warnings
- **Version**: Always 6.1.0 - do not change without approval