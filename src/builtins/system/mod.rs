//! Purpose:
//! Groups all `system`-area time/date/sleep builtin homes into this module so the
//! registry can collect them in one place. Each submodule declares exactly one builtin
//! via `builtin!` and provides its lowering hook (and optional check hook).
//!
//! Called from:
//! - `crate::builtins` (`mod system;` in `src/builtins/mod.rs`).
//!
//! Key details:
//! - Pure-data builtins (no check hook): time, sleep, usleep, checkdate, date, gmdate,
//!   mktime, gmmktime, hrtime, getdate, localtime, date_default_timezone_get/set,
//!   __elephc_mktime_raw, __elephc_gmmktime_raw, __elephc_strtotime_raw.
//! - Two builtins need check hooks: microtime (literal-dependent return type) and
//!   strtotime (returns Union(Int, Bool)).
//! - Add `pub mod <name>;` here for every new system builtin home.

pub mod __elephc_gmmktime_raw;
pub mod __elephc_mktime_raw;
pub mod __elephc_strtotime_raw;
pub mod checkdate;
pub mod date;
pub mod date_default_timezone_get;
pub mod date_default_timezone_set;
pub mod getdate;
pub mod gmdate;
pub mod gmmktime;
pub mod hrtime;
pub mod localtime;
pub mod microtime;
pub mod mktime;
pub mod sleep;
pub mod strtotime;
pub mod time;
pub mod usleep;
