//! Purpose:
//! Emits the `__rt_memory_get_usage` / `__rt_memory_get_peak_usage` runtime helpers, which
//! read elephc's always-on heap live-byte and peak-byte counters.
//!
//! Called from:
//! - `crate::codegen_support::runtime::emitters::emit_runtime()` via
//!   `crate::codegen_support::runtime::system`.
//!
//! Key details:
//! - `_gc_live` / `_gc_peak` are maintained on every `__rt_heap_alloc` / `__rt_heap_free`,
//!   independently of `_heap_debug_enabled` (which only gates free-list VALIDATION), so they
//!   report elephc's real runtime heap footprint — the faithful analogue of PHP's zend-
//!   allocator `memory_get_usage()` / `memory_get_peak_usage()` for status/display use.
//! - Each helper is a leaf load: it reads the counter into the integer result register and
//!   returns, clobbering no callee-saved register and needing no stack frame.

use crate::codegen_support::{abi, emit::Emitter};
use crate::types::PhpType;

/// Emits `__rt_memory_get_usage`: loads the current heap live-byte counter into the integer
/// result register. Output: x0 (ARM64) / rax (x86_64) = live heap bytes.
pub(crate) fn emit_memory_get_usage(emitter: &mut Emitter) {
    emitter.blank();
    emitter.comment("--- runtime: memory_get_usage ---");
    emitter.label_global("__rt_memory_get_usage");
    abi::emit_load_symbol_to_result(emitter, "_gc_live", &PhpType::Int);
    emitter.instruction("ret");
}

/// Emits `__rt_memory_get_peak_usage`: loads the peak heap live-byte high-watermark into the
/// integer result register. Output: x0 (ARM64) / rax (x86_64) = peak heap bytes.
pub(crate) fn emit_memory_get_peak_usage(emitter: &mut Emitter) {
    emitter.blank();
    emitter.comment("--- runtime: memory_get_peak_usage ---");
    emitter.label_global("__rt_memory_get_peak_usage");
    abi::emit_load_symbol_to_result(emitter, "_gc_peak", &PhpType::Int);
    emitter.instruction("ret");
}
