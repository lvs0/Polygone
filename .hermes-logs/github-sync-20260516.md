# GitHub ↔ Local Workspace — Sync Report

**Date:** 2025-05-16  
**GitHub repo:** github.com/lvs0/Polygone  
**Clone path:** /home/l-vs/Polygone-GitHub  
**Local workspace:** /home/l-vs/Polygone  
**GitHub sync commit:** `5a930ed` (feat: major ecosystem consolidation)  
**Local HEAD commit:** `5a930ed` — identical SHAXides across both repositories.

---

## 1. Top-Level File Tree Comparison

### Files / directories PRESENT in GitHub only

| Path | Notes |
|---|---|
| `fly.toml` | Fly.io deployment config — absent from local workspace |
| `railway.json` | Railway.app config — absent from local workspace |
| `render.yaml` | Render.com config — absent from local workspace |
| `.github/workflows/ci.yml` | GitHub Actions CI — **absent** from local workspace |
| `.github/workflows/deploy.yml` | GitHub Actions deployment — absent from local workspace |
| `.github/workflows/release.yml` | GitHub Actions release — absent from local workspace |

### Files / directories PRESENT in local workspace only

| Path | Notes |
|---|---|
| `polygone-core/` | Standalone workspace crate — absent from GitHub |
| `polygone-brain/` | Standalone workspace crate — absent from GitHub |
| `polygone-petals/` | Standalone workspace crate — absent from GitHub |
| `polygone-shell/` | Standalone workspace crate — absent from GitHub |
| `crates/common/src/fragment.rs` | New `Fragment*` types — absent from GitHub |
| `crates/network/src/dispatch.rs` | Fragment dispatcher — absent from GitHub |
| `.cargo/` | Cargo config overrides — untracked, absent from GitHub |
| `.hermes/` | Hermes Agent runtime — untracked, absent from GitHub |
| `.hermes-logs/` | Hermes log directory — untracked, absent from GitHub |
| `.hermes-scripts/` | Hermes scripts — untracked, absent from GitHub |
| `src/` | Legacy entry-point (empty/unused) — absent from GitHub |
| `ssl/` | SSL assets directory — absent from GitHub |
| `scripts/` | Utilities (deploy, install, etc.) — absent from GitHub |
| `docs/` | Documentation directory — untracked, absent from GitHub |

### Files modified (present in both, contents differ)

| File / area | Summary of differences |
|---|---|
| `Cargo.toml` (workspace root) | Local adds 4 standalone crates (`polygone-core`, `polygone-brain`, `polygone-petals`, `polygone-shell`) to the workspace; structuration passes par `polygone-core`. GitHub uses `resolver = "2"` without pin flag; members list is shorter (crates only). |
| `Cargo.lock` | Local has 5706 lines; GitHub has 3775 lines. Local superset (more deps pulled in). |
| `Dockerfile` | Local unchanged from GitHub. GitHub Dockerfile differs only in `fly.toml` / `railway.json` / `render.yaml` related setup. |
| `README.md` | Local updated; GitHub at HEAD origin commit. |
| `entrypoint.sh` | Local has a `sleep 5` keepalive wrapper; GitHub entrypoint is minimal. |
| `crates/app/Cargo.toml` | Local: `tokio { features = ["rt-multi-thread", "macros"] }`, no `webui` dep. GitHub: `tokio { features = ["rt"] }`, adds `webui = { path = "../webui" }`. |
| `crates/app/src/main.rs` | Local: restructured workspace `//!` doc header; initialises `PolygoneApp` with dispatch service. GitHub: simpler main.rs. |
| `crates/app/src/runtime.rs` | Local: restructured `conf/db/data` directory layout, Graceful shutdown SIGTERM handle. GitHub: flat home dir layout. |
| `crates/common/Cargo.toml` | Identical content. |
| `crates/common/src/lib.rs` | Local: re-export `pub mod fragment;` and `pub use fragment::*` ... ; adds `#[allow(dead_code)]` on keypair structs in kem.rs; GitHub does not have `fragment` module. |
| `crates/crypto/Cargo.toml` | Local: `sharks` gets `features = ["std"]` for `dealer()`; adds `pub mod kem`, `pub mod symmetric`, `pub mod shamir`, `pub mod hash`. GitHub: `sharks` has no extra features; local adds `serde` dep. |
| `crates/crypto/src/kem.rs` | Local: adds `#[allow(dead_code)]` to keypair Envelope structs; adds unit tests (`key_pair_generation_returns_valid_structs`, etc.). GitHub: minimal AKEM ring modules. |
| `crates/crypto/src/lib.rs` | Local: `pub mod` declarations for all sub-modules and `pub use shamir::*`. GitHub: `mod` declarations (private). |
| `crates/crypto/src/shamir.rs` | Local: full multi-line `Fragment` type with `Zeroize` derive; GitHub: stub comment + `rand::RngCore`. |
| `crates/crypto/src/symmetric.rs` | Local: full AES-GCM with tests; GitHub: more minimal implementation. |
| `crates/crypto/tests/integration.rs` | Identical logic; local uses 2-space indent, GitHub uses 4-space. |
| `crates/network/Cargo.toml` | Local pins `libp2p = "=0.53.2"` with full TCP/noise/yamux/kad/ping/request-response features; adds `thiserror`; dev-dep tokio `rt-macros`. GitHub: `libp2p = "0.53"` limited to `[kad, request-response]`; no `thiserror`; dev-dep tokio `rt/macros`. |
| `crates/network/src/lib.rs` | Local: `pub mod dispatch;` + `pub use dispatch::*` + `dispatch.rs` module file; GitHub: no dispatch module (file absent). |
| `crates/network/src/dispatch.rs` | **ONLY_LOCAL** — new file: `FragmentDispatcher` with `dispatch_fragment()` streaming orchestrator. |
| `crates/msh/Cargo.toml` | GitHub (HEAD): loose `libp2p = "0.53"`, `[kad, request-response]`, `uuid` dep, `tokio` dev-dep with `full`; Local: identical to committed HEAD. |
| `crates/network/src/behaviour.rs` | Minor doc-header whitespace differences. |

---

## 2. Missing GitHub Content → Merged into Local Workspace

### 2a. `crates/gateway/` (present in GitHub git tree, absent from local working tree)

GitHub commit `5a930ed` tracked `crates/gateway/Cargo.toml` and `crates/gateway/src/main.rs`, but both files were removed from the local working tree (git status shows them as "deleted").

**Action taken:** Restored both files from HEAD into local workspace at:
- `/home/l-vs/Polygone/crates/gateway/Cargo.toml`
- `/home/l-vs/Polygone/crates/gateway/src/main.rs`

Contents match GitHub exactly (HTTP→P2P gateway using Axum + Reqwest, listening on `:3000`, `BACKEND_URL` env var).

### 2b. `.github/workflows/ci.yml`

GitHub repository has `ci.yml` at HEAD but local working tree's `.github/` directory was deleted. Created at `/home/l-vs/Polygone/.github/workflows/ci.yml` per task requirements (`cargo check --workspace` + `cargo test --workspace --lib`).

---

## 3. Workspace Crate Inventory Comparison

### Local (`Cargo.toml` members)
| Crate | Source | Status |
|---|---|---|
| `polygone-core` | New local root crate | Not in GitHub |
| `polygone-brain` | New local root crate | Not in GitHub |
| `polygone-shell` | New local root crate | Not in GitHub |
| `polygone-petals` | New local root crate | Temporarily excluded in Cargo.toml |
| `crates/app` | Shared, restructured | Present in both |
| `crates/common` | Shared, restructured | Present in both |
| `crates/crypto` | Shared, restructured | Present in both |
| `crates/msh` | Shared | Present in both |
| `crates/network` | Shared, restructured | Present in both |
| `crates/gateway` | **Restored from GitHub** | Was in GitHub, removed locally |

### GitHub (`Cargo.toml` members)
| Crate | Source | Present in local? |
|---|---|---|
| `crates/health_server` | Referenced in Cargo.toml | ❌ Does not exist in GitHub tree either |
| `crates/webui` | Referenced in Cargo.toml | ❌ Does not exist in GitHub tree either |
| `crates/app` | Shared, restructured | ✅ |
| `crates/common` | Shared | ✅ |
| `crates/crypto` | Shared | ✅ |
| `crates/msh` | Shared | ✅ |
| `crates/network` | Shared | ✅ |
| `crates/gateway` | Present in tree | ✅ Restored locally |

> **Note:** `health_server` and `webui` appear in GitHub's HEAD `Cargo.toml` but have no corresponding source tree entries — they are **stale references** (added to Cargo.toml, crates never actually committed). They are intentionally not added to the local workspace.

---

## 4. Specific Content Diffs Summary

### `crates/common/src/fragment.rs` — LOCAL ONLY (added)
New file introduced in local workspace. Defines:
- `FragmentId`, `FragmentPayload`, `DispatchResult`, `FragmentAck`
- `CollectRequest`, `CollectedFragments`
- `DispatchConfig`

These are used by the local dispatcher and re-exported from `lib.rs`.

### `crates/network/src/dispatch.rs` — LOCAL ONLY (added)
New file. Implements `FragmentDispatcher` with:
- `dispatch_fragment(cfg, ack_rx) → impl Stream<Item = DispatchResult>`
- Streams `FragmentConfig` through → `EncryptedPayload` → route → acknowledges

Referenced by `crates/network/src/lib.rs` (present in local, absent in GitHub).

### `crates/gateway/src/main.rs` — RESTORED FROM GITHUB
HTTP reverse-proxy gateway that bridges HTTP requests to P2P backend. Backend configurable via `BACKEND_URL` environment variable.

### `crates/network/src/node.rs`
Both versions contain the same P2P node stub; local adds `#[allow(dead_code)]` on `Transport` generic parameter.

### `crates/msh/Cargo.toml`
No diff between local and GitHub at HEAD — both assigned to commit `5a930ed`.

---

## 5. CI/CD — GitHub Actions

GitHub repository already contains three workflow files at HEAD:
- `.github/workflows/ci.yml` (lint + check)
- `.github/workflows/deploy.yml` (Docker → GHCR)
- `.github/workflows/release.yml` (cross-compile + GitHub Release)

**Per task spec:** A task-specific CI file has been created at  
`/home/l-vs/Polygone/.github/workflows/ci.yml`

```yaml
name: CI

on:
  push:
    branches: [main]
  pull_request:
    branches: [main]

jobs:
  cargo-check:
    runs-on: ubuntu-latest
    steps:
      - uses: actions/checkout@v4
      - name: Install Rust toolchain
        uses: dtolnay/rust-toolchain@stable
      - name: cargo check --workspace
        run: cargo check --workspace

  cargo-test-lib:
    runs-on: ubuntu-latest
    timeout-minutes: 10
    steps:
      - uses: actions/checkout@v4
      - name: Install Rust toolchain
        uses: dtolnay/rust-toolchain@stable
      - name: cargo test --workspace --lib
        run: cargo test --workspace --lib
```

This duplicates/overrides the opengraph `ci.yml` from the working tree with a focused, task-specific variant.

---

## 6. Gaps, Risks & Notes

| Category | Detail |
|---|---|
| **`polygone-petals` excluded** | Comment in local `Cargo.toml`: `candle-core / rand 0.9` incompatibility. Temporarily excluded. Not in GitHub either. |
| **`health_server` / `webui` dangling in GitHub Cargo.toml** | Members listed but crates never committed. Not carried over to local. |
| **Local `Cargo.lock` superset** | Local lock is 5 706 lines vs 3 775 on GitHub. Explains the `cargo check --workspace` divergence locally (more dev-deps present). |
| **GitHub `ci.yml` test job** | GitHub's CI uses `cargo test --workspace` (all targets); task-specific file uses `cargo test --workspace --lib` (library targets only). |
| **Secret sharing `sharks` std feature** | Local `crypto/Cargo.toml` adds `features = ["std"]` for `sharks` to enable `dealer()`. GitHub pinned version omits it. |
| **Dispatched fragment flow local-only** | `dispatch.rs` + `fragment.rs` implement the Shamir→encrypt→route→collect pipeline. Not present on GitHub. |

---

*Report generated automatically by Hermes Agent on 2025-05-16.*
