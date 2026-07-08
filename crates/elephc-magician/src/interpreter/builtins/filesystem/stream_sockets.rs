//! Purpose:
//! Orchestrates eval stream socket builtins over host TCP and local socket pairs.
//!
//! Called from:
//! - `crate::interpreter::expressions::eval_positional_expr_call()`.
//! - Dynamic callable dispatch under `builtins::registry::dispatch`.
//!
//! Key details:
//! - Concrete socket builtin behavior lives in focused `stream_sockets/` modules
//!   so stream resource helpers and by-reference writeback stay isolated.

use super::super::super::*;
use super::*;

mod common;
mod datagrams;
mod lifecycle;
mod open;
mod selection;

use common::*;
pub(in crate::interpreter) use datagrams::*;
pub(in crate::interpreter) use lifecycle::*;
pub(in crate::interpreter) use open::*;
pub(in crate::interpreter) use selection::*;
use selection::eval_socket_resource_id;
