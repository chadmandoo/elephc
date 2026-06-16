//! Purpose:
//! Tests for the linear-scan register allocator over EIR functions.
//!
//! Called from:
//! - `cargo test` through Rust's test harness.
//!
//! Key details:
//! - Functions are built by hand with `crate::ir::Builder`. Tests target the
//!   AArch64 pool (eight integer, seven float callee-saved registers) so pool
//!   sizes are deterministic.

use crate::codegen::platform::{Arch, Platform, Target};
use crate::ir::{Builder, Function, Immediate, IrType, Op, Ownership, Terminator, ValueId};
use crate::ir_passes::allocate_registers;
use crate::types::PhpType;

/// The AArch64 target used by these tests, giving a fixed pool size.
fn aarch64() -> Target {
    Target::new(Platform::Linux, Arch::AArch64)
}

/// Emits a float constant in the current block and returns its value.
fn emit_const_f64(builder: &mut Builder<'_>, value: f64) -> ValueId {
    builder
        .emit(
            Op::ConstF64,
            vec![],
            Some(Immediate::F64(value)),
            IrType::F64,
            PhpType::Float,
            Ownership::NonHeap,
        )
        .expect("const_f64 produces a value")
}

/// Three integer temporaries with low register pressure each receive a
/// register, and the function records the callee-saved registers it used.
#[test]
fn straight_line_integers_receive_registers() {
    let mut function = Function::new("ints".to_string(), IrType::I64, PhpType::Int);
    {
        let mut builder = Builder::new(&mut function);
        let entry = builder.create_named_block("entry", vec![]);
        builder.set_entry(entry);
        builder.position_at_end(entry);
        let v0 = builder.emit_const_i64(1);
        let v1 = builder.emit_const_i64(2);
        let v2 = builder.emit_iadd(v0, v1);
        builder.terminate(Terminator::Return { value: Some(v2) });
    }

    let allocation = allocate_registers(&function, aarch64());

    for raw in 0..3 {
        let reg = allocation.register_of(ValueId::from_raw(raw));
        assert!(reg.is_some(), "value v{raw} should receive a register");
        assert!(
            reg.unwrap().starts_with('x'),
            "integer value v{raw} belongs in the integer pool"
        );
    }
    assert!(
        !allocation.used_callee_saved().is_empty(),
        "assigning registers must record callee-saved usage"
    );
}

/// With more simultaneously-live values than the pool holds, the allocator
/// spills at least one value rather than assigning a register twice, and never
/// reports using more registers than the pool provides.
#[test]
fn register_pressure_forces_spills() {
    const COUNT: u32 = 12; // AArch64 integer pool holds 8
    let mut function = Function::new("pressure".to_string(), IrType::I64, PhpType::Int);
    {
        let mut builder = Builder::new(&mut function);
        let entry = builder.create_named_block("entry", vec![]);
        builder.set_entry(entry);
        builder.position_at_end(entry);

        let consts: Vec<ValueId> = (0..COUNT).map(|i| builder.emit_const_i64(i as i64)).collect();
        // Fold left so every constant stays live until its add: the first add
        // sees all later constants still pending, creating peak pressure.
        let mut acc = consts[0];
        for &c in &consts[1..] {
            acc = builder.emit_iadd(acc, c);
        }
        builder.terminate(Terminator::Return { value: Some(acc) });
    }

    let allocation = allocate_registers(&function, aarch64());

    let spilled = (0..COUNT)
        .filter(|&i| allocation.register_of(ValueId::from_raw(i)).is_none())
        .count();
    assert!(spilled >= 1, "more live values than registers must spill some");
    assert!(
        allocation.used_callee_saved().len() <= 8,
        "cannot use more than the eight-register integer pool"
    );
}

/// Integer and float values draw from separate register pools.
#[test]
fn integers_and_floats_use_separate_pools() {
    let mut function = Function::new("mixed".to_string(), IrType::I64, PhpType::Int);
    let (int_val, float_val) = {
        let mut builder = Builder::new(&mut function);
        let entry = builder.create_named_block("entry", vec![]);
        builder.set_entry(entry);
        builder.position_at_end(entry);
        let i = builder.emit_const_i64(1);
        let f = emit_const_f64(&mut builder, 2.5);
        builder.terminate(Terminator::Return { value: Some(i) });
        (i, f)
    };

    let allocation = allocate_registers(&function, aarch64());

    assert!(
        allocation.register_of(int_val).unwrap().starts_with('x'),
        "integer value uses an x-register"
    );
    assert!(
        allocation.register_of(float_val).unwrap().starts_with('d'),
        "float value uses a d-register"
    );
}

/// Block parameters and branch arguments stay in stack slots so the existing
/// slot-based block-parameter moves remain correct.
#[test]
fn block_parameters_and_branch_arguments_stay_spilled() {
    let mut function = Function::new("params".to_string(), IrType::I64, PhpType::Int);
    let (arg, param) = {
        let mut builder = Builder::new(&mut function);
        let entry = builder.create_named_block("entry", vec![]);
        let body = builder.create_named_block("body", vec![(IrType::I64, PhpType::Int)]);
        builder.set_entry(entry);

        builder.position_at_end(entry);
        let arg = builder.emit_const_i64(7);
        builder.terminate(Terminator::Br {
            target: body,
            args: vec![arg],
        });

        let param = builder.block_param(body, 0);
        builder.position_at_end(body);
        builder.terminate(Terminator::Return { value: Some(param) });
        (arg, param)
    };

    let allocation = allocate_registers(&function, aarch64());

    assert_eq!(
        allocation.register_of(arg),
        None,
        "a branch argument must stay in its slot"
    );
    assert_eq!(
        allocation.register_of(param),
        None,
        "a block parameter must stay in its slot"
    );
}

/// Generator functions fall back to all-spilled in this first cut, because
/// values must live in the generator state frame across suspends.
#[test]
fn generator_functions_are_all_spilled() {
    let mut function = Function::new("gen".to_string(), IrType::I64, PhpType::Int);
    {
        let mut builder = Builder::new(&mut function);
        let entry = builder.create_named_block("entry", vec![]);
        builder.set_entry(entry);
        builder.position_at_end(entry);
        let v0 = builder.emit_const_i64(1);
        builder.terminate(Terminator::Return { value: Some(v0) });
    }
    function.flags.is_generator = true;

    let allocation = allocate_registers(&function, aarch64());

    assert_eq!(allocation.register_of(ValueId::from_raw(0)), None);
    assert!(allocation.used_callee_saved().is_empty());
}
