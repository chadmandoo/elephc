//! Fiber public-API runtime helpers.
//!
//! These are the functions the codegen will call when lowering Fiber method
//! invocations. The dispatcher keeps architecture selection here while the
//! target-specific emitters live in dedicated modules.

mod arm64;
mod common;
mod x86_64;

use crate::codegen::emit::Emitter;
use crate::codegen::platform::Arch;

pub fn emit_fiber_throw_state_error(emitter: &mut Emitter) {
    if emitter.target.arch == Arch::X86_64 {
        x86_64::emit_throw_state_error_x86_64(emitter);
        return;
    }

    arm64::emit_throw_state_error(emitter);
}

pub fn emit_fiber_construct(emitter: &mut Emitter) {
    if emitter.target.arch == Arch::X86_64 {
        x86_64::emit_construct_x86_64(emitter);
        return;
    }

    arm64::emit_construct(emitter);
}

pub fn emit_fiber_start(emitter: &mut Emitter) {
    if emitter.target.arch == Arch::X86_64 {
        x86_64::emit_start_x86_64(emitter);
        return;
    }

    arm64::emit_start(emitter);
}

pub fn emit_fiber_resume(emitter: &mut Emitter) {
    if emitter.target.arch == Arch::X86_64 {
        x86_64::emit_resume_x86_64(emitter);
        return;
    }

    arm64::emit_resume(emitter);
}

pub fn emit_fiber_suspend(emitter: &mut Emitter) {
    if emitter.target.arch == Arch::X86_64 {
        x86_64::emit_suspend_x86_64(emitter);
        return;
    }

    arm64::emit_suspend(emitter);
}

pub fn emit_fiber_throw(emitter: &mut Emitter) {
    if emitter.target.arch == Arch::X86_64 {
        x86_64::emit_throw_x86_64(emitter);
        return;
    }

    arm64::emit_throw(emitter);
}

pub fn emit_fiber_get_current(emitter: &mut Emitter) {
    if emitter.target.arch == Arch::X86_64 {
        x86_64::emit_get_current_x86_64(emitter);
        return;
    }

    arm64::emit_get_current(emitter);
}

pub fn emit_fiber_get_return(emitter: &mut Emitter) {
    if emitter.target.arch == Arch::X86_64 {
        x86_64::emit_get_return_x86_64(emitter);
        return;
    }

    arm64::emit_get_return(emitter);
}

pub fn emit_fiber_state_getter(emitter: &mut Emitter) {
    if emitter.target.arch == Arch::X86_64 {
        x86_64::emit_state_getter_x86_64(emitter);
        return;
    }

    arm64::emit_state_getter(emitter);
}
