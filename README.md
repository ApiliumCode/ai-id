<p align="center">
  <img src="https://raw.githubusercontent.com/ApiliumCode/aingle/main/assets/aingle.svg" alt="AIngle Logo" width="200"/>
</p>

<h1 align="center">ai-id</h1>

<p align="center">
  <strong>Base32 identity encoding for AIngle agents, keys, and resources</strong>
</p>

<p align="center">
  <a href="https://crates.io/crates/aiid"><img src="https://img.shields.io/crates/v/aiid.svg" alt="Crates.io"/></a>
  <a href="https://docs.rs/aiid"><img src="https://docs.rs/aiid/badge.svg" alt="Documentation"/></a>
  <a href="https://www.npmjs.com/package/@AIngle/aiid"><img src="https://img.shields.io/npm/v/@AIngle/aiid.svg" alt="npm"/></a>
  <a href="https://github.com/ApiliumCode/ai-id/blob/main/LICENSE"><img src="https://img.shields.io/badge/license-Apache--2.0-blue.svg" alt="License"/></a>
</p>

---

## Overview

`ai-id` provides a standardized Base32 encoding scheme for identifiers in the AIngle ecosystem. It ensures consistent, human-readable representations of cryptographic keys, agent IDs, entry hashes, and other identifiers across the distributed network.

## Features

- **Human-readable** - Base32 encoding optimized for readability
- **Checksum validation** - Built-in integrity verification
- **Multi-language** - Available in Rust and JavaScript
- **Type-safe** - Distinct types for different identifier categories
- **Compact** - Efficient encoding for network transmission

## Libraries

| Language | Package | Documentation |
|----------|---------|---------------|
| **Rust** | [aiid](./aiid) | [docs.rs](https://docs.rs/aiid) |
| **JavaScript** | [aiid-js](./aiid-js) | [README](./aiid-js/README.md) |

## Quick Start (Rust)

```toml
[dependencies]
aiid = "0.1"
```

```rust
use aiid::{AgentId, EntryHash};

// Create an agent ID from bytes
let agent = AgentId::from_raw_bytes(&agent_pubkey);
println!("Agent: {}", agent); // agl-abc123...

// Parse from string
let hash: EntryHash = "aeh-xyz789...".parse()?;
```

## Quick Start (JavaScript)

```bash
npm install @aingle/aiid
```

```javascript
import { AgentId, EntryHash } from '@aingle/aiid';

const agent = AgentId.fromBytes(pubkeyBytes);
console.log(agent.toString()); // agl-abc123...
```

## Identifier Types

| Type | Prefix | Description |
|------|--------|-------------|
| `AgentId` | `agl-` | Agent public key |
| `EntryHash` | `aeh-` | Entry content hash |
| `ActionHash` | `ach-` | Action/header hash |
| `DnaHash` | `dnh-` | DNA definition hash |

## Part of AIngle

This crate is part of the [AIngle](https://github.com/ApiliumCode/aingle) ecosystem - a Semantic DAG framework for IoT and distributed AI applications.

## License

Licensed under the Apache License, Version 2.0. See [LICENSE](LICENSE) for details.

---

<p align="center">
  <sub>Maintained by <a href="https://apilium.com">Apilium Technologies</a> - Tallinn, Estonia</sub>
</p>
