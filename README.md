# Ogma (`ogma-rs`)

**Sub-Millisecond 30-Dimensional Structural Manifold Analysis & Multi-Pass Course-Correction Engine for AI Harnesses**

Named after [Ogma](https://en.wikipedia.org/wiki/Ogma) ([Ogmios](https://en.wikipedia.org/wiki/Ogmios))—the Celtic deity of eloquence, rhetoric, and inventor of the structural notch-and-stem Ogham script—**`Ogma` (`ogma-rs`)** is a zero-dependency (`std`-only), deterministic Rust engine, C-ABI shared/static library (`libogma_engine`), Python SDK (`ogma_sdk.py`), Model Context Protocol (`stdio` JSON-RPC 2.0) server, HTTP/SSE reverse proxy, and SARIF v2.1.0 CI linter.

Grounded in computational narratology and discourse-structure research—specifically [StoryScope: Investigating idiosyncrasies in AI fiction (Russell et al., 2026)](https://arxiv.org/html/2604.03136v6) and [SlopShape: Identifying AI-Generated Commercial Web Content (Madler, 2026)](https://arxiv.org/html/2609.15369v3)—`Ogma` maps any draft into a 30-dimensional structural manifold ($N=600$ reference baseline: $N_H=100$ human, $N_{AI}=500$ across GPT, Claude, Gemini, DeepSeek, and Kimi) and executes an **Adaptive Multi-Pass Convergence Engine** across 12 concrete `StructuralRepairOperator` modules while strictly conserving 100% of dates (`1800–2030`) and URLs (`https://`).

## Pre-Built Multi-Platform Binary Distribution

Every GitHub Release (`v*.*.*`) automatically builds, checksums (`SHA256SUMS.txt` / `SHA512SUMS.txt`), and attests SLSA v1.0 provenance for **10 target architectures** plus universal Python wheels:

| Platform / OS | Target Triple | Distributed Artifacts in Archive |
| :--- | :--- | :--- |
| **Linux x86_64 (glibc)** | `x86_64-unknown-linux-gnu` | `bin/ogma`, `lib/libogma_engine.so`, `lib/libogma_engine.a`, `include/ogma_engine.h`, `python/ogma_sdk.py` |
| **Linux x86_64 (static musl)** | `x86_64-unknown-linux-musl` | Fully static `bin/ogma`, `lib/libogma_engine.a`, `include/ogma_engine.h`, `python/ogma_sdk.py` |
| **Linux ARM64 (glibc)** | `aarch64-unknown-linux-gnu` | `bin/ogma`, `lib/libogma_engine.so`, `lib/libogma_engine.a`, `include/ogma_engine.h`, `python/ogma_sdk.py` |
| **Linux ARM64 (static musl)** | `aarch64-unknown-linux-musl` | Fully static `bin/ogma`, `lib/libogma_engine.a`, `include/ogma_engine.h`, `python/ogma_sdk.py` |
| **macOS Intel (x86_64)** | `x86_64-apple-darwin` | `bin/ogma`, `lib/libogma_engine.dylib`, `lib/libogma_engine.a`, `include/ogma_engine.h`, `python/ogma_sdk.py` |
| **macOS Apple Silicon (ARM64)** | `aarch64-apple-darwin` | `bin/ogma`, `lib/libogma_engine.dylib`, `lib/libogma_engine.a`, `include/ogma_engine.h`, `python/ogma_sdk.py` |
| **macOS Universal2 (Fat Binary)** | `universal2-apple-darwin` | Fat `bin/ogma` + `lib/libogma_engine.dylib` (`x86_64` + `aarch64`), `include/ogma_engine.h`, `python/ogma_sdk.py` |
| **Windows x86_64 (MSVC)** | `x86_64-pc-windows-msvc` | `bin/ogma.exe`, `lib/ogma_engine.dll`, `lib/ogma_engine.dll.lib`, `include/ogma_engine.h`, `python/ogma_sdk.py` |
| **Windows ARM64 (MSVC)** | `aarch64-pc-windows-msvc` | `bin/ogma.exe`, `lib/ogma_engine.dll`, `lib/ogma_engine.dll.lib`, `include/ogma_engine.h`, `python/ogma_sdk.py` |
| **WebAssembly (WASI)** | `wasm32-wasip1` | Portable `bin/ogma.wasm`, `include/ogma_engine.h`, `python/ogma_sdk.py` |

## Quick Start

```bash
# Run the 6-tier 56-test 100.00% bytecode line coverage suite in Python
python3 ogma_sdk.py --verify-coverage

# Compile and run all 56 Rust tests across all 6 tiers (UT, PT, IT, BDD, BM, E2E)
cargo test --release

# Run CLI commands
python3 ogma_sdk.py seed "Write a distributed systems postmortem" claude
python3 ogma_sdk.py converge < draft.md
python3 ogma_sdk.py sarif draft.md < draft.md
```

## License

Dual-licensed under `MIT` or `Apache-2.0` at your option.
