# BotLib Development Prompt Guide

**Version:** 6.1.0  
**Purpose:** LLM context for BotLib shared library development

---

## ZERO TOLERANCE POLICY

**This project has the strictest code quality requirements possible.**

**EVERY SINGLE WARNING MUST BE FIXED. NO EXCEPTIONS.**

---

## ABSOLUTE PROHIBITIONS

```
❌ NEVER use #![allow()] or #[allow()] in source code to silence warnings
❌ NEVER use _ prefix for unused variables - DELETE the variable or USE it
❌ NEVER use .unwrap() - use ? or proper error handling
❌ NEVER use .expect() - use ? or proper error handling  
❌ NEVER use panic!() or unreachable!() - handle all cases
❌ NEVER use todo!() or unimplemented!() - write real code
❌ NEVER leave unused imports - DELETE them
❌ NEVER leave dead code - DELETE it or IMPLEMENT it
❌ NEVER use approximate constants (3.14159) - use std::f64::consts::PI
❌ NEVER silence clippy in code - FIX THE CODE or configure in Cargo.toml
❌ NEVER add comments explaining what code does - code must be self-documenting
```

---

## CARGO.TOML LINT EXCEPTIONS

When a clippy lint has **technical false positives** that cannot be fixed in code,
disable it in `Cargo.toml` with a comment explaining why:

```toml
[lints.clippy]
# Disabled: has false positives for functions with mut self, heap types (Vec, String)
missing_const_for_fn = "allow"
# Disabled: Tauri commands require owned types (Window) that cannot be passed by reference
needless_pass_by_value = "allow"
# Disabled: transitive dependencies we cannot control
multiple_crate_versions = "allow"
```

**Approved exceptions:**
- `missing_const_for_fn` - false positives for `mut self`, heap types
- `needless_pass_by_value` - Tauri/framework requirements
- `multiple_crate_versions` - transitive dependencies
- `future_not_send` - when async traits require non-Send futures

---

## MANDATORY CODE PATTERNS

### Error Handling - Use `?` Operator

```rust
// ❌ WRONG
let value = something.unwrap();
let value = something.expect("msg");

// ✅ CORRECT
let value = something?;
let value = something.ok_or_else(|| Error::NotFound)?;
```

### Self Usage in Impl Blocks

```rust
// ❌ WRONG
impl MyStruct {
    fn new() -> MyStruct { MyStruct { } }
}

// ✅ CORRECT
impl MyStruct {
    fn new() -> Self { Self { } }
}
```

### Format Strings - Inline Variables

```rust
// ❌ WRONG
format!("Hello {}", name)

// ✅ CORRECT
format!("Hello {name}")
```

### Display vs ToString

```rust
// ❌ WRONG
impl ToString for MyType { }

// ✅ CORRECT
impl std::fmt::Display for MyType { }
```

### Derive Eq with PartialEq

```rust
// ❌ WRONG
#[derive(PartialEq)]
struct MyStruct { }

// ✅ CORRECT
#[derive(PartialEq, Eq)]
struct MyStruct { }
```

---

## Version Management

**Version is 6.1.0 - NEVER CHANGE without explicit approval**

---

## Project Overview

BotLib is the shared foundation library for the General Bots workspace.

```
botlib/        # THIS PROJECT - Shared library
botserver/     # Main server (depends on botlib)
botui/         # Web/Desktop UI (depends on botlib)
botapp/        # Desktop app (depends on botlib)
```

---

## Module Structure

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

## Dependencies

| Library | Version | Purpose |
|---------|---------|---------|
| anyhow | 1.0 | Error handling |
| thiserror | 2.0 | Error derive |
| chrono | 0.4 | Date/time |
| serde | 1.0 | Serialization |
| uuid | 1.11 | UUIDs |
| diesel | 2.1 | Database ORM |
| reqwest | 0.12 | HTTP client |

---

## Remember

- **ZERO WARNINGS** - Every clippy warning must be fixed
- **NO ALLOW IN CODE** - Never use #[allow()] in source files
- **CARGO.TOML EXCEPTIONS OK** - Disable lints with false positives in Cargo.toml with comment
- **NO DEAD CODE** - Delete unused code, never prefix with _
- **NO UNWRAP/EXPECT** - Use ? operator or proper error handling
- **INLINE FORMAT ARGS** - format!("{name}") not format!("{}", name)
- **USE SELF** - In impl blocks, use Self not the type name
- **DERIVE EQ** - Always derive Eq with PartialEq
- **DISPLAY NOT TOSTRING** - Implement Display, not ToString
- **USE DIAGNOSTICS** - Use IDE diagnostics tool, never call cargo clippy directly
- **Version**: Always 6.1.0 - do not change without approval
- **Session Continuation**: When running out of context, create detailed summary: (1) what was done, (2) what remains, (3) specific files and line numbers, (4) exact next steps.