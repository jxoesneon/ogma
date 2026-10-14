# Ogma (`ogma-rs`) — Omni-Protocol API, FFI, MCP & Wire Specification Reference

## 1. Overview of Supported Interfaces

1. **Rust Native API (`src/lib.rs`)**: `OgmaEngine`, `OgmaAnalysisReport`, `AdaptiveOperatorPipeline`, `SseStreamInterceptor`, `StructuralRepairOperator` (`12` variants).
2. **C-ABI Foreign Function Interface (`include/ogma_engine.h`)**: `ogma_engine_new`, `ogma_engine_free`, `ogma_engine_pre_seed`, `ogma_engine_analyze_json`, `ogma_engine_surgical_fix_json`, `ogma_engine_converge_json`, `ogma_engine_sarif_json`, `ogma_engine_free_str`.
3. **Python Native & FFI SDK (`ogma_sdk.py`)**: Dual-mode Python SDK with `OgmaEngine` and `OgmaFfiClient` (`ctypes.CDLL`).
4. **Model Context Protocol (MCP) Server (`JSON-RPC 2.0`)**: Exposes `ogma_pre_seed`, `ogma_analyze`, `ogma_course_correct`, and `ogma_converge` over `stdio`.
5. **Multi-Provider HTTP & Streaming SSE Reverse Proxy**: Supports `/v1/chat/completions`, `/v1/responses`, `/v1/messages`, `:generateContent`, and `/v1/converge`.
6. **CLI & SARIF v2.1.0 CI Linter (`ogma`)**: Subcommands `seed`, `analyze`, `fix`, `converge`, `sarif`, `mcp`, and default self-verification demo.
