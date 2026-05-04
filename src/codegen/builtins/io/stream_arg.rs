use crate::codegen::abi;
use crate::codegen::context::Context;
use crate::codegen::data_section::DataSection;
use crate::codegen::emit::Emitter;
use crate::codegen::expr::emit_expr;
use crate::codegen::platform::Arch;
use crate::parser::ast::Expr;
use crate::types::PhpType;

pub(super) fn emit_stream_fd_arg(
    function_name: &str,
    arg: &Expr,
    emitter: &mut Emitter,
    ctx: &mut Context,
    data: &mut DataSection,
) -> PhpType {
    let ty = emit_expr(arg, emitter, ctx, data);
    if matches!(ty, PhpType::Mixed | PhpType::Union(_)) {
        emit_unbox_stream_or_fatal(function_name, emitter, ctx, data);
    }
    ty
}

fn emit_unbox_stream_or_fatal(
    function_name: &str,
    emitter: &mut Emitter,
    ctx: &mut Context,
    data: &mut DataSection,
) {
    let ok_label = ctx.next_label("stream_resource_ok");

    abi::emit_call_label(emitter, "__rt_mixed_unbox");                          // unwrap a resource|false handle returned by fopen()
    match emitter.target.arch {
        Arch::AArch64 => {
            emitter.instruction("cmp x0, #9");                                  // is the boxed handle a stream resource payload?
            emitter.instruction(&format!("b.eq {}", ok_label));                 // continue only for resource values
        }
        Arch::X86_64 => {
            emitter.instruction("cmp rax, 9");                                  // is the boxed handle a stream resource payload?
            emitter.instruction(&format!("je {}", ok_label));                   // continue only for resource values
        }
    }
    emit_stream_type_error(function_name, emitter, data);
    emitter.label(&ok_label);
    match emitter.target.arch {
        Arch::AArch64 => {
            emitter.instruction("mov x0, x1");                                  // expose the unboxed native stream descriptor as the ordinary integer result
        }
        Arch::X86_64 => {
            emitter.instruction("mov rax, rdi");                                // expose the unboxed native stream descriptor as the ordinary integer result
        }
    }
}

fn emit_stream_type_error(
    function_name: &str,
    emitter: &mut Emitter,
    data: &mut DataSection,
) {
    let message = format!(
        "Fatal error: Uncaught TypeError: {}(): Argument #1 ($stream) must be of type resource, non-resource given\n",
        function_name
    );
    let (label, len) = data.add_string(message.as_bytes());
    match emitter.target.arch {
        Arch::AArch64 => {
            emitter.instruction("mov x0, #2");                                  // fd = stderr for the stream TypeError diagnostic
            emitter.adrp("x1", &label);                                         // load the page that contains the stream TypeError diagnostic
            emitter.add_lo12("x1", "x1", &label);                               // resolve the stream TypeError diagnostic address within that page
            emitter.instruction(&format!("mov x2, #{}", len));                  // pass the stream TypeError diagnostic length to write()
            emitter.syscall(4);
            emitter.instruction("mov x0, #1");                                  // exit status 1 indicates abnormal termination
            emitter.syscall(1);
        }
        Arch::X86_64 => {
            abi::emit_symbol_address(emitter, "rsi", &label);                   // point the Linux write buffer at the stream TypeError diagnostic
            emitter.instruction(&format!("mov edx, {}", len));                  // pass the stream TypeError diagnostic length to write()
            emitter.instruction("mov edi, 2");                                  // fd = stderr for the stream TypeError diagnostic
            emitter.instruction("mov eax, 1");                                  // Linux x86_64 syscall 1 = write
            emitter.instruction("syscall");                                     // emit the stream TypeError diagnostic
            emitter.instruction("mov edi, 1");                                  // exit status 1 indicates abnormal termination
            emitter.instruction("mov eax, 60");                                 // Linux x86_64 syscall 60 = exit
            emitter.instruction("syscall");                                     // terminate after reporting the stream TypeError diagnostic
        }
    }
}
