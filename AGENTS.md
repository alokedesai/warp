# AGENTS.md

## Cursor Cloud specific instructions

### Environment overview

Warp is a Rust-based GPU-accelerated terminal emulator. The client codebase compiles entirely via Cargo (workspace with 60+ crates). No external backend services are required to build or test locally.

### Key commands

Standard dev commands are documented in `WARP.md` and `CONTRIBUTING.md`. Quick reference:

- **Build:** `cargo build --bin warp-oss --features gui`
- **Run:** `./script/run` (detects OSS vs internal channel automatically)
- **Lint:** `cargo fmt -- --check` and `cargo clippy --workspace --exclude warp_completer --all-targets --tests -- -D warnings` plus `cargo clippy -p warp_completer --all-targets --tests -- -D warnings`
- **Tests:** `cargo nextest run --no-fail-fast --workspace --exclude command-signatures-v2`
- **Presubmit:** `./script/presubmit` (runs fmt, clippy, clang-format, wgslfmt, and tests)

### Cloud VM gotchas

- **libstdc++ symlink:** The Cloud VM may be missing `/usr/lib/x86_64-linux-gnu/libstdc++.so` (the linker input symlink). If test compilation fails with `unable to find library -lstdc++`, run: `sudo ln -sf /usr/lib/x86_64-linux-gnu/libstdc++.so.6 /usr/lib/x86_64-linux-gnu/libstdc++.so`
- **OOM during linking:** Test binaries (especially the `warp` app crate) are very large. On VMs with 16 GB RAM, use `CARGO_BUILD_JOBS=1` when running `cargo nextest` to prevent the linker from being OOM-killed. The regular build (`cargo build`) works fine with default parallelism.
- **Integration tests require a display:** All integration tests in `crates/integration/` require a display server (X11/Wayland + Vulkan). They will fail in headless Cloud VMs. For unit tests, exclude integration: `cargo nextest run --workspace --exclude command-signatures-v2 --exclude integration`.
- **Clipboard tests need X11:** Two clipboard tests in `warpui` (`test_file_uri_decoded`, `test_absolute_paths_extracted`) require an X11 clipboard and will fail without a display.
- **command-signatures-v2 requires yarn/corepack:** The `command-signatures-v2` crate's build.rs needs Node.js with corepack-managed yarn. It is always excluded from test runs (`--exclude command-signatures-v2`).
- **Rust toolchain is pinned:** `rust-toolchain.toml` pins Rust to 1.92.0. The toolchain auto-installs via rustup on first cargo invocation.
