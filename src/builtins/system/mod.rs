//! Purpose:
//! Groups all `system`-area time/date/sleep/env/process/output/define builtin homes
//! into this module so the registry can collect them in one place. Each submodule
//! declares exactly one builtin via `builtin!` and provides its lowering hook (and
//! optional check hook).
//!
//! Called from:
//! - `crate::builtins` (`mod system;` in `src/builtins/mod.rs`).
//!
//! Key details:
//! - Pure-data builtins (no check hook): time, sleep, usleep, checkdate, date, gmdate,
//!   mktime, gmmktime, hrtime, getdate, localtime, date_default_timezone_get/set,
//!   __elephc_mktime_raw, __elephc_gmmktime_raw, __elephc_strtotime_raw,
//!   putenv, http_response_code, header, phpversion, exec, shell_exec, system, passthru.
//! - Check-hook builtins: microtime (literal-dependent return type), strtotime
//!   (returns Union(Int, Bool)), getenv (returns Union(Str, Bool)), php_uname (validates
//!   arg type), define (side-effect: registers constant type), defined (validates literal).
//! - Add `pub mod <name>;` here for every new system builtin home.

pub mod __elephc_gmmktime_raw;
pub mod __elephc_mktime_raw;
pub mod __elephc_strtotime_raw;
pub mod checkdate;
pub mod date;
pub mod date_default_timezone_get;
pub mod date_default_timezone_set;
pub mod define;
pub mod defined;
pub mod exec;
pub mod getdate;
pub mod getenv;
pub mod gmdate;
pub mod gmmktime;
pub mod header;
pub mod hrtime;
pub mod http_response_code;
pub mod localtime;
pub mod microtime;
pub mod mktime;
pub mod passthru;
pub mod php_uname;
pub mod phpversion;
pub mod putenv;
pub mod shell_exec;
pub mod sleep;
pub mod strtotime;
pub mod system;
pub mod time;
pub mod usleep;
