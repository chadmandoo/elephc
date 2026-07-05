.intel_syntax noprefix
.text

.globl _class_propinit_0
_class_propinit_0:
    # prologue
    push rbp
    mov rbp, rsp
    sub rsp, 96
    mov r10, QWORD PTR [rip + _concat_off]
    mov QWORD PTR [rbp - 88], r10
    # save callee-saved registers used by the register allocator
    mov QWORD PTR [rbp - 80], rbx
    # param $this from rdi
    mov QWORD PTR [rbp - 72], rdi
_eir__class_propinit_0_entry_0:
    mov rax, QWORD PTR [rbp - 72]
    mov QWORD PTR [rbp - 24], rax
    lea rax, [rip + _str_0]
    mov rdx, 0
    mov QWORD PTR [rbp - 40], rax
    mov QWORD PTR [rbp - 32], rdx
    mov r11, QWORD PTR [rbp - 24]
    sub rsp, 16
    mov QWORD PTR [rsp], r11
    mov rax, QWORD PTR [rbp - 40]
    mov rdx, QWORD PTR [rbp - 32]
    call __rt_str_persist
    mov r11, QWORD PTR [rsp]
    add rsp, 16
    sub rsp, 16
    mov QWORD PTR [rsp], rax
    mov QWORD PTR [rsp + 8], rdx
    sub rsp, 16
    mov QWORD PTR [rsp], r11
    mov rax, QWORD PTR [r11 + 8]
    call __rt_heap_free_safe
    mov r11, QWORD PTR [rsp]
    add rsp, 16
    mov rax, QWORD PTR [rsp]
    mov rdx, QWORD PTR [rsp + 8]
    add rsp, 16
    mov QWORD PTR [r11 + 8], rax
    mov QWORD PTR [r11 + 16], rdx
    mov rax, QWORD PTR [rbp - 72]
    mov QWORD PTR [rbp - 56], rax
    mov rax, 0
    mov rbx, rax
    mov r11, QWORD PTR [rbp - 56]
    sub rsp, 16
    mov QWORD PTR [rsp], r11
    mov rax, rbx
    mov r11, QWORD PTR [rsp]
    add rsp, 16
    mov QWORD PTR [r11 + 24], rax
    mov QWORD PTR [r11 + 32], 0
    jmp _class_propinit_0_epilogue
_class_propinit_0_epilogue:
    # restore callee-saved registers used by the register allocator
    mov rbx, QWORD PTR [rbp - 80]
    add rsp, 96
    pop rbp
    ret

.globl _class_propinit_1
_class_propinit_1:
    # prologue
    push rbp
    mov rbp, rsp
    sub rsp, 96
    mov r10, QWORD PTR [rip + _concat_off]
    mov QWORD PTR [rbp - 88], r10
    # save callee-saved registers used by the register allocator
    mov QWORD PTR [rbp - 80], rbx
    # param $this from rdi
    mov QWORD PTR [rbp - 72], rdi
_eir__class_propinit_1_entry_0:
    mov rax, QWORD PTR [rbp - 72]
    mov QWORD PTR [rbp - 24], rax
    lea rax, [rip + _str_0]
    mov rdx, 0
    mov QWORD PTR [rbp - 40], rax
    mov QWORD PTR [rbp - 32], rdx
    mov r11, QWORD PTR [rbp - 24]
    sub rsp, 16
    mov QWORD PTR [rsp], r11
    mov rax, QWORD PTR [rbp - 40]
    mov rdx, QWORD PTR [rbp - 32]
    call __rt_str_persist
    mov r11, QWORD PTR [rsp]
    add rsp, 16
    sub rsp, 16
    mov QWORD PTR [rsp], rax
    mov QWORD PTR [rsp + 8], rdx
    sub rsp, 16
    mov QWORD PTR [rsp], r11
    mov rax, QWORD PTR [r11 + 8]
    call __rt_heap_free_safe
    mov r11, QWORD PTR [rsp]
    add rsp, 16
    mov rax, QWORD PTR [rsp]
    mov rdx, QWORD PTR [rsp + 8]
    add rsp, 16
    mov QWORD PTR [r11 + 8], rax
    mov QWORD PTR [r11 + 16], rdx
    mov rax, QWORD PTR [rbp - 72]
    mov QWORD PTR [rbp - 56], rax
    mov rax, 0
    mov rbx, rax
    mov r11, QWORD PTR [rbp - 56]
    sub rsp, 16
    mov QWORD PTR [rsp], r11
    mov rax, rbx
    mov r11, QWORD PTR [rsp]
    add rsp, 16
    mov QWORD PTR [r11 + 24], rax
    mov QWORD PTR [r11 + 32], 0
    jmp _class_propinit_1_epilogue
_class_propinit_1_epilogue:
    # restore callee-saved registers used by the register allocator
    mov rbx, QWORD PTR [rbp - 80]
    add rsp, 96
    pop rbp
    ret

.globl _class_propinit_2
_class_propinit_2:
    # prologue
    push rbp
    mov rbp, rsp
    sub rsp, 96
    mov r10, QWORD PTR [rip + _concat_off]
    mov QWORD PTR [rbp - 88], r10
    # save callee-saved registers used by the register allocator
    mov QWORD PTR [rbp - 80], rbx
    # param $this from rdi
    mov QWORD PTR [rbp - 72], rdi
_eir__class_propinit_2_entry_0:
    mov rax, QWORD PTR [rbp - 72]
    mov QWORD PTR [rbp - 24], rax
    lea rax, [rip + _str_0]
    mov rdx, 0
    mov QWORD PTR [rbp - 40], rax
    mov QWORD PTR [rbp - 32], rdx
    mov r11, QWORD PTR [rbp - 24]
    sub rsp, 16
    mov QWORD PTR [rsp], r11
    mov rax, QWORD PTR [rbp - 40]
    mov rdx, QWORD PTR [rbp - 32]
    call __rt_str_persist
    mov r11, QWORD PTR [rsp]
    add rsp, 16
    sub rsp, 16
    mov QWORD PTR [rsp], rax
    mov QWORD PTR [rsp + 8], rdx
    sub rsp, 16
    mov QWORD PTR [rsp], r11
    mov rax, QWORD PTR [r11 + 8]
    call __rt_heap_free_safe
    mov r11, QWORD PTR [rsp]
    add rsp, 16
    mov rax, QWORD PTR [rsp]
    mov rdx, QWORD PTR [rsp + 8]
    add rsp, 16
    mov QWORD PTR [r11 + 8], rax
    mov QWORD PTR [r11 + 16], rdx
    mov rax, QWORD PTR [rbp - 72]
    mov QWORD PTR [rbp - 56], rax
    mov rax, 0
    mov rbx, rax
    mov r11, QWORD PTR [rbp - 56]
    sub rsp, 16
    mov QWORD PTR [rsp], r11
    mov rax, rbx
    mov r11, QWORD PTR [rsp]
    add rsp, 16
    mov QWORD PTR [r11 + 24], rax
    mov QWORD PTR [r11 + 32], 0
    jmp _class_propinit_2_epilogue
_class_propinit_2_epilogue:
    # restore callee-saved registers used by the register allocator
    mov rbx, QWORD PTR [rbp - 80]
    add rsp, 96
    pop rbp
    ret

.globl _class_propinit_3
_class_propinit_3:
    # prologue
    push rbp
    mov rbp, rsp
    sub rsp, 64
    mov r10, QWORD PTR [rip + _concat_off]
    mov QWORD PTR [rbp - 64], r10
    # param $this from rdi
    mov QWORD PTR [rbp - 56], rdi
_eir__class_propinit_3_entry_0:
    mov rax, QWORD PTR [rbp - 56]
    mov QWORD PTR [rbp - 24], rax
    lea rax, [rip + _str_1]
    mov rdx, 3
    mov QWORD PTR [rbp - 40], rax
    mov QWORD PTR [rbp - 32], rdx
    mov r11, QWORD PTR [rbp - 24]
    sub rsp, 16
    mov QWORD PTR [rsp], r11
    mov rax, QWORD PTR [rbp - 40]
    mov rdx, QWORD PTR [rbp - 32]
    call __rt_str_persist
    mov r11, QWORD PTR [rsp]
    add rsp, 16
    sub rsp, 16
    mov QWORD PTR [rsp], rax
    mov QWORD PTR [rsp + 8], rdx
    sub rsp, 16
    mov QWORD PTR [rsp], r11
    mov rax, QWORD PTR [r11 + 8]
    call __rt_heap_free_safe
    mov r11, QWORD PTR [rsp]
    add rsp, 16
    mov rax, QWORD PTR [rsp]
    mov rdx, QWORD PTR [rsp + 8]
    add rsp, 16
    mov QWORD PTR [r11 + 8], rax
    mov QWORD PTR [r11 + 16], rdx
    jmp _class_propinit_3_epilogue
_class_propinit_3_epilogue:
    add rsp, 64
    pop rbp
    ret

.globl _class_propinit_6
_class_propinit_6:
    # prologue
    push rbp
    mov rbp, rsp
    sub rsp, 192
    mov r10, QWORD PTR [rip + _concat_off]
    mov QWORD PTR [rbp - 184], r10
    # save callee-saved registers used by the register allocator
    mov QWORD PTR [rbp - 176], rbx
    # param $this from rdi
    mov QWORD PTR [rbp - 168], rdi
_eir__class_propinit_6_entry_0:
    mov rax, QWORD PTR [rbp - 168]
    mov QWORD PTR [rbp - 24], rax
    lea rax, [rip + _str_0]
    mov rdx, 0
    mov QWORD PTR [rbp - 40], rax
    mov QWORD PTR [rbp - 32], rdx
    mov r11, QWORD PTR [rbp - 24]
    sub rsp, 16
    mov QWORD PTR [rsp], r11
    mov rax, QWORD PTR [rbp - 40]
    mov rdx, QWORD PTR [rbp - 32]
    call __rt_str_persist
    mov r11, QWORD PTR [rsp]
    add rsp, 16
    sub rsp, 16
    mov QWORD PTR [rsp], rax
    mov QWORD PTR [rsp + 8], rdx
    sub rsp, 16
    mov QWORD PTR [rsp], r11
    mov rax, QWORD PTR [r11 + 8]
    call __rt_heap_free_safe
    mov r11, QWORD PTR [rsp]
    add rsp, 16
    mov rax, QWORD PTR [rsp]
    mov rdx, QWORD PTR [rsp + 8]
    add rsp, 16
    mov QWORD PTR [r11 + 8], rax
    mov QWORD PTR [r11 + 16], rdx
    mov rax, QWORD PTR [rbp - 168]
    mov QWORD PTR [rbp - 56], rax
    mov rax, 0
    mov rbx, rax
    mov r11, QWORD PTR [rbp - 56]
    sub rsp, 16
    mov QWORD PTR [rsp], r11
    mov rax, rbx
    mov r11, QWORD PTR [rsp]
    add rsp, 16
    mov QWORD PTR [r11 + 24], rax
    mov QWORD PTR [r11 + 32], 0
    mov rax, QWORD PTR [rbp - 168]
    mov QWORD PTR [rbp - 80], rax
    mov rax, 0
    mov rbx, rax
    mov r11, QWORD PTR [rbp - 80]
    sub rsp, 16
    mov QWORD PTR [rsp], r11
    mov rax, rbx
    mov r11, QWORD PTR [rsp]
    add rsp, 16
    mov QWORD PTR [r11 + 40], rax
    mov QWORD PTR [r11 + 48], 0
    mov rax, QWORD PTR [rbp - 168]
    mov QWORD PTR [rbp - 104], rax
    mov rax, 0
    mov rbx, rax
    mov r11, QWORD PTR [rbp - 104]
    sub rsp, 16
    mov QWORD PTR [rsp], r11
    mov rax, rbx
    mov r11, QWORD PTR [rsp]
    add rsp, 16
    mov QWORD PTR [r11 + 56], rax
    mov QWORD PTR [r11 + 64], 0
    mov rax, QWORD PTR [rbp - 168]
    mov QWORD PTR [rbp - 128], rax
    mov rax, 0
    mov rbx, rax
    mov r11, QWORD PTR [rbp - 128]
    sub rsp, 16
    mov QWORD PTR [rsp], r11
    mov rax, rbx
    mov r11, QWORD PTR [rsp]
    add rsp, 16
    mov QWORD PTR [r11 + 72], rax
    mov QWORD PTR [r11 + 80], 0
    mov rax, QWORD PTR [rbp - 168]
    mov QWORD PTR [rbp - 152], rax
    mov rax, 9223372036854775806
    mov rbx, rax
    mov r11, QWORD PTR [rbp - 152]
    sub rsp, 16
    mov QWORD PTR [rsp], r11
    mov rax, QWORD PTR [r11 + 88]
    call __rt_decref_mixed
    mov r11, QWORD PTR [rsp]
    add rsp, 16
    mov QWORD PTR [r11 + 88], 0
    mov QWORD PTR [r11 + 96], 0
    jmp _class_propinit_6_epilogue
_class_propinit_6_epilogue:
    # restore callee-saved registers used by the register allocator
    mov rbx, QWORD PTR [rbp - 176]
    add rsp, 192
    pop rbp
    ret

.globl _class_propinit_9
_class_propinit_9:
    # prologue
    push rbp
    mov rbp, rsp
    sub rsp, 416
    mov r10, QWORD PTR [rip + _concat_off]
    mov QWORD PTR [rbp - 408], r10
    # save callee-saved registers used by the register allocator
    mov QWORD PTR [rbp - 400], rbx
    # param $this from rdi
    mov QWORD PTR [rbp - 392], rdi
_eir__class_propinit_9_entry_0:
    mov rax, QWORD PTR [rbp - 392]
    mov QWORD PTR [rbp - 16], rax
    mov rax, 0
    mov rbx, rax
    mov r11, QWORD PTR [rbp - 16]
    sub rsp, 16
    mov QWORD PTR [rsp], r11
    mov rax, rbx
    mov r11, QWORD PTR [rsp]
    add rsp, 16
    mov QWORD PTR [r11 + 8], rax
    mov QWORD PTR [r11 + 16], 0
    mov rax, QWORD PTR [rbp - 392]
    mov QWORD PTR [rbp - 40], rax
    mov rax, 0
    mov rbx, rax
    mov r11, QWORD PTR [rbp - 40]
    sub rsp, 16
    mov QWORD PTR [rsp], r11
    mov rax, rbx
    mov r11, QWORD PTR [rsp]
    add rsp, 16
    mov QWORD PTR [r11 + 24], rax
    mov QWORD PTR [r11 + 32], 0
    mov rax, QWORD PTR [rbp - 392]
    mov QWORD PTR [rbp - 64], rax
    mov rax, 0
    mov rbx, rax
    mov r11, QWORD PTR [rbp - 64]
    sub rsp, 16
    mov QWORD PTR [rsp], r11
    mov rax, rbx
    mov r11, QWORD PTR [rsp]
    add rsp, 16
    mov QWORD PTR [r11 + 40], rax
    mov QWORD PTR [r11 + 48], 0
    mov rax, QWORD PTR [rbp - 392]
    mov QWORD PTR [rbp - 88], rax
    mov rax, 0
    mov rbx, rax
    mov r11, QWORD PTR [rbp - 88]
    sub rsp, 16
    mov QWORD PTR [rsp], r11
    mov rax, rbx
    mov r11, QWORD PTR [rsp]
    add rsp, 16
    mov QWORD PTR [r11 + 56], rax
    mov QWORD PTR [r11 + 64], 0
    mov rax, QWORD PTR [rbp - 392]
    mov QWORD PTR [rbp - 112], rax
    mov rax, 0
    mov rbx, rax
    mov r11, QWORD PTR [rbp - 112]
    sub rsp, 16
    mov QWORD PTR [rsp], r11
    mov rax, rbx
    mov r11, QWORD PTR [rsp]
    add rsp, 16
    mov QWORD PTR [r11 + 72], rax
    mov QWORD PTR [r11 + 80], 0
    mov rax, QWORD PTR [rbp - 392]
    mov QWORD PTR [rbp - 136], rax
    mov rax, 0
    mov rbx, rax
    mov r11, QWORD PTR [rbp - 136]
    sub rsp, 16
    mov QWORD PTR [rsp], r11
    mov rax, rbx
    mov r11, QWORD PTR [rsp]
    add rsp, 16
    mov QWORD PTR [r11 + 88], rax
    mov QWORD PTR [r11 + 96], 0
    mov rax, QWORD PTR [rbp - 392]
    mov QWORD PTR [rbp - 160], rax
    mov rax, 0
    mov rbx, rax
    mov r11, QWORD PTR [rbp - 160]
    sub rsp, 16
    mov QWORD PTR [rsp], r11
    mov rax, rbx
    mov r11, QWORD PTR [rsp]
    add rsp, 16
    mov QWORD PTR [r11 + 104], rax
    mov QWORD PTR [r11 + 112], 0
    mov rax, QWORD PTR [rbp - 392]
    mov QWORD PTR [rbp - 184], rax
    mov rax, 0
    mov rbx, rax
    mov r11, QWORD PTR [rbp - 184]
    sub rsp, 16
    mov QWORD PTR [rsp], r11
    mov rax, rbx
    mov r11, QWORD PTR [rsp]
    add rsp, 16
    mov QWORD PTR [r11 + 120], rax
    mov QWORD PTR [r11 + 128], 0
    mov rax, QWORD PTR [rbp - 392]
    mov QWORD PTR [rbp - 208], rax
    mov rax, 0
    mov rbx, rax
    mov r11, QWORD PTR [rbp - 208]
    sub rsp, 16
    mov QWORD PTR [rsp], r11
    mov rax, rbx
    mov r11, QWORD PTR [rsp]
    add rsp, 16
    mov QWORD PTR [r11 + 136], rax
    mov QWORD PTR [r11 + 144], 0
    mov rax, QWORD PTR [rbp - 392]
    mov QWORD PTR [rbp - 232], rax
    mov rax, 0
    mov rbx, rax
    mov r11, QWORD PTR [rbp - 232]
    sub rsp, 16
    mov QWORD PTR [rsp], r11
    mov rax, rbx
    mov r11, QWORD PTR [rsp]
    add rsp, 16
    mov QWORD PTR [r11 + 152], rax
    mov QWORD PTR [r11 + 160], 0
    mov rax, QWORD PTR [rbp - 392]
    mov QWORD PTR [rbp - 256], rax
    mov rax, 0
    mov rbx, rax
    mov r11, QWORD PTR [rbp - 256]
    sub rsp, 16
    mov QWORD PTR [rsp], r11
    mov rax, rbx
    mov r11, QWORD PTR [rsp]
    add rsp, 16
    mov QWORD PTR [r11 + 168], rax
    mov QWORD PTR [r11 + 176], 0
    mov rax, QWORD PTR [rbp - 392]
    mov QWORD PTR [rbp - 280], rax
    mov rax, 0
    mov rbx, rax
    mov r11, QWORD PTR [rbp - 280]
    sub rsp, 16
    mov QWORD PTR [rsp], r11
    mov rax, rbx
    mov r11, QWORD PTR [rsp]
    add rsp, 16
    mov QWORD PTR [r11 + 184], rax
    mov QWORD PTR [r11 + 192], 0
    mov rax, QWORD PTR [rbp - 392]
    mov QWORD PTR [rbp - 304], rax
    mov rax, 0
    mov rbx, rax
    mov r11, QWORD PTR [rbp - 304]
    sub rsp, 16
    mov QWORD PTR [rsp], r11
    mov rax, rbx
    mov r11, QWORD PTR [rsp]
    add rsp, 16
    mov QWORD PTR [r11 + 200], rax
    mov QWORD PTR [r11 + 208], 0
    mov rax, QWORD PTR [rbp - 392]
    mov QWORD PTR [rbp - 328], rax
    mov rax, 0
    mov rbx, rax
    mov r11, QWORD PTR [rbp - 328]
    sub rsp, 16
    mov QWORD PTR [rsp], r11
    mov rax, rbx
    mov r11, QWORD PTR [rsp]
    add rsp, 16
    mov QWORD PTR [r11 + 216], rax
    mov QWORD PTR [r11 + 224], 0
    mov rax, QWORD PTR [rbp - 392]
    mov QWORD PTR [rbp - 352], rax
    mov rax, 0
    mov rbx, rax
    mov r11, QWORD PTR [rbp - 352]
    sub rsp, 16
    mov QWORD PTR [rsp], r11
    mov rax, rbx
    mov r11, QWORD PTR [rsp]
    add rsp, 16
    mov QWORD PTR [r11 + 232], rax
    mov QWORD PTR [r11 + 240], 0
    mov rax, QWORD PTR [rbp - 392]
    mov QWORD PTR [rbp - 376], rax
    mov rax, 0
    mov rbx, rax
    mov r11, QWORD PTR [rbp - 376]
    sub rsp, 16
    mov QWORD PTR [rsp], r11
    mov rax, rbx
    mov r11, QWORD PTR [rsp]
    add rsp, 16
    mov QWORD PTR [r11 + 248], rax
    mov QWORD PTR [r11 + 256], 0
    jmp _class_propinit_9_epilogue
_class_propinit_9_epilogue:
    # restore callee-saved registers used by the register allocator
    mov rbx, QWORD PTR [rbp - 400]
    add rsp, 416
    pop rbp
    ret

.globl _class_propinit_13
_class_propinit_13:
    # prologue
    push rbp
    mov rbp, rsp
    sub rsp, 192
    mov r10, QWORD PTR [rip + _concat_off]
    mov QWORD PTR [rbp - 184], r10
    # save callee-saved registers used by the register allocator
    mov QWORD PTR [rbp - 176], rbx
    # param $this from rdi
    mov QWORD PTR [rbp - 168], rdi
_eir__class_propinit_13_entry_0:
    mov rax, QWORD PTR [rbp - 168]
    mov QWORD PTR [rbp - 24], rax
    lea rax, [rip + _str_0]
    mov rdx, 0
    mov QWORD PTR [rbp - 40], rax
    mov QWORD PTR [rbp - 32], rdx
    mov r11, QWORD PTR [rbp - 24]
    sub rsp, 16
    mov QWORD PTR [rsp], r11
    mov rax, QWORD PTR [rbp - 40]
    mov rdx, QWORD PTR [rbp - 32]
    call __rt_str_persist
    mov r11, QWORD PTR [rsp]
    add rsp, 16
    sub rsp, 16
    mov QWORD PTR [rsp], rax
    mov QWORD PTR [rsp + 8], rdx
    sub rsp, 16
    mov QWORD PTR [rsp], r11
    mov rax, QWORD PTR [r11 + 8]
    call __rt_heap_free_safe
    mov r11, QWORD PTR [rsp]
    add rsp, 16
    mov rax, QWORD PTR [rsp]
    mov rdx, QWORD PTR [rsp + 8]
    add rsp, 16
    mov QWORD PTR [r11 + 8], rax
    mov QWORD PTR [r11 + 16], rdx
    mov rax, QWORD PTR [rbp - 168]
    mov QWORD PTR [rbp - 64], rax
    lea rax, [rip + _str_0]
    mov rdx, 0
    mov QWORD PTR [rbp - 80], rax
    mov QWORD PTR [rbp - 72], rdx
    mov r11, QWORD PTR [rbp - 64]
    sub rsp, 16
    mov QWORD PTR [rsp], r11
    mov rax, QWORD PTR [rbp - 80]
    mov rdx, QWORD PTR [rbp - 72]
    call __rt_str_persist
    mov r11, QWORD PTR [rsp]
    add rsp, 16
    sub rsp, 16
    mov QWORD PTR [rsp], rax
    mov QWORD PTR [rsp + 8], rdx
    sub rsp, 16
    mov QWORD PTR [rsp], r11
    mov rax, QWORD PTR [r11 + 24]
    call __rt_heap_free_safe
    mov r11, QWORD PTR [rsp]
    add rsp, 16
    mov rax, QWORD PTR [rsp]
    mov rdx, QWORD PTR [rsp + 8]
    add rsp, 16
    mov QWORD PTR [r11 + 24], rax
    mov QWORD PTR [r11 + 32], rdx
    mov rax, QWORD PTR [rbp - 168]
    mov QWORD PTR [rbp - 96], rax
    mov rax, 0
    mov rbx, rax
    mov r11, QWORD PTR [rbp - 96]
    sub rsp, 16
    mov QWORD PTR [rsp], r11
    mov rax, rbx
    mov r11, QWORD PTR [rsp]
    add rsp, 16
    mov QWORD PTR [r11 + 40], rax
    mov QWORD PTR [r11 + 48], 0
    mov rax, QWORD PTR [rbp - 168]
    mov QWORD PTR [rbp - 120], rax
    mov rax, 0
    mov rbx, rax
    mov r11, QWORD PTR [rbp - 120]
    sub rsp, 16
    mov QWORD PTR [rsp], r11
    mov rax, rbx
    mov r11, QWORD PTR [rsp]
    add rsp, 16
    mov QWORD PTR [r11 + 56], rax
    mov QWORD PTR [r11 + 64], 0
    mov rdi, 4
    mov rsi, 8
    call __rt_array_new
    sub rsp, 16
    mov QWORD PTR [rsp], r12
    mov r10, QWORD PTR [rax - 8]
    mov r12, 0xffffffff000080ff
    and r10, r12
    mov r12, 8
    shl r12, 8
    or r10, r12
    mov QWORD PTR [rax - 8], r10
    mov r12, QWORD PTR [rsp]
    add rsp, 16
    mov QWORD PTR [rbp - 136], rax
    mov rax, QWORD PTR [rbp - 168]
    mov QWORD PTR [rbp - 144], rax
    mov rdi, 4
    mov rsi, 8
    call __rt_array_new
    sub rsp, 16
    mov QWORD PTR [rsp], r12
    mov r10, QWORD PTR [rax - 8]
    mov r12, 0xffffffff000080ff
    and r10, r12
    mov r12, 8
    shl r12, 8
    or r10, r12
    mov QWORD PTR [rax - 8], r10
    mov r12, QWORD PTR [rsp]
    add rsp, 16
    mov QWORD PTR [rbp - 152], rax
    mov r11, QWORD PTR [rbp - 144]
    sub rsp, 16
    mov QWORD PTR [rsp], r11
    mov rax, QWORD PTR [rbp - 152]
    mov rsi, 8
    mov rdi, rax
    call __rt_array_to_mixed
    sub rsp, 16
    mov QWORD PTR [rsp], rax
    call __rt_incref
    mov rax, QWORD PTR [rsp]
    add rsp, 16
    mov r11, QWORD PTR [rsp]
    add rsp, 16
    sub rsp, 16
    mov QWORD PTR [rsp], rax
    sub rsp, 16
    mov QWORD PTR [rsp], r11
    mov rax, QWORD PTR [r11 + 72]
    call __rt_decref_array
    mov r11, QWORD PTR [rsp]
    add rsp, 16
    mov rax, QWORD PTR [rsp]
    add rsp, 16
    mov QWORD PTR [r11 + 72], rax
    mov QWORD PTR [r11 + 80], 0
    mov rax, QWORD PTR [rbp - 152]
    call __rt_decref_array
    mov rax, QWORD PTR [rbp - 136]
    call __rt_decref_array
    jmp _class_propinit_13_epilogue
_class_propinit_13_epilogue:
    # restore callee-saved registers used by the register allocator
    mov rbx, QWORD PTR [rbp - 176]
    add rsp, 192
    pop rbp
    ret

.globl _class_propinit_14
_class_propinit_14:
    # prologue
    push rbp
    mov rbp, rsp
    sub rsp, 96
    mov r10, QWORD PTR [rip + _concat_off]
    mov QWORD PTR [rbp - 88], r10
    # save callee-saved registers used by the register allocator
    mov QWORD PTR [rbp - 80], rbx
    # param $this from rdi
    mov QWORD PTR [rbp - 72], rdi
_eir__class_propinit_14_entry_0:
    mov rax, QWORD PTR [rbp - 72]
    mov QWORD PTR [rbp - 24], rax
    lea rax, [rip + _str_0]
    mov rdx, 0
    mov QWORD PTR [rbp - 40], rax
    mov QWORD PTR [rbp - 32], rdx
    mov r11, QWORD PTR [rbp - 24]
    sub rsp, 16
    mov QWORD PTR [rsp], r11
    mov rax, QWORD PTR [rbp - 40]
    mov rdx, QWORD PTR [rbp - 32]
    call __rt_str_persist
    mov r11, QWORD PTR [rsp]
    add rsp, 16
    sub rsp, 16
    mov QWORD PTR [rsp], rax
    mov QWORD PTR [rsp + 8], rdx
    sub rsp, 16
    mov QWORD PTR [rsp], r11
    mov rax, QWORD PTR [r11 + 8]
    call __rt_heap_free_safe
    mov r11, QWORD PTR [rsp]
    add rsp, 16
    mov rax, QWORD PTR [rsp]
    mov rdx, QWORD PTR [rsp + 8]
    add rsp, 16
    mov QWORD PTR [r11 + 8], rax
    mov QWORD PTR [r11 + 16], rdx
    mov rax, QWORD PTR [rbp - 72]
    mov QWORD PTR [rbp - 56], rax
    mov rax, 0
    mov rbx, rax
    mov r11, QWORD PTR [rbp - 56]
    sub rsp, 16
    mov QWORD PTR [rsp], r11
    mov rax, rbx
    mov r11, QWORD PTR [rsp]
    add rsp, 16
    mov QWORD PTR [r11 + 24], rax
    mov QWORD PTR [r11 + 32], 0
    jmp _class_propinit_14_epilogue
_class_propinit_14_epilogue:
    # restore callee-saved registers used by the register allocator
    mov rbx, QWORD PTR [rbp - 80]
    add rsp, 96
    pop rbp
    ret

.globl _class_propinit_15
_class_propinit_15:
    # prologue
    push rbp
    mov rbp, rsp
    sub rsp, 96
    mov r10, QWORD PTR [rip + _concat_off]
    mov QWORD PTR [rbp - 88], r10
    # save callee-saved registers used by the register allocator
    mov QWORD PTR [rbp - 80], rbx
    # param $this from rdi
    mov QWORD PTR [rbp - 72], rdi
_eir__class_propinit_15_entry_0:
    mov rax, QWORD PTR [rbp - 72]
    mov QWORD PTR [rbp - 24], rax
    lea rax, [rip + _str_0]
    mov rdx, 0
    mov QWORD PTR [rbp - 40], rax
    mov QWORD PTR [rbp - 32], rdx
    mov r11, QWORD PTR [rbp - 24]
    sub rsp, 16
    mov QWORD PTR [rsp], r11
    mov rax, QWORD PTR [rbp - 40]
    mov rdx, QWORD PTR [rbp - 32]
    call __rt_str_persist
    mov r11, QWORD PTR [rsp]
    add rsp, 16
    sub rsp, 16
    mov QWORD PTR [rsp], rax
    mov QWORD PTR [rsp + 8], rdx
    sub rsp, 16
    mov QWORD PTR [rsp], r11
    mov rax, QWORD PTR [r11 + 8]
    call __rt_heap_free_safe
    mov r11, QWORD PTR [rsp]
    add rsp, 16
    mov rax, QWORD PTR [rsp]
    mov rdx, QWORD PTR [rsp + 8]
    add rsp, 16
    mov QWORD PTR [r11 + 8], rax
    mov QWORD PTR [r11 + 16], rdx
    mov rax, QWORD PTR [rbp - 72]
    mov QWORD PTR [rbp - 56], rax
    mov rax, 0
    mov rbx, rax
    mov r11, QWORD PTR [rbp - 56]
    sub rsp, 16
    mov QWORD PTR [rsp], r11
    mov rax, rbx
    mov r11, QWORD PTR [rsp]
    add rsp, 16
    mov QWORD PTR [r11 + 24], rax
    mov QWORD PTR [r11 + 32], 0
    jmp _class_propinit_15_epilogue
_class_propinit_15_epilogue:
    # restore callee-saved registers used by the register allocator
    mov rbx, QWORD PTR [rbp - 80]
    add rsp, 96
    pop rbp
    ret

.globl _class_propinit_18
_class_propinit_18:
    # prologue
    push rbp
    mov rbp, rsp
    sub rsp, 48
    mov r10, QWORD PTR [rip + _concat_off]
    mov QWORD PTR [rbp - 48], r10
    # param $this from rdi
    mov QWORD PTR [rbp - 40], rdi
_eir__class_propinit_18_entry_0:
    mov rdi, 4
    mov rsi, 8
    call __rt_array_new
    sub rsp, 16
    mov QWORD PTR [rsp], r12
    mov r10, QWORD PTR [rax - 8]
    mov r12, 0xffffffff000080ff
    and r10, r12
    mov r12, 8
    shl r12, 8
    or r10, r12
    mov QWORD PTR [rax - 8], r10
    mov r12, QWORD PTR [rsp]
    add rsp, 16
    mov QWORD PTR [rbp - 8], rax
    mov rax, QWORD PTR [rbp - 40]
    mov QWORD PTR [rbp - 16], rax
    mov rdi, 4
    mov rsi, 8
    call __rt_array_new
    sub rsp, 16
    mov QWORD PTR [rsp], r12
    mov r10, QWORD PTR [rax - 8]
    mov r12, 0xffffffff000080ff
    and r10, r12
    mov r12, 8
    shl r12, 8
    or r10, r12
    mov QWORD PTR [rax - 8], r10
    mov r12, QWORD PTR [rsp]
    add rsp, 16
    mov QWORD PTR [rbp - 24], rax
    mov r11, QWORD PTR [rbp - 16]
    sub rsp, 16
    mov QWORD PTR [rsp], r11
    mov rax, QWORD PTR [rbp - 24]
    mov rsi, 8
    mov rdi, rax
    call __rt_array_to_mixed
    sub rsp, 16
    mov QWORD PTR [rsp], rax
    call __rt_incref
    mov rax, QWORD PTR [rsp]
    add rsp, 16
    mov r11, QWORD PTR [rsp]
    add rsp, 16
    sub rsp, 16
    mov QWORD PTR [rsp], rax
    sub rsp, 16
    mov QWORD PTR [rsp], r11
    mov rax, QWORD PTR [r11 + 8]
    call __rt_decref_array
    mov r11, QWORD PTR [rsp]
    add rsp, 16
    mov rax, QWORD PTR [rsp]
    add rsp, 16
    mov QWORD PTR [r11 + 8], rax
    mov QWORD PTR [r11 + 16], 0
    mov rax, QWORD PTR [rbp - 24]
    call __rt_decref_array
    mov rax, QWORD PTR [rbp - 8]
    call __rt_decref_array
    jmp _class_propinit_18_epilogue
_class_propinit_18_epilogue:
    add rsp, 48
    pop rbp
    ret

.globl _class_propinit_19
_class_propinit_19:
    # prologue
    push rbp
    mov rbp, rsp
    sub rsp, 96
    mov r10, QWORD PTR [rip + _concat_off]
    mov QWORD PTR [rbp - 88], r10
    # save callee-saved registers used by the register allocator
    mov QWORD PTR [rbp - 80], rbx
    # param $this from rdi
    mov QWORD PTR [rbp - 72], rdi
_eir__class_propinit_19_entry_0:
    mov rax, QWORD PTR [rbp - 72]
    mov QWORD PTR [rbp - 24], rax
    lea rax, [rip + _str_0]
    mov rdx, 0
    mov QWORD PTR [rbp - 40], rax
    mov QWORD PTR [rbp - 32], rdx
    mov r11, QWORD PTR [rbp - 24]
    sub rsp, 16
    mov QWORD PTR [rsp], r11
    mov rax, QWORD PTR [rbp - 40]
    mov rdx, QWORD PTR [rbp - 32]
    call __rt_str_persist
    mov r11, QWORD PTR [rsp]
    add rsp, 16
    sub rsp, 16
    mov QWORD PTR [rsp], rax
    mov QWORD PTR [rsp + 8], rdx
    sub rsp, 16
    mov QWORD PTR [rsp], r11
    mov rax, QWORD PTR [r11 + 8]
    call __rt_heap_free_safe
    mov r11, QWORD PTR [rsp]
    add rsp, 16
    mov rax, QWORD PTR [rsp]
    mov rdx, QWORD PTR [rsp + 8]
    add rsp, 16
    mov QWORD PTR [r11 + 8], rax
    mov QWORD PTR [r11 + 16], rdx
    mov rax, QWORD PTR [rbp - 72]
    mov QWORD PTR [rbp - 56], rax
    mov rax, 0
    mov rbx, rax
    mov r11, QWORD PTR [rbp - 56]
    sub rsp, 16
    mov QWORD PTR [rsp], r11
    mov rax, rbx
    mov r11, QWORD PTR [rsp]
    add rsp, 16
    mov QWORD PTR [r11 + 24], rax
    mov QWORD PTR [r11 + 32], 0
    jmp _class_propinit_19_epilogue
_class_propinit_19_epilogue:
    # restore callee-saved registers used by the register allocator
    mov rbx, QWORD PTR [rbp - 80]
    add rsp, 96
    pop rbp
    ret

.globl _class_propinit_24
_class_propinit_24:
    # prologue
    push rbp
    mov rbp, rsp
    sub rsp, 128
    mov r10, QWORD PTR [rip + _concat_off]
    mov QWORD PTR [rbp - 120], r10
    # save callee-saved registers used by the register allocator
    mov QWORD PTR [rbp - 112], rbx
    # param $this from rdi
    mov QWORD PTR [rbp - 104], rdi
_eir__class_propinit_24_entry_0:
    mov rax, QWORD PTR [rbp - 104]
    mov QWORD PTR [rbp - 24], rax
    lea rax, [rip + _str_0]
    mov rdx, 0
    mov QWORD PTR [rbp - 40], rax
    mov QWORD PTR [rbp - 32], rdx
    mov r11, QWORD PTR [rbp - 24]
    sub rsp, 16
    mov QWORD PTR [rsp], r11
    mov rax, QWORD PTR [rbp - 40]
    mov rdx, QWORD PTR [rbp - 32]
    call __rt_str_persist
    mov r11, QWORD PTR [rsp]
    add rsp, 16
    sub rsp, 16
    mov QWORD PTR [rsp], rax
    mov QWORD PTR [rsp + 8], rdx
    sub rsp, 16
    mov QWORD PTR [rsp], r11
    mov rax, QWORD PTR [r11 + 8]
    call __rt_heap_free_safe
    mov r11, QWORD PTR [rsp]
    add rsp, 16
    mov rax, QWORD PTR [rsp]
    mov rdx, QWORD PTR [rsp + 8]
    add rsp, 16
    mov QWORD PTR [r11 + 8], rax
    mov QWORD PTR [r11 + 16], rdx
    mov rax, QWORD PTR [rbp - 104]
    mov QWORD PTR [rbp - 56], rax
    mov rax, 0
    mov rbx, rax
    mov r11, QWORD PTR [rbp - 56]
    sub rsp, 16
    mov QWORD PTR [rsp], r11
    mov rax, rbx
    mov r11, QWORD PTR [rsp]
    add rsp, 16
    mov QWORD PTR [r11 + 24], rax
    mov QWORD PTR [r11 + 32], 0
    mov rax, QWORD PTR [rbp - 104]
    mov QWORD PTR [rbp - 80], rax
    mov rax, 0
    mov rbx, rax
    mov r11, QWORD PTR [rbp - 80]
    sub rsp, 16
    mov QWORD PTR [rsp], r11
    mov rax, rbx
    mov r11, QWORD PTR [rsp]
    add rsp, 16
    mov QWORD PTR [r11 + 40], rax
    mov QWORD PTR [r11 + 48], 0
    jmp _class_propinit_24_epilogue
_class_propinit_24_epilogue:
    # restore callee-saved registers used by the register allocator
    mov rbx, QWORD PTR [rbp - 112]
    add rsp, 128
    pop rbp
    ret

.globl _class_propinit_27
_class_propinit_27:
    # prologue
    push rbp
    mov rbp, rsp
    sub rsp, 64
    mov r10, QWORD PTR [rip + _concat_off]
    mov QWORD PTR [rbp - 56], r10
    # save callee-saved registers used by the register allocator
    mov QWORD PTR [rbp - 48], rbx
    # param $this from rdi
    mov QWORD PTR [rbp - 40], rdi
_eir__class_propinit_27_entry_0:
    mov rax, QWORD PTR [rbp - 40]
    mov QWORD PTR [rbp - 16], rax
    mov rax, 0
    mov rbx, rax
    mov r11, QWORD PTR [rbp - 16]
    sub rsp, 16
    mov QWORD PTR [rsp], r11
    mov rax, rbx
    mov r11, QWORD PTR [rsp]
    add rsp, 16
    mov QWORD PTR [r11 + 88], rax
    mov QWORD PTR [r11 + 96], 0
    jmp _class_propinit_27_epilogue
_class_propinit_27_epilogue:
    # restore callee-saved registers used by the register allocator
    mov rbx, QWORD PTR [rbp - 48]
    add rsp, 64
    pop rbp
    ret

.globl _class_propinit_29
_class_propinit_29:
    # prologue
    push rbp
    mov rbp, rsp
    sub rsp, 256
    mov r10, QWORD PTR [rip + _concat_off]
    mov QWORD PTR [rbp - 248], r10
    # save callee-saved registers used by the register allocator
    mov QWORD PTR [rbp - 240], rbx
    # param $this from rdi
    mov QWORD PTR [rbp - 232], rdi
_eir__class_propinit_29_entry_0:
    mov rax, QWORD PTR [rbp - 232]
    mov QWORD PTR [rbp - 16], rax
    mov rax, 0
    mov rbx, rax
    mov r11, QWORD PTR [rbp - 16]
    sub rsp, 16
    mov QWORD PTR [rsp], r11
    mov rax, rbx
    mov r11, QWORD PTR [rsp]
    add rsp, 16
    mov QWORD PTR [r11 + 8], rax
    mov QWORD PTR [r11 + 16], 0
    mov rax, QWORD PTR [rbp - 232]
    mov QWORD PTR [rbp - 40], rax
    mov rax, 0
    mov rbx, rax
    mov r11, QWORD PTR [rbp - 40]
    sub rsp, 16
    mov QWORD PTR [rsp], r11
    mov rax, rbx
    mov r11, QWORD PTR [rsp]
    add rsp, 16
    mov QWORD PTR [r11 + 24], rax
    mov QWORD PTR [r11 + 32], 0
    mov rax, QWORD PTR [rbp - 232]
    mov QWORD PTR [rbp - 64], rax
    mov rax, 0
    mov rbx, rax
    mov r11, QWORD PTR [rbp - 64]
    sub rsp, 16
    mov QWORD PTR [rsp], r11
    mov rax, rbx
    mov r11, QWORD PTR [rsp]
    add rsp, 16
    mov QWORD PTR [r11 + 40], rax
    mov QWORD PTR [r11 + 48], 0
    mov rax, QWORD PTR [rbp - 232]
    mov QWORD PTR [rbp - 88], rax
    mov rax, 0
    mov rbx, rax
    mov r11, QWORD PTR [rbp - 88]
    sub rsp, 16
    mov QWORD PTR [rsp], r11
    mov rax, rbx
    mov r11, QWORD PTR [rsp]
    add rsp, 16
    mov QWORD PTR [r11 + 56], rax
    mov QWORD PTR [r11 + 64], 0
    mov rax, QWORD PTR [rbp - 232]
    mov QWORD PTR [rbp - 112], rax
    mov rax, 0
    mov rbx, rax
    mov r11, QWORD PTR [rbp - 112]
    sub rsp, 16
    mov QWORD PTR [rsp], r11
    mov rax, rbx
    mov r11, QWORD PTR [rsp]
    add rsp, 16
    mov QWORD PTR [r11 + 72], rax
    mov QWORD PTR [r11 + 80], 0
    mov rax, QWORD PTR [rbp - 232]
    mov QWORD PTR [rbp - 136], rax
    mov rax, 0
    mov rbx, rax
    mov r11, QWORD PTR [rbp - 136]
    sub rsp, 16
    mov QWORD PTR [rsp], r11
    mov rax, rbx
    mov r11, QWORD PTR [rsp]
    add rsp, 16
    mov QWORD PTR [r11 + 88], rax
    mov QWORD PTR [r11 + 96], 0
    mov rax, QWORD PTR [rbp - 232]
    mov QWORD PTR [rbp - 160], rax
    lea r11, [rip + _float_2]
    movsd xmm0, QWORD PTR [r11]
    movsd QWORD PTR [rbp - 168], xmm0
    mov r11, QWORD PTR [rbp - 160]
    sub rsp, 16
    mov QWORD PTR [rsp], r11
    movsd xmm0, QWORD PTR [rbp - 168]
    mov r11, QWORD PTR [rsp]
    add rsp, 16
    movsd QWORD PTR [r11 + 104], xmm0
    mov QWORD PTR [r11 + 112], 0
    mov rax, QWORD PTR [rbp - 232]
    mov QWORD PTR [rbp - 184], rax
    mov rax, 0
    mov rbx, rax
    mov r11, QWORD PTR [rbp - 184]
    sub rsp, 16
    mov QWORD PTR [rsp], r11
    mov rax, rbx
    mov r11, QWORD PTR [rsp]
    add rsp, 16
    mov QWORD PTR [r11 + 120], rax
    mov QWORD PTR [r11 + 128], 0
    mov rax, QWORD PTR [rbp - 232]
    mov QWORD PTR [rbp - 208], rax
    mov rax, 0
    mov rbx, rax
    mov r11, QWORD PTR [rbp - 208]
    sub rsp, 16
    mov QWORD PTR [rsp], r11
    mov rax, rbx
    mov rdi, rax
    xor rsi, rsi
    mov rax, 3
    call __rt_mixed_from_value
    mov r11, QWORD PTR [rsp]
    add rsp, 16
    sub rsp, 16
    mov QWORD PTR [rsp], rax
    sub rsp, 16
    mov QWORD PTR [rsp], r11
    mov rax, QWORD PTR [r11 + 136]
    call __rt_decref_mixed
    mov r11, QWORD PTR [rsp]
    add rsp, 16
    mov rax, QWORD PTR [rsp]
    add rsp, 16
    mov QWORD PTR [r11 + 136], rax
    mov QWORD PTR [r11 + 144], 0
    jmp _class_propinit_29_epilogue
_class_propinit_29_epilogue:
    # restore callee-saved registers used by the register allocator
    mov rbx, QWORD PTR [rbp - 240]
    add rsp, 256
    pop rbp
    ret

.globl _class_propinit_31
_class_propinit_31:
    # prologue
    push rbp
    mov rbp, rsp
    sub rsp, 64
    mov r10, QWORD PTR [rip + _concat_off]
    mov QWORD PTR [rbp - 56], r10
    # save callee-saved registers used by the register allocator
    mov QWORD PTR [rbp - 48], rbx
    # param $this from rdi
    mov QWORD PTR [rbp - 40], rdi
_eir__class_propinit_31_entry_0:
    mov rax, QWORD PTR [rbp - 40]
    mov QWORD PTR [rbp - 16], rax
    mov rax, 9223372036854775806
    mov rbx, rax
    mov r11, QWORD PTR [rbp - 16]
    sub rsp, 16
    mov QWORD PTR [rsp], r11
    mov rax, QWORD PTR [r11 + 8]
    call __rt_decref_mixed
    mov r11, QWORD PTR [rsp]
    add rsp, 16
    mov QWORD PTR [r11 + 8], 0
    mov QWORD PTR [r11 + 16], 0
    jmp _class_propinit_31_epilogue
_class_propinit_31_epilogue:
    # restore callee-saved registers used by the register allocator
    mov rbx, QWORD PTR [rbp - 48]
    add rsp, 64
    pop rbp
    ret

.globl _class_propinit_32
_class_propinit_32:
    # prologue
    push rbp
    mov rbp, rsp
    sub rsp, 128
    mov r10, QWORD PTR [rip + _concat_off]
    mov QWORD PTR [rbp - 120], r10
    # save callee-saved registers used by the register allocator
    mov QWORD PTR [rbp - 112], rbx
    # param $this from rdi
    mov QWORD PTR [rbp - 104], rdi
_eir__class_propinit_32_entry_0:
    mov rax, QWORD PTR [rbp - 104]
    mov QWORD PTR [rbp - 16], rax
    mov rax, 0
    mov rbx, rax
    mov r11, QWORD PTR [rbp - 16]
    sub rsp, 16
    mov QWORD PTR [rsp], r11
    mov rax, rbx
    mov r11, QWORD PTR [rsp]
    add rsp, 16
    mov QWORD PTR [r11 + 8], rax
    mov QWORD PTR [r11 + 16], 0
    mov rax, QWORD PTR [rbp - 104]
    mov QWORD PTR [rbp - 48], rax
    lea rax, [rip + _str_1]
    mov rdx, 3
    mov QWORD PTR [rbp - 64], rax
    mov QWORD PTR [rbp - 56], rdx
    mov r11, QWORD PTR [rbp - 48]
    sub rsp, 16
    mov QWORD PTR [rsp], r11
    mov rax, QWORD PTR [rbp - 64]
    mov rdx, QWORD PTR [rbp - 56]
    call __rt_str_persist
    mov r11, QWORD PTR [rsp]
    add rsp, 16
    sub rsp, 16
    mov QWORD PTR [rsp], rax
    mov QWORD PTR [rsp + 8], rdx
    sub rsp, 16
    mov QWORD PTR [rsp], r11
    mov rax, QWORD PTR [r11 + 24]
    call __rt_heap_free_safe
    mov r11, QWORD PTR [rsp]
    add rsp, 16
    mov rax, QWORD PTR [rsp]
    mov rdx, QWORD PTR [rsp + 8]
    add rsp, 16
    mov QWORD PTR [r11 + 24], rax
    mov QWORD PTR [r11 + 32], rdx
    mov rax, QWORD PTR [rbp - 104]
    mov QWORD PTR [rbp - 80], rax
    mov rax, 0
    mov rbx, rax
    mov r11, QWORD PTR [rbp - 80]
    sub rsp, 16
    mov QWORD PTR [rsp], r11
    mov rax, rbx
    mov r11, QWORD PTR [rsp]
    add rsp, 16
    mov QWORD PTR [r11 + 40], rax
    mov QWORD PTR [r11 + 48], 0
    jmp _class_propinit_32_epilogue
_class_propinit_32_epilogue:
    # restore callee-saved registers used by the register allocator
    mov rbx, QWORD PTR [rbp - 112]
    add rsp, 128
    pop rbp
    ret

.globl _class_propinit_37
_class_propinit_37:
    # prologue
    push rbp
    mov rbp, rsp
    sub rsp, 96
    mov r10, QWORD PTR [rip + _concat_off]
    mov QWORD PTR [rbp - 88], r10
    # save callee-saved registers used by the register allocator
    mov QWORD PTR [rbp - 80], rbx
    # param $this from rdi
    mov QWORD PTR [rbp - 72], rdi
_eir__class_propinit_37_entry_0:
    mov rax, QWORD PTR [rbp - 72]
    mov QWORD PTR [rbp - 24], rax
    lea rax, [rip + _str_0]
    mov rdx, 0
    mov QWORD PTR [rbp - 40], rax
    mov QWORD PTR [rbp - 32], rdx
    mov r11, QWORD PTR [rbp - 24]
    sub rsp, 16
    mov QWORD PTR [rsp], r11
    mov rax, QWORD PTR [rbp - 40]
    mov rdx, QWORD PTR [rbp - 32]
    call __rt_str_persist
    mov r11, QWORD PTR [rsp]
    add rsp, 16
    sub rsp, 16
    mov QWORD PTR [rsp], rax
    mov QWORD PTR [rsp + 8], rdx
    sub rsp, 16
    mov QWORD PTR [rsp], r11
    mov rax, QWORD PTR [r11 + 8]
    call __rt_heap_free_safe
    mov r11, QWORD PTR [rsp]
    add rsp, 16
    mov rax, QWORD PTR [rsp]
    mov rdx, QWORD PTR [rsp + 8]
    add rsp, 16
    mov QWORD PTR [r11 + 8], rax
    mov QWORD PTR [r11 + 16], rdx
    mov rax, QWORD PTR [rbp - 72]
    mov QWORD PTR [rbp - 56], rax
    mov rax, 0
    mov rbx, rax
    mov r11, QWORD PTR [rbp - 56]
    sub rsp, 16
    mov QWORD PTR [rsp], r11
    mov rax, rbx
    mov r11, QWORD PTR [rsp]
    add rsp, 16
    mov QWORD PTR [r11 + 24], rax
    mov QWORD PTR [r11 + 32], 0
    jmp _class_propinit_37_epilogue
_class_propinit_37_epilogue:
    # restore callee-saved registers used by the register allocator
    mov rbx, QWORD PTR [rbp - 80]
    add rsp, 96
    pop rbp
    ret

.globl _class_propinit_39
_class_propinit_39:
    # prologue
    push rbp
    mov rbp, rsp
    sub rsp, 96
    mov r10, QWORD PTR [rip + _concat_off]
    mov QWORD PTR [rbp - 88], r10
    # save callee-saved registers used by the register allocator
    mov QWORD PTR [rbp - 80], rbx
    # param $this from rdi
    mov QWORD PTR [rbp - 72], rdi
_eir__class_propinit_39_entry_0:
    mov rax, QWORD PTR [rbp - 72]
    mov QWORD PTR [rbp - 24], rax
    lea rax, [rip + _str_0]
    mov rdx, 0
    mov QWORD PTR [rbp - 40], rax
    mov QWORD PTR [rbp - 32], rdx
    mov r11, QWORD PTR [rbp - 24]
    sub rsp, 16
    mov QWORD PTR [rsp], r11
    mov rax, QWORD PTR [rbp - 40]
    mov rdx, QWORD PTR [rbp - 32]
    call __rt_str_persist
    mov r11, QWORD PTR [rsp]
    add rsp, 16
    sub rsp, 16
    mov QWORD PTR [rsp], rax
    mov QWORD PTR [rsp + 8], rdx
    sub rsp, 16
    mov QWORD PTR [rsp], r11
    mov rax, QWORD PTR [r11 + 8]
    call __rt_heap_free_safe
    mov r11, QWORD PTR [rsp]
    add rsp, 16
    mov rax, QWORD PTR [rsp]
    mov rdx, QWORD PTR [rsp + 8]
    add rsp, 16
    mov QWORD PTR [r11 + 8], rax
    mov QWORD PTR [r11 + 16], rdx
    mov rax, QWORD PTR [rbp - 72]
    mov QWORD PTR [rbp - 56], rax
    mov rax, 0
    mov rbx, rax
    mov r11, QWORD PTR [rbp - 56]
    sub rsp, 16
    mov QWORD PTR [rsp], r11
    mov rax, rbx
    mov r11, QWORD PTR [rsp]
    add rsp, 16
    mov QWORD PTR [r11 + 24], rax
    mov QWORD PTR [r11 + 32], 0
    jmp _class_propinit_39_epilogue
_class_propinit_39_epilogue:
    # restore callee-saved registers used by the register allocator
    mov rbx, QWORD PTR [rbp - 80]
    add rsp, 96
    pop rbp
    ret

.globl _class_propinit_40
_class_propinit_40:
    # prologue
    push rbp
    mov rbp, rsp
    sub rsp, 48
    mov r10, QWORD PTR [rip + _concat_off]
    mov QWORD PTR [rbp - 48], r10
    # param $this from rdi
    mov QWORD PTR [rbp - 40], rdi
_eir__class_propinit_40_entry_0:
    mov rdi, 4
    mov rsi, 8
    call __rt_array_new
    sub rsp, 16
    mov QWORD PTR [rsp], r12
    mov r10, QWORD PTR [rax - 8]
    mov r12, 0xffffffff000080ff
    and r10, r12
    mov r12, 8
    shl r12, 8
    or r10, r12
    mov QWORD PTR [rax - 8], r10
    mov r12, QWORD PTR [rsp]
    add rsp, 16
    mov QWORD PTR [rbp - 8], rax
    mov rax, QWORD PTR [rbp - 40]
    mov QWORD PTR [rbp - 16], rax
    mov rdi, 4
    mov rsi, 8
    call __rt_array_new
    sub rsp, 16
    mov QWORD PTR [rsp], r12
    mov r10, QWORD PTR [rax - 8]
    mov r12, 0xffffffff000080ff
    and r10, r12
    mov r12, 8
    shl r12, 8
    or r10, r12
    mov QWORD PTR [rax - 8], r10
    mov r12, QWORD PTR [rsp]
    add rsp, 16
    mov QWORD PTR [rbp - 24], rax
    mov r11, QWORD PTR [rbp - 16]
    sub rsp, 16
    mov QWORD PTR [rsp], r11
    mov rax, QWORD PTR [rbp - 24]
    mov rsi, 8
    mov rdi, rax
    call __rt_array_to_mixed
    sub rsp, 16
    mov QWORD PTR [rsp], rax
    call __rt_incref
    mov rax, QWORD PTR [rsp]
    add rsp, 16
    mov r11, QWORD PTR [rsp]
    add rsp, 16
    sub rsp, 16
    mov QWORD PTR [rsp], rax
    sub rsp, 16
    mov QWORD PTR [rsp], r11
    mov rax, QWORD PTR [r11 + 8]
    call __rt_decref_array
    mov r11, QWORD PTR [rsp]
    add rsp, 16
    mov rax, QWORD PTR [rsp]
    add rsp, 16
    mov QWORD PTR [r11 + 8], rax
    mov QWORD PTR [r11 + 16], 0
    mov rax, QWORD PTR [rbp - 24]
    call __rt_decref_array
    mov rax, QWORD PTR [rbp - 8]
    call __rt_decref_array
    jmp _class_propinit_40_epilogue
_class_propinit_40_epilogue:
    add rsp, 48
    pop rbp
    ret

.globl _class_propinit_41
_class_propinit_41:
    # prologue
    push rbp
    mov rbp, rsp
    sub rsp, 96
    mov r10, QWORD PTR [rip + _concat_off]
    mov QWORD PTR [rbp - 88], r10
    # save callee-saved registers used by the register allocator
    mov QWORD PTR [rbp - 80], rbx
    # param $this from rdi
    mov QWORD PTR [rbp - 72], rdi
_eir__class_propinit_41_entry_0:
    mov rax, QWORD PTR [rbp - 72]
    mov QWORD PTR [rbp - 24], rax
    lea rax, [rip + _str_0]
    mov rdx, 0
    mov QWORD PTR [rbp - 40], rax
    mov QWORD PTR [rbp - 32], rdx
    mov r11, QWORD PTR [rbp - 24]
    sub rsp, 16
    mov QWORD PTR [rsp], r11
    mov rax, QWORD PTR [rbp - 40]
    mov rdx, QWORD PTR [rbp - 32]
    call __rt_str_persist
    mov r11, QWORD PTR [rsp]
    add rsp, 16
    sub rsp, 16
    mov QWORD PTR [rsp], rax
    mov QWORD PTR [rsp + 8], rdx
    sub rsp, 16
    mov QWORD PTR [rsp], r11
    mov rax, QWORD PTR [r11 + 8]
    call __rt_heap_free_safe
    mov r11, QWORD PTR [rsp]
    add rsp, 16
    mov rax, QWORD PTR [rsp]
    mov rdx, QWORD PTR [rsp + 8]
    add rsp, 16
    mov QWORD PTR [r11 + 8], rax
    mov QWORD PTR [r11 + 16], rdx
    mov rax, QWORD PTR [rbp - 72]
    mov QWORD PTR [rbp - 56], rax
    mov rax, 0
    mov rbx, rax
    mov r11, QWORD PTR [rbp - 56]
    sub rsp, 16
    mov QWORD PTR [rsp], r11
    mov rax, rbx
    mov r11, QWORD PTR [rsp]
    add rsp, 16
    mov QWORD PTR [r11 + 24], rax
    mov QWORD PTR [r11 + 32], 0
    jmp _class_propinit_41_epilogue
_class_propinit_41_epilogue:
    # restore callee-saved registers used by the register allocator
    mov rbx, QWORD PTR [rbp - 80]
    add rsp, 96
    pop rbp
    ret

.globl _class_propinit_42
_class_propinit_42:
    # prologue
    push rbp
    mov rbp, rsp
    sub rsp, 96
    mov r10, QWORD PTR [rip + _concat_off]
    mov QWORD PTR [rbp - 88], r10
    # save callee-saved registers used by the register allocator
    mov QWORD PTR [rbp - 80], rbx
    # param $this from rdi
    mov QWORD PTR [rbp - 72], rdi
_eir__class_propinit_42_entry_0:
    mov rax, QWORD PTR [rbp - 72]
    mov QWORD PTR [rbp - 24], rax
    lea rax, [rip + _str_0]
    mov rdx, 0
    mov QWORD PTR [rbp - 40], rax
    mov QWORD PTR [rbp - 32], rdx
    mov r11, QWORD PTR [rbp - 24]
    sub rsp, 16
    mov QWORD PTR [rsp], r11
    mov rax, QWORD PTR [rbp - 40]
    mov rdx, QWORD PTR [rbp - 32]
    call __rt_str_persist
    mov r11, QWORD PTR [rsp]
    add rsp, 16
    sub rsp, 16
    mov QWORD PTR [rsp], rax
    mov QWORD PTR [rsp + 8], rdx
    sub rsp, 16
    mov QWORD PTR [rsp], r11
    mov rax, QWORD PTR [r11 + 8]
    call __rt_heap_free_safe
    mov r11, QWORD PTR [rsp]
    add rsp, 16
    mov rax, QWORD PTR [rsp]
    mov rdx, QWORD PTR [rsp + 8]
    add rsp, 16
    mov QWORD PTR [r11 + 8], rax
    mov QWORD PTR [r11 + 16], rdx
    mov rax, QWORD PTR [rbp - 72]
    mov QWORD PTR [rbp - 56], rax
    mov rax, 0
    mov rbx, rax
    mov r11, QWORD PTR [rbp - 56]
    sub rsp, 16
    mov QWORD PTR [rsp], r11
    mov rax, rbx
    mov r11, QWORD PTR [rsp]
    add rsp, 16
    mov QWORD PTR [r11 + 24], rax
    mov QWORD PTR [r11 + 32], 0
    jmp _class_propinit_42_epilogue
_class_propinit_42_epilogue:
    # restore callee-saved registers used by the register allocator
    mov rbx, QWORD PTR [rbp - 80]
    add rsp, 96
    pop rbp
    ret

.globl _class_propinit_43
_class_propinit_43:
    # prologue
    push rbp
    mov rbp, rsp
    sub rsp, 96
    mov r10, QWORD PTR [rip + _concat_off]
    mov QWORD PTR [rbp - 88], r10
    # save callee-saved registers used by the register allocator
    mov QWORD PTR [rbp - 80], rbx
    # param $this from rdi
    mov QWORD PTR [rbp - 72], rdi
_eir__class_propinit_43_entry_0:
    mov rax, QWORD PTR [rbp - 72]
    mov QWORD PTR [rbp - 24], rax
    lea rax, [rip + _str_0]
    mov rdx, 0
    mov QWORD PTR [rbp - 40], rax
    mov QWORD PTR [rbp - 32], rdx
    mov r11, QWORD PTR [rbp - 24]
    sub rsp, 16
    mov QWORD PTR [rsp], r11
    mov rax, QWORD PTR [rbp - 40]
    mov rdx, QWORD PTR [rbp - 32]
    call __rt_str_persist
    mov r11, QWORD PTR [rsp]
    add rsp, 16
    sub rsp, 16
    mov QWORD PTR [rsp], rax
    mov QWORD PTR [rsp + 8], rdx
    sub rsp, 16
    mov QWORD PTR [rsp], r11
    mov rax, QWORD PTR [r11 + 8]
    call __rt_heap_free_safe
    mov r11, QWORD PTR [rsp]
    add rsp, 16
    mov rax, QWORD PTR [rsp]
    mov rdx, QWORD PTR [rsp + 8]
    add rsp, 16
    mov QWORD PTR [r11 + 8], rax
    mov QWORD PTR [r11 + 16], rdx
    mov rax, QWORD PTR [rbp - 72]
    mov QWORD PTR [rbp - 56], rax
    mov rax, 0
    mov rbx, rax
    mov r11, QWORD PTR [rbp - 56]
    sub rsp, 16
    mov QWORD PTR [rsp], r11
    mov rax, rbx
    mov r11, QWORD PTR [rsp]
    add rsp, 16
    mov QWORD PTR [r11 + 24], rax
    mov QWORD PTR [r11 + 32], 0
    jmp _class_propinit_43_epilogue
_class_propinit_43_epilogue:
    # restore callee-saved registers used by the register allocator
    mov rbx, QWORD PTR [rbp - 80]
    add rsp, 96
    pop rbp
    ret

.globl _class_propinit_46
_class_propinit_46:
    # prologue
    push rbp
    mov rbp, rsp
    sub rsp, 96
    mov r10, QWORD PTR [rip + _concat_off]
    mov QWORD PTR [rbp - 88], r10
    # save callee-saved registers used by the register allocator
    mov QWORD PTR [rbp - 80], rbx
    # param $this from rdi
    mov QWORD PTR [rbp - 72], rdi
_eir__class_propinit_46_entry_0:
    mov rax, QWORD PTR [rbp - 72]
    mov QWORD PTR [rbp - 24], rax
    lea rax, [rip + _str_0]
    mov rdx, 0
    mov QWORD PTR [rbp - 40], rax
    mov QWORD PTR [rbp - 32], rdx
    mov r11, QWORD PTR [rbp - 24]
    sub rsp, 16
    mov QWORD PTR [rsp], r11
    mov rax, QWORD PTR [rbp - 40]
    mov rdx, QWORD PTR [rbp - 32]
    call __rt_str_persist
    mov r11, QWORD PTR [rsp]
    add rsp, 16
    sub rsp, 16
    mov QWORD PTR [rsp], rax
    mov QWORD PTR [rsp + 8], rdx
    sub rsp, 16
    mov QWORD PTR [rsp], r11
    mov rax, QWORD PTR [r11 + 8]
    call __rt_heap_free_safe
    mov r11, QWORD PTR [rsp]
    add rsp, 16
    mov rax, QWORD PTR [rsp]
    mov rdx, QWORD PTR [rsp + 8]
    add rsp, 16
    mov QWORD PTR [r11 + 8], rax
    mov QWORD PTR [r11 + 16], rdx
    mov rax, QWORD PTR [rbp - 72]
    mov QWORD PTR [rbp - 56], rax
    mov rax, 0
    mov rbx, rax
    mov r11, QWORD PTR [rbp - 56]
    sub rsp, 16
    mov QWORD PTR [rsp], r11
    mov rax, rbx
    mov r11, QWORD PTR [rsp]
    add rsp, 16
    mov QWORD PTR [r11 + 24], rax
    mov QWORD PTR [r11 + 32], 0
    jmp _class_propinit_46_epilogue
_class_propinit_46_epilogue:
    # restore callee-saved registers used by the register allocator
    mov rbx, QWORD PTR [rbp - 80]
    add rsp, 96
    pop rbp
    ret

.globl _class_propinit_47
_class_propinit_47:
    # prologue
    push rbp
    mov rbp, rsp
    sub rsp, 96
    mov r10, QWORD PTR [rip + _concat_off]
    mov QWORD PTR [rbp - 88], r10
    # save callee-saved registers used by the register allocator
    mov QWORD PTR [rbp - 80], rbx
    # param $this from rdi
    mov QWORD PTR [rbp - 72], rdi
_eir__class_propinit_47_entry_0:
    mov rax, QWORD PTR [rbp - 72]
    mov QWORD PTR [rbp - 24], rax
    lea rax, [rip + _str_0]
    mov rdx, 0
    mov QWORD PTR [rbp - 40], rax
    mov QWORD PTR [rbp - 32], rdx
    mov r11, QWORD PTR [rbp - 24]
    sub rsp, 16
    mov QWORD PTR [rsp], r11
    mov rax, QWORD PTR [rbp - 40]
    mov rdx, QWORD PTR [rbp - 32]
    call __rt_str_persist
    mov r11, QWORD PTR [rsp]
    add rsp, 16
    sub rsp, 16
    mov QWORD PTR [rsp], rax
    mov QWORD PTR [rsp + 8], rdx
    sub rsp, 16
    mov QWORD PTR [rsp], r11
    mov rax, QWORD PTR [r11 + 8]
    call __rt_heap_free_safe
    mov r11, QWORD PTR [rsp]
    add rsp, 16
    mov rax, QWORD PTR [rsp]
    mov rdx, QWORD PTR [rsp + 8]
    add rsp, 16
    mov QWORD PTR [r11 + 8], rax
    mov QWORD PTR [r11 + 16], rdx
    mov rax, QWORD PTR [rbp - 72]
    mov QWORD PTR [rbp - 56], rax
    mov rax, 0
    mov rbx, rax
    mov r11, QWORD PTR [rbp - 56]
    sub rsp, 16
    mov QWORD PTR [rsp], r11
    mov rax, rbx
    mov r11, QWORD PTR [rsp]
    add rsp, 16
    mov QWORD PTR [r11 + 24], rax
    mov QWORD PTR [r11 + 32], 0
    jmp _class_propinit_47_epilogue
_class_propinit_47_epilogue:
    # restore callee-saved registers used by the register allocator
    mov rbx, QWORD PTR [rbp - 80]
    add rsp, 96
    pop rbp
    ret

.globl _class_propinit_48
_class_propinit_48:
    # prologue
    push rbp
    mov rbp, rsp
    sub rsp, 96
    mov r10, QWORD PTR [rip + _concat_off]
    mov QWORD PTR [rbp - 88], r10
    # save callee-saved registers used by the register allocator
    mov QWORD PTR [rbp - 80], rbx
    # param $this from rdi
    mov QWORD PTR [rbp - 72], rdi
_eir__class_propinit_48_entry_0:
    mov rax, QWORD PTR [rbp - 72]
    mov QWORD PTR [rbp - 24], rax
    lea rax, [rip + _str_0]
    mov rdx, 0
    mov QWORD PTR [rbp - 40], rax
    mov QWORD PTR [rbp - 32], rdx
    mov r11, QWORD PTR [rbp - 24]
    sub rsp, 16
    mov QWORD PTR [rsp], r11
    mov rax, QWORD PTR [rbp - 40]
    mov rdx, QWORD PTR [rbp - 32]
    call __rt_str_persist
    mov r11, QWORD PTR [rsp]
    add rsp, 16
    sub rsp, 16
    mov QWORD PTR [rsp], rax
    mov QWORD PTR [rsp + 8], rdx
    sub rsp, 16
    mov QWORD PTR [rsp], r11
    mov rax, QWORD PTR [r11 + 8]
    call __rt_heap_free_safe
    mov r11, QWORD PTR [rsp]
    add rsp, 16
    mov rax, QWORD PTR [rsp]
    mov rdx, QWORD PTR [rsp + 8]
    add rsp, 16
    mov QWORD PTR [r11 + 8], rax
    mov QWORD PTR [r11 + 16], rdx
    mov rax, QWORD PTR [rbp - 72]
    mov QWORD PTR [rbp - 56], rax
    mov rax, 0
    mov rbx, rax
    mov r11, QWORD PTR [rbp - 56]
    sub rsp, 16
    mov QWORD PTR [rsp], r11
    mov rax, rbx
    mov r11, QWORD PTR [rsp]
    add rsp, 16
    mov QWORD PTR [r11 + 24], rax
    mov QWORD PTR [r11 + 32], 0
    jmp _class_propinit_48_epilogue
_class_propinit_48_epilogue:
    # restore callee-saved registers used by the register allocator
    mov rbx, QWORD PTR [rbp - 80]
    add rsp, 96
    pop rbp
    ret

.globl _class_propinit_52
_class_propinit_52:
    # prologue
    push rbp
    mov rbp, rsp
    sub rsp, 64
    mov r10, QWORD PTR [rip + _concat_off]
    mov QWORD PTR [rbp - 56], r10
    # save callee-saved registers used by the register allocator
    mov QWORD PTR [rbp - 48], rbx
    # param $this from rdi
    mov QWORD PTR [rbp - 40], rdi
_eir__class_propinit_52_entry_0:
    mov rax, QWORD PTR [rbp - 40]
    mov QWORD PTR [rbp - 16], rax
    mov rax, 9223372036854775806
    mov rbx, rax
    mov r11, QWORD PTR [rbp - 16]
    sub rsp, 16
    mov QWORD PTR [rsp], r11
    mov rax, QWORD PTR [r11 + 88]
    call __rt_decref_mixed
    mov r11, QWORD PTR [rsp]
    add rsp, 16
    mov QWORD PTR [r11 + 88], 0
    mov QWORD PTR [r11 + 96], 0
    jmp _class_propinit_52_epilogue
_class_propinit_52_epilogue:
    # restore callee-saved registers used by the register allocator
    mov rbx, QWORD PTR [rbp - 48]
    add rsp, 64
    pop rbp
    ret

.globl _class_propinit_53
_class_propinit_53:
    # prologue
    push rbp
    mov rbp, rsp
    sub rsp, 64
    mov r10, QWORD PTR [rip + _concat_off]
    mov QWORD PTR [rbp - 56], r10
    # save callee-saved registers used by the register allocator
    mov QWORD PTR [rbp - 48], rbx
    # param $this from rdi
    mov QWORD PTR [rbp - 40], rdi
_eir__class_propinit_53_entry_0:
    mov rax, QWORD PTR [rbp - 40]
    mov QWORD PTR [rbp - 16], rax
    mov rax, 9223372036854775806
    mov rbx, rax
    mov r11, QWORD PTR [rbp - 16]
    sub rsp, 16
    mov QWORD PTR [rsp], r11
    mov rax, QWORD PTR [r11 + 88]
    call __rt_decref_mixed
    mov r11, QWORD PTR [rsp]
    add rsp, 16
    mov QWORD PTR [r11 + 88], 0
    mov QWORD PTR [r11 + 96], 0
    jmp _class_propinit_53_epilogue
_class_propinit_53_epilogue:
    # restore callee-saved registers used by the register allocator
    mov rbx, QWORD PTR [rbp - 48]
    add rsp, 64
    pop rbp
    ret

.globl _class_propinit_54
_class_propinit_54:
    # prologue
    push rbp
    mov rbp, rsp
    sub rsp, 80
    mov r10, QWORD PTR [rip + _concat_off]
    mov QWORD PTR [rbp - 80], r10
    # param $this from rdi
    mov QWORD PTR [rbp - 72], rdi
_eir__class_propinit_54_entry_0:
    mov rax, QWORD PTR [rbp - 72]
    mov QWORD PTR [rbp - 24], rax
    lea rax, [rip + _str_0]
    mov rdx, 0
    mov QWORD PTR [rbp - 40], rax
    mov QWORD PTR [rbp - 32], rdx
    mov r11, QWORD PTR [rbp - 24]
    sub rsp, 16
    mov QWORD PTR [rsp], r11
    mov rax, QWORD PTR [rbp - 40]
    mov rdx, QWORD PTR [rbp - 32]
    call __rt_str_persist
    mov r11, QWORD PTR [rsp]
    add rsp, 16
    sub rsp, 16
    mov QWORD PTR [rsp], rax
    mov QWORD PTR [rsp + 8], rdx
    sub rsp, 16
    mov QWORD PTR [rsp], r11
    mov rax, QWORD PTR [r11 + 8]
    call __rt_heap_free_safe
    mov r11, QWORD PTR [rsp]
    add rsp, 16
    mov rax, QWORD PTR [rsp]
    mov rdx, QWORD PTR [rsp + 8]
    add rsp, 16
    mov QWORD PTR [r11 + 8], rax
    mov QWORD PTR [r11 + 16], rdx
    mov rdi, 4
    mov rsi, 8
    call __rt_array_new
    sub rsp, 16
    mov QWORD PTR [rsp], r12
    mov r10, QWORD PTR [rax - 8]
    mov r12, 0xffffffff000080ff
    and r10, r12
    mov r12, 8
    shl r12, 8
    or r10, r12
    mov QWORD PTR [rax - 8], r10
    mov r12, QWORD PTR [rsp]
    add rsp, 16
    mov QWORD PTR [rbp - 48], rax
    mov rax, QWORD PTR [rbp - 72]
    mov QWORD PTR [rbp - 56], rax
    mov rdi, 4
    mov rsi, 8
    call __rt_array_new
    sub rsp, 16
    mov QWORD PTR [rsp], r12
    mov r10, QWORD PTR [rax - 8]
    mov r12, 0xffffffff000080ff
    and r10, r12
    mov r12, 8
    shl r12, 8
    or r10, r12
    mov QWORD PTR [rax - 8], r10
    mov r12, QWORD PTR [rsp]
    add rsp, 16
    mov QWORD PTR [rbp - 64], rax
    mov r11, QWORD PTR [rbp - 56]
    sub rsp, 16
    mov QWORD PTR [rsp], r11
    mov rax, QWORD PTR [rbp - 64]
    mov rsi, 8
    mov rdi, rax
    call __rt_array_to_mixed
    sub rsp, 16
    mov QWORD PTR [rsp], rax
    call __rt_incref
    mov rax, QWORD PTR [rsp]
    add rsp, 16
    mov r11, QWORD PTR [rsp]
    add rsp, 16
    sub rsp, 16
    mov QWORD PTR [rsp], rax
    sub rsp, 16
    mov QWORD PTR [rsp], r11
    mov rax, QWORD PTR [r11 + 24]
    call __rt_decref_array
    mov r11, QWORD PTR [rsp]
    add rsp, 16
    mov rax, QWORD PTR [rsp]
    add rsp, 16
    mov QWORD PTR [r11 + 24], rax
    mov QWORD PTR [r11 + 32], 0
    mov rax, QWORD PTR [rbp - 64]
    call __rt_decref_array
    mov rax, QWORD PTR [rbp - 48]
    call __rt_decref_array
    jmp _class_propinit_54_epilogue
_class_propinit_54_epilogue:
    add rsp, 80
    pop rbp
    ret

.globl _class_propinit_56
_class_propinit_56:
    # prologue
    push rbp
    mov rbp, rsp
    sub rsp, 96
    mov r10, QWORD PTR [rip + _concat_off]
    mov QWORD PTR [rbp - 88], r10
    # save callee-saved registers used by the register allocator
    mov QWORD PTR [rbp - 80], rbx
    # param $this from rdi
    mov QWORD PTR [rbp - 72], rdi
_eir__class_propinit_56_entry_0:
    mov rax, QWORD PTR [rbp - 72]
    mov QWORD PTR [rbp - 24], rax
    lea rax, [rip + _str_0]
    mov rdx, 0
    mov QWORD PTR [rbp - 40], rax
    mov QWORD PTR [rbp - 32], rdx
    mov r11, QWORD PTR [rbp - 24]
    sub rsp, 16
    mov QWORD PTR [rsp], r11
    mov rax, QWORD PTR [rbp - 40]
    mov rdx, QWORD PTR [rbp - 32]
    call __rt_str_persist
    mov r11, QWORD PTR [rsp]
    add rsp, 16
    sub rsp, 16
    mov QWORD PTR [rsp], rax
    mov QWORD PTR [rsp + 8], rdx
    sub rsp, 16
    mov QWORD PTR [rsp], r11
    mov rax, QWORD PTR [r11 + 8]
    call __rt_heap_free_safe
    mov r11, QWORD PTR [rsp]
    add rsp, 16
    mov rax, QWORD PTR [rsp]
    mov rdx, QWORD PTR [rsp + 8]
    add rsp, 16
    mov QWORD PTR [r11 + 8], rax
    mov QWORD PTR [r11 + 16], rdx
    mov rax, QWORD PTR [rbp - 72]
    mov QWORD PTR [rbp - 56], rax
    mov rax, 0
    mov rbx, rax
    mov r11, QWORD PTR [rbp - 56]
    sub rsp, 16
    mov QWORD PTR [rsp], r11
    mov rax, rbx
    mov r11, QWORD PTR [rsp]
    add rsp, 16
    mov QWORD PTR [r11 + 24], rax
    mov QWORD PTR [r11 + 32], 0
    jmp _class_propinit_56_epilogue
_class_propinit_56_epilogue:
    # restore callee-saved registers used by the register allocator
    mov rbx, QWORD PTR [rbp - 80]
    add rsp, 96
    pop rbp
    ret

.globl _class_propinit_58
_class_propinit_58:
    # prologue
    push rbp
    mov rbp, rsp
    sub rsp, 96
    mov r10, QWORD PTR [rip + _concat_off]
    mov QWORD PTR [rbp - 88], r10
    # save callee-saved registers used by the register allocator
    mov QWORD PTR [rbp - 80], rbx
    # param $this from rdi
    mov QWORD PTR [rbp - 72], rdi
_eir__class_propinit_58_entry_0:
    mov rax, QWORD PTR [rbp - 72]
    mov QWORD PTR [rbp - 24], rax
    lea rax, [rip + _str_0]
    mov rdx, 0
    mov QWORD PTR [rbp - 40], rax
    mov QWORD PTR [rbp - 32], rdx
    mov r11, QWORD PTR [rbp - 24]
    sub rsp, 16
    mov QWORD PTR [rsp], r11
    mov rax, QWORD PTR [rbp - 40]
    mov rdx, QWORD PTR [rbp - 32]
    call __rt_str_persist
    mov r11, QWORD PTR [rsp]
    add rsp, 16
    sub rsp, 16
    mov QWORD PTR [rsp], rax
    mov QWORD PTR [rsp + 8], rdx
    sub rsp, 16
    mov QWORD PTR [rsp], r11
    mov rax, QWORD PTR [r11 + 8]
    call __rt_heap_free_safe
    mov r11, QWORD PTR [rsp]
    add rsp, 16
    mov rax, QWORD PTR [rsp]
    mov rdx, QWORD PTR [rsp + 8]
    add rsp, 16
    mov QWORD PTR [r11 + 8], rax
    mov QWORD PTR [r11 + 16], rdx
    mov rax, QWORD PTR [rbp - 72]
    mov QWORD PTR [rbp - 56], rax
    mov rax, 0
    mov rbx, rax
    mov r11, QWORD PTR [rbp - 56]
    sub rsp, 16
    mov QWORD PTR [rsp], r11
    mov rax, rbx
    mov r11, QWORD PTR [rsp]
    add rsp, 16
    mov QWORD PTR [r11 + 24], rax
    mov QWORD PTR [r11 + 32], 0
    jmp _class_propinit_58_epilogue
_class_propinit_58_epilogue:
    # restore callee-saved registers used by the register allocator
    mov rbx, QWORD PTR [rbp - 80]
    add rsp, 96
    pop rbp
    ret

.globl _class_propinit_59
_class_propinit_59:
    # prologue
    push rbp
    mov rbp, rsp
    sub rsp, 96
    mov r10, QWORD PTR [rip + _concat_off]
    mov QWORD PTR [rbp - 88], r10
    # save callee-saved registers used by the register allocator
    mov QWORD PTR [rbp - 80], rbx
    # param $this from rdi
    mov QWORD PTR [rbp - 72], rdi
_eir__class_propinit_59_entry_0:
    mov rax, QWORD PTR [rbp - 72]
    mov QWORD PTR [rbp - 24], rax
    lea rax, [rip + _str_0]
    mov rdx, 0
    mov QWORD PTR [rbp - 40], rax
    mov QWORD PTR [rbp - 32], rdx
    mov r11, QWORD PTR [rbp - 24]
    sub rsp, 16
    mov QWORD PTR [rsp], r11
    mov rax, QWORD PTR [rbp - 40]
    mov rdx, QWORD PTR [rbp - 32]
    call __rt_str_persist
    mov r11, QWORD PTR [rsp]
    add rsp, 16
    sub rsp, 16
    mov QWORD PTR [rsp], rax
    mov QWORD PTR [rsp + 8], rdx
    sub rsp, 16
    mov QWORD PTR [rsp], r11
    mov rax, QWORD PTR [r11 + 8]
    call __rt_heap_free_safe
    mov r11, QWORD PTR [rsp]
    add rsp, 16
    mov rax, QWORD PTR [rsp]
    mov rdx, QWORD PTR [rsp + 8]
    add rsp, 16
    mov QWORD PTR [r11 + 8], rax
    mov QWORD PTR [r11 + 16], rdx
    mov rax, QWORD PTR [rbp - 72]
    mov QWORD PTR [rbp - 56], rax
    mov rax, 0
    mov rbx, rax
    mov r11, QWORD PTR [rbp - 56]
    sub rsp, 16
    mov QWORD PTR [rsp], r11
    mov rax, rbx
    mov r11, QWORD PTR [rsp]
    add rsp, 16
    mov QWORD PTR [r11 + 24], rax
    mov QWORD PTR [r11 + 32], 0
    jmp _class_propinit_59_epilogue
_class_propinit_59_epilogue:
    # restore callee-saved registers used by the register allocator
    mov rbx, QWORD PTR [rbp - 80]
    add rsp, 96
    pop rbp
    ret

.globl _class_propinit_60
_class_propinit_60:
    # prologue
    push rbp
    mov rbp, rsp
    sub rsp, 128
    mov r10, QWORD PTR [rip + _concat_off]
    mov QWORD PTR [rbp - 120], r10
    # save callee-saved registers used by the register allocator
    mov QWORD PTR [rbp - 112], rbx
    # param $this from rdi
    mov QWORD PTR [rbp - 104], rdi
_eir__class_propinit_60_entry_0:
    mov rax, QWORD PTR [rbp - 104]
    mov QWORD PTR [rbp - 24], rax
    lea rax, [rip + _str_0]
    mov rdx, 0
    mov QWORD PTR [rbp - 40], rax
    mov QWORD PTR [rbp - 32], rdx
    mov r11, QWORD PTR [rbp - 24]
    sub rsp, 16
    mov QWORD PTR [rsp], r11
    mov rax, QWORD PTR [rbp - 40]
    mov rdx, QWORD PTR [rbp - 32]
    call __rt_str_persist
    mov r11, QWORD PTR [rsp]
    add rsp, 16
    sub rsp, 16
    mov QWORD PTR [rsp], rax
    mov QWORD PTR [rsp + 8], rdx
    sub rsp, 16
    mov QWORD PTR [rsp], r11
    mov rax, QWORD PTR [r11 + 8]
    call __rt_heap_free_safe
    mov r11, QWORD PTR [rsp]
    add rsp, 16
    mov rax, QWORD PTR [rsp]
    mov rdx, QWORD PTR [rsp + 8]
    add rsp, 16
    mov QWORD PTR [r11 + 8], rax
    mov QWORD PTR [r11 + 16], rdx
    mov rdi, 4
    mov rsi, 8
    call __rt_array_new
    sub rsp, 16
    mov QWORD PTR [rsp], r12
    mov r10, QWORD PTR [rax - 8]
    mov r12, 0xffffffff000080ff
    and r10, r12
    mov r12, 8
    shl r12, 8
    or r10, r12
    mov QWORD PTR [rax - 8], r10
    mov r12, QWORD PTR [rsp]
    add rsp, 16
    mov QWORD PTR [rbp - 48], rax
    mov rax, QWORD PTR [rbp - 104]
    mov QWORD PTR [rbp - 56], rax
    mov rdi, 4
    mov rsi, 8
    call __rt_array_new
    sub rsp, 16
    mov QWORD PTR [rsp], r12
    mov r10, QWORD PTR [rax - 8]
    mov r12, 0xffffffff000080ff
    and r10, r12
    mov r12, 8
    shl r12, 8
    or r10, r12
    mov QWORD PTR [rax - 8], r10
    mov r12, QWORD PTR [rsp]
    add rsp, 16
    mov QWORD PTR [rbp - 64], rax
    mov r11, QWORD PTR [rbp - 56]
    sub rsp, 16
    mov QWORD PTR [rsp], r11
    mov rax, QWORD PTR [rbp - 64]
    mov rsi, 8
    mov rdi, rax
    call __rt_array_to_mixed
    sub rsp, 16
    mov QWORD PTR [rsp], rax
    call __rt_incref
    mov rax, QWORD PTR [rsp]
    add rsp, 16
    mov r11, QWORD PTR [rsp]
    add rsp, 16
    sub rsp, 16
    mov QWORD PTR [rsp], rax
    sub rsp, 16
    mov QWORD PTR [rsp], r11
    mov rax, QWORD PTR [r11 + 24]
    call __rt_decref_array
    mov r11, QWORD PTR [rsp]
    add rsp, 16
    mov rax, QWORD PTR [rsp]
    add rsp, 16
    mov QWORD PTR [r11 + 24], rax
    mov QWORD PTR [r11 + 32], 0
    mov rax, QWORD PTR [rbp - 64]
    call __rt_decref_array
    mov rax, QWORD PTR [rbp - 48]
    call __rt_decref_array
    mov rax, QWORD PTR [rbp - 104]
    mov QWORD PTR [rbp - 80], rax
    mov rax, 0
    mov rbx, rax
    mov r11, QWORD PTR [rbp - 80]
    sub rsp, 16
    mov QWORD PTR [rsp], r11
    mov rax, rbx
    mov r11, QWORD PTR [rsp]
    add rsp, 16
    mov QWORD PTR [r11 + 40], rax
    mov QWORD PTR [r11 + 48], 0
    jmp _class_propinit_60_epilogue
_class_propinit_60_epilogue:
    # restore callee-saved registers used by the register allocator
    mov rbx, QWORD PTR [rbp - 112]
    add rsp, 128
    pop rbp
    ret

.globl _class_propinit_62
_class_propinit_62:
    # prologue
    push rbp
    mov rbp, rsp
    sub rsp, 128
    mov r10, QWORD PTR [rip + _concat_off]
    mov QWORD PTR [rbp - 120], r10
    # save callee-saved registers used by the register allocator
    mov QWORD PTR [rbp - 112], rbx
    # param $this from rdi
    mov QWORD PTR [rbp - 104], rdi
_eir__class_propinit_62_entry_0:
    mov rax, QWORD PTR [rbp - 104]
    mov QWORD PTR [rbp - 16], rax
    mov rax, 0
    mov rbx, rax
    mov r11, QWORD PTR [rbp - 16]
    sub rsp, 16
    mov QWORD PTR [rsp], r11
    mov rax, rbx
    mov r11, QWORD PTR [rsp]
    add rsp, 16
    mov QWORD PTR [r11 + 8], rax
    mov QWORD PTR [r11 + 16], 0
    mov rax, QWORD PTR [rbp - 104]
    mov QWORD PTR [rbp - 48], rax
    lea rax, [rip + _str_1]
    mov rdx, 3
    mov QWORD PTR [rbp - 64], rax
    mov QWORD PTR [rbp - 56], rdx
    mov r11, QWORD PTR [rbp - 48]
    sub rsp, 16
    mov QWORD PTR [rsp], r11
    mov rax, QWORD PTR [rbp - 64]
    mov rdx, QWORD PTR [rbp - 56]
    call __rt_str_persist
    mov r11, QWORD PTR [rsp]
    add rsp, 16
    sub rsp, 16
    mov QWORD PTR [rsp], rax
    mov QWORD PTR [rsp + 8], rdx
    sub rsp, 16
    mov QWORD PTR [rsp], r11
    mov rax, QWORD PTR [r11 + 24]
    call __rt_heap_free_safe
    mov r11, QWORD PTR [rsp]
    add rsp, 16
    mov rax, QWORD PTR [rsp]
    mov rdx, QWORD PTR [rsp + 8]
    add rsp, 16
    mov QWORD PTR [r11 + 24], rax
    mov QWORD PTR [r11 + 32], rdx
    mov rax, QWORD PTR [rbp - 104]
    mov QWORD PTR [rbp - 80], rax
    mov rax, 0
    mov rbx, rax
    mov r11, QWORD PTR [rbp - 80]
    sub rsp, 16
    mov QWORD PTR [rsp], r11
    mov rax, rbx
    mov r11, QWORD PTR [rsp]
    add rsp, 16
    mov QWORD PTR [r11 + 40], rax
    mov QWORD PTR [r11 + 48], 0
    jmp _class_propinit_62_epilogue
_class_propinit_62_epilogue:
    # restore callee-saved registers used by the register allocator
    mov rbx, QWORD PTR [rbp - 112]
    add rsp, 128
    pop rbp
    ret

.globl _class_propinit_66
_class_propinit_66:
    # prologue
    push rbp
    mov rbp, rsp
    sub rsp, 96
    mov r10, QWORD PTR [rip + _concat_off]
    mov QWORD PTR [rbp - 88], r10
    # save callee-saved registers used by the register allocator
    mov QWORD PTR [rbp - 80], rbx
    # param $this from rdi
    mov QWORD PTR [rbp - 72], rdi
_eir__class_propinit_66_entry_0:
    mov rax, QWORD PTR [rbp - 72]
    mov QWORD PTR [rbp - 24], rax
    lea rax, [rip + _str_0]
    mov rdx, 0
    mov QWORD PTR [rbp - 40], rax
    mov QWORD PTR [rbp - 32], rdx
    mov r11, QWORD PTR [rbp - 24]
    sub rsp, 16
    mov QWORD PTR [rsp], r11
    mov rax, QWORD PTR [rbp - 40]
    mov rdx, QWORD PTR [rbp - 32]
    call __rt_str_persist
    mov r11, QWORD PTR [rsp]
    add rsp, 16
    sub rsp, 16
    mov QWORD PTR [rsp], rax
    mov QWORD PTR [rsp + 8], rdx
    sub rsp, 16
    mov QWORD PTR [rsp], r11
    mov rax, QWORD PTR [r11 + 8]
    call __rt_heap_free_safe
    mov r11, QWORD PTR [rsp]
    add rsp, 16
    mov rax, QWORD PTR [rsp]
    mov rdx, QWORD PTR [rsp + 8]
    add rsp, 16
    mov QWORD PTR [r11 + 8], rax
    mov QWORD PTR [r11 + 16], rdx
    mov rax, QWORD PTR [rbp - 72]
    mov QWORD PTR [rbp - 56], rax
    mov rax, 0
    mov rbx, rax
    mov r11, QWORD PTR [rbp - 56]
    sub rsp, 16
    mov QWORD PTR [rsp], r11
    mov rax, rbx
    mov r11, QWORD PTR [rsp]
    add rsp, 16
    mov QWORD PTR [r11 + 24], rax
    mov QWORD PTR [r11 + 32], 0
    jmp _class_propinit_66_epilogue
_class_propinit_66_epilogue:
    # restore callee-saved registers used by the register allocator
    mov rbx, QWORD PTR [rbp - 80]
    add rsp, 96
    pop rbp
    ret

.globl _class_propinit_68
_class_propinit_68:
    # prologue
    push rbp
    mov rbp, rsp
    sub rsp, 96
    mov r10, QWORD PTR [rip + _concat_off]
    mov QWORD PTR [rbp - 88], r10
    # save callee-saved registers used by the register allocator
    mov QWORD PTR [rbp - 80], rbx
    # param $this from rdi
    mov QWORD PTR [rbp - 72], rdi
_eir__class_propinit_68_entry_0:
    mov rax, QWORD PTR [rbp - 72]
    mov QWORD PTR [rbp - 24], rax
    lea rax, [rip + _str_0]
    mov rdx, 0
    mov QWORD PTR [rbp - 40], rax
    mov QWORD PTR [rbp - 32], rdx
    mov r11, QWORD PTR [rbp - 24]
    sub rsp, 16
    mov QWORD PTR [rsp], r11
    mov rax, QWORD PTR [rbp - 40]
    mov rdx, QWORD PTR [rbp - 32]
    call __rt_str_persist
    mov r11, QWORD PTR [rsp]
    add rsp, 16
    sub rsp, 16
    mov QWORD PTR [rsp], rax
    mov QWORD PTR [rsp + 8], rdx
    sub rsp, 16
    mov QWORD PTR [rsp], r11
    mov rax, QWORD PTR [r11 + 8]
    call __rt_heap_free_safe
    mov r11, QWORD PTR [rsp]
    add rsp, 16
    mov rax, QWORD PTR [rsp]
    mov rdx, QWORD PTR [rsp + 8]
    add rsp, 16
    mov QWORD PTR [r11 + 8], rax
    mov QWORD PTR [r11 + 16], rdx
    mov rax, QWORD PTR [rbp - 72]
    mov QWORD PTR [rbp - 56], rax
    mov rax, 0
    mov rbx, rax
    mov r11, QWORD PTR [rbp - 56]
    sub rsp, 16
    mov QWORD PTR [rsp], r11
    mov rax, rbx
    mov r11, QWORD PTR [rsp]
    add rsp, 16
    mov QWORD PTR [r11 + 24], rax
    mov QWORD PTR [r11 + 32], 0
    jmp _class_propinit_68_epilogue
_class_propinit_68_epilogue:
    # restore callee-saved registers used by the register allocator
    mov rbx, QWORD PTR [rbp - 80]
    add rsp, 96
    pop rbp
    ret

.globl _class_propinit_69
_class_propinit_69:
    # prologue
    push rbp
    mov rbp, rsp
    sub rsp, 96
    mov r10, QWORD PTR [rip + _concat_off]
    mov QWORD PTR [rbp - 88], r10
    # save callee-saved registers used by the register allocator
    mov QWORD PTR [rbp - 80], rbx
    # param $this from rdi
    mov QWORD PTR [rbp - 72], rdi
_eir__class_propinit_69_entry_0:
    mov rax, QWORD PTR [rbp - 72]
    mov QWORD PTR [rbp - 24], rax
    lea rax, [rip + _str_0]
    mov rdx, 0
    mov QWORD PTR [rbp - 40], rax
    mov QWORD PTR [rbp - 32], rdx
    mov r11, QWORD PTR [rbp - 24]
    sub rsp, 16
    mov QWORD PTR [rsp], r11
    mov rax, QWORD PTR [rbp - 40]
    mov rdx, QWORD PTR [rbp - 32]
    call __rt_str_persist
    mov r11, QWORD PTR [rsp]
    add rsp, 16
    sub rsp, 16
    mov QWORD PTR [rsp], rax
    mov QWORD PTR [rsp + 8], rdx
    sub rsp, 16
    mov QWORD PTR [rsp], r11
    mov rax, QWORD PTR [r11 + 8]
    call __rt_heap_free_safe
    mov r11, QWORD PTR [rsp]
    add rsp, 16
    mov rax, QWORD PTR [rsp]
    mov rdx, QWORD PTR [rsp + 8]
    add rsp, 16
    mov QWORD PTR [r11 + 8], rax
    mov QWORD PTR [r11 + 16], rdx
    mov rax, QWORD PTR [rbp - 72]
    mov QWORD PTR [rbp - 56], rax
    mov rax, 0
    mov rbx, rax
    mov r11, QWORD PTR [rbp - 56]
    sub rsp, 16
    mov QWORD PTR [rsp], r11
    mov rax, rbx
    mov r11, QWORD PTR [rsp]
    add rsp, 16
    mov QWORD PTR [r11 + 24], rax
    mov QWORD PTR [r11 + 32], 0
    jmp _class_propinit_69_epilogue
_class_propinit_69_epilogue:
    # restore callee-saved registers used by the register allocator
    mov rbx, QWORD PTR [rbp - 80]
    add rsp, 96
    pop rbp
    ret

.globl _class_propinit_73
_class_propinit_73:
    # prologue
    push rbp
    mov rbp, rsp
    sub rsp, 96
    mov r10, QWORD PTR [rip + _concat_off]
    mov QWORD PTR [rbp - 88], r10
    # save callee-saved registers used by the register allocator
    mov QWORD PTR [rbp - 80], rbx
    # param $this from rdi
    mov QWORD PTR [rbp - 72], rdi
_eir__class_propinit_73_entry_0:
    mov rax, QWORD PTR [rbp - 72]
    mov QWORD PTR [rbp - 24], rax
    lea rax, [rip + _str_0]
    mov rdx, 0
    mov QWORD PTR [rbp - 40], rax
    mov QWORD PTR [rbp - 32], rdx
    mov r11, QWORD PTR [rbp - 24]
    sub rsp, 16
    mov QWORD PTR [rsp], r11
    mov rax, QWORD PTR [rbp - 40]
    mov rdx, QWORD PTR [rbp - 32]
    call __rt_str_persist
    mov r11, QWORD PTR [rsp]
    add rsp, 16
    sub rsp, 16
    mov QWORD PTR [rsp], rax
    mov QWORD PTR [rsp + 8], rdx
    sub rsp, 16
    mov QWORD PTR [rsp], r11
    mov rax, QWORD PTR [r11 + 8]
    call __rt_heap_free_safe
    mov r11, QWORD PTR [rsp]
    add rsp, 16
    mov rax, QWORD PTR [rsp]
    mov rdx, QWORD PTR [rsp + 8]
    add rsp, 16
    mov QWORD PTR [r11 + 8], rax
    mov QWORD PTR [r11 + 16], rdx
    mov rax, QWORD PTR [rbp - 72]
    mov QWORD PTR [rbp - 56], rax
    mov rax, 0
    mov rbx, rax
    mov r11, QWORD PTR [rbp - 56]
    sub rsp, 16
    mov QWORD PTR [rsp], r11
    mov rax, rbx
    mov r11, QWORD PTR [rsp]
    add rsp, 16
    mov QWORD PTR [r11 + 24], rax
    mov QWORD PTR [r11 + 32], 0
    jmp _class_propinit_73_epilogue
_class_propinit_73_epilogue:
    # restore callee-saved registers used by the register allocator
    mov rbx, QWORD PTR [rbp - 80]
    add rsp, 96
    pop rbp
    ret

.globl _class_propinit_74
_class_propinit_74:
    # prologue
    push rbp
    mov rbp, rsp
    sub rsp, 96
    mov r10, QWORD PTR [rip + _concat_off]
    mov QWORD PTR [rbp - 88], r10
    # save callee-saved registers used by the register allocator
    mov QWORD PTR [rbp - 80], rbx
    # param $this from rdi
    mov QWORD PTR [rbp - 72], rdi
_eir__class_propinit_74_entry_0:
    mov rax, QWORD PTR [rbp - 72]
    mov QWORD PTR [rbp - 24], rax
    lea rax, [rip + _str_0]
    mov rdx, 0
    mov QWORD PTR [rbp - 40], rax
    mov QWORD PTR [rbp - 32], rdx
    mov r11, QWORD PTR [rbp - 24]
    sub rsp, 16
    mov QWORD PTR [rsp], r11
    mov rax, QWORD PTR [rbp - 40]
    mov rdx, QWORD PTR [rbp - 32]
    call __rt_str_persist
    mov r11, QWORD PTR [rsp]
    add rsp, 16
    sub rsp, 16
    mov QWORD PTR [rsp], rax
    mov QWORD PTR [rsp + 8], rdx
    sub rsp, 16
    mov QWORD PTR [rsp], r11
    mov rax, QWORD PTR [r11 + 8]
    call __rt_heap_free_safe
    mov r11, QWORD PTR [rsp]
    add rsp, 16
    mov rax, QWORD PTR [rsp]
    mov rdx, QWORD PTR [rsp + 8]
    add rsp, 16
    mov QWORD PTR [r11 + 8], rax
    mov QWORD PTR [r11 + 16], rdx
    mov rax, QWORD PTR [rbp - 72]
    mov QWORD PTR [rbp - 56], rax
    mov rax, 0
    mov rbx, rax
    mov r11, QWORD PTR [rbp - 56]
    sub rsp, 16
    mov QWORD PTR [rsp], r11
    mov rax, rbx
    mov r11, QWORD PTR [rsp]
    add rsp, 16
    mov QWORD PTR [r11 + 24], rax
    mov QWORD PTR [r11 + 32], 0
    jmp _class_propinit_74_epilogue
_class_propinit_74_epilogue:
    # restore callee-saved registers used by the register allocator
    mov rbx, QWORD PTR [rbp - 80]
    add rsp, 96
    pop rbp
    ret

.globl _class_propinit_75
_class_propinit_75:
    # prologue
    push rbp
    mov rbp, rsp
    sub rsp, 96
    mov r10, QWORD PTR [rip + _concat_off]
    mov QWORD PTR [rbp - 88], r10
    # save callee-saved registers used by the register allocator
    mov QWORD PTR [rbp - 80], rbx
    # param $this from rdi
    mov QWORD PTR [rbp - 72], rdi
_eir__class_propinit_75_entry_0:
    mov rax, QWORD PTR [rbp - 72]
    mov QWORD PTR [rbp - 24], rax
    lea rax, [rip + _str_0]
    mov rdx, 0
    mov QWORD PTR [rbp - 40], rax
    mov QWORD PTR [rbp - 32], rdx
    mov r11, QWORD PTR [rbp - 24]
    sub rsp, 16
    mov QWORD PTR [rsp], r11
    mov rax, QWORD PTR [rbp - 40]
    mov rdx, QWORD PTR [rbp - 32]
    call __rt_str_persist
    mov r11, QWORD PTR [rsp]
    add rsp, 16
    sub rsp, 16
    mov QWORD PTR [rsp], rax
    mov QWORD PTR [rsp + 8], rdx
    sub rsp, 16
    mov QWORD PTR [rsp], r11
    mov rax, QWORD PTR [r11 + 8]
    call __rt_heap_free_safe
    mov r11, QWORD PTR [rsp]
    add rsp, 16
    mov rax, QWORD PTR [rsp]
    mov rdx, QWORD PTR [rsp + 8]
    add rsp, 16
    mov QWORD PTR [r11 + 8], rax
    mov QWORD PTR [r11 + 16], rdx
    mov rax, QWORD PTR [rbp - 72]
    mov QWORD PTR [rbp - 56], rax
    mov rax, 0
    mov rbx, rax
    mov r11, QWORD PTR [rbp - 56]
    sub rsp, 16
    mov QWORD PTR [rsp], r11
    mov rax, rbx
    mov r11, QWORD PTR [rsp]
    add rsp, 16
    mov QWORD PTR [r11 + 24], rax
    mov QWORD PTR [r11 + 32], 0
    jmp _class_propinit_75_epilogue
_class_propinit_75_epilogue:
    # restore callee-saved registers used by the register allocator
    mov rbx, QWORD PTR [rbp - 80]
    add rsp, 96
    pop rbp
    ret

.globl _class_propinit_77
_class_propinit_77:
    # prologue
    push rbp
    mov rbp, rsp
    sub rsp, 96
    mov r10, QWORD PTR [rip + _concat_off]
    mov QWORD PTR [rbp - 88], r10
    # save callee-saved registers used by the register allocator
    mov QWORD PTR [rbp - 80], rbx
    # param $this from rdi
    mov QWORD PTR [rbp - 72], rdi
_eir__class_propinit_77_entry_0:
    mov rax, QWORD PTR [rbp - 72]
    mov QWORD PTR [rbp - 24], rax
    lea rax, [rip + _str_0]
    mov rdx, 0
    mov QWORD PTR [rbp - 40], rax
    mov QWORD PTR [rbp - 32], rdx
    mov r11, QWORD PTR [rbp - 24]
    sub rsp, 16
    mov QWORD PTR [rsp], r11
    mov rax, QWORD PTR [rbp - 40]
    mov rdx, QWORD PTR [rbp - 32]
    call __rt_str_persist
    mov r11, QWORD PTR [rsp]
    add rsp, 16
    sub rsp, 16
    mov QWORD PTR [rsp], rax
    mov QWORD PTR [rsp + 8], rdx
    sub rsp, 16
    mov QWORD PTR [rsp], r11
    mov rax, QWORD PTR [r11 + 8]
    call __rt_heap_free_safe
    mov r11, QWORD PTR [rsp]
    add rsp, 16
    mov rax, QWORD PTR [rsp]
    mov rdx, QWORD PTR [rsp + 8]
    add rsp, 16
    mov QWORD PTR [r11 + 8], rax
    mov QWORD PTR [r11 + 16], rdx
    mov rax, QWORD PTR [rbp - 72]
    mov QWORD PTR [rbp - 56], rax
    mov rax, 0
    mov rbx, rax
    mov r11, QWORD PTR [rbp - 56]
    sub rsp, 16
    mov QWORD PTR [rsp], r11
    mov rax, rbx
    mov r11, QWORD PTR [rsp]
    add rsp, 16
    mov QWORD PTR [r11 + 24], rax
    mov QWORD PTR [r11 + 32], 0
    jmp _class_propinit_77_epilogue
_class_propinit_77_epilogue:
    # restore callee-saved registers used by the register allocator
    mov rbx, QWORD PTR [rbp - 80]
    add rsp, 96
    pop rbp
    ret

.globl _class_propinit_78
_class_propinit_78:
    # prologue
    push rbp
    mov rbp, rsp
    sub rsp, 96
    mov r10, QWORD PTR [rip + _concat_off]
    mov QWORD PTR [rbp - 88], r10
    # save callee-saved registers used by the register allocator
    mov QWORD PTR [rbp - 80], rbx
    # param $this from rdi
    mov QWORD PTR [rbp - 72], rdi
_eir__class_propinit_78_entry_0:
    mov rax, QWORD PTR [rbp - 72]
    mov QWORD PTR [rbp - 24], rax
    lea rax, [rip + _str_0]
    mov rdx, 0
    mov QWORD PTR [rbp - 40], rax
    mov QWORD PTR [rbp - 32], rdx
    mov r11, QWORD PTR [rbp - 24]
    sub rsp, 16
    mov QWORD PTR [rsp], r11
    mov rax, QWORD PTR [rbp - 40]
    mov rdx, QWORD PTR [rbp - 32]
    call __rt_str_persist
    mov r11, QWORD PTR [rsp]
    add rsp, 16
    sub rsp, 16
    mov QWORD PTR [rsp], rax
    mov QWORD PTR [rsp + 8], rdx
    sub rsp, 16
    mov QWORD PTR [rsp], r11
    mov rax, QWORD PTR [r11 + 8]
    call __rt_heap_free_safe
    mov r11, QWORD PTR [rsp]
    add rsp, 16
    mov rax, QWORD PTR [rsp]
    mov rdx, QWORD PTR [rsp + 8]
    add rsp, 16
    mov QWORD PTR [r11 + 8], rax
    mov QWORD PTR [r11 + 16], rdx
    mov rax, QWORD PTR [rbp - 72]
    mov QWORD PTR [rbp - 56], rax
    mov rax, 0
    mov rbx, rax
    mov r11, QWORD PTR [rbp - 56]
    sub rsp, 16
    mov QWORD PTR [rsp], r11
    mov rax, rbx
    mov r11, QWORD PTR [rsp]
    add rsp, 16
    mov QWORD PTR [r11 + 24], rax
    mov QWORD PTR [r11 + 32], 0
    jmp _class_propinit_78_epilogue
_class_propinit_78_epilogue:
    # restore callee-saved registers used by the register allocator
    mov rbx, QWORD PTR [rbp - 80]
    add rsp, 96
    pop rbp
    ret

.globl _class_propinit_80
_class_propinit_80:
    # prologue
    push rbp
    mov rbp, rsp
    sub rsp, 96
    mov r10, QWORD PTR [rip + _concat_off]
    mov QWORD PTR [rbp - 88], r10
    # save callee-saved registers used by the register allocator
    mov QWORD PTR [rbp - 80], rbx
    # param $this from rdi
    mov QWORD PTR [rbp - 72], rdi
_eir__class_propinit_80_entry_0:
    mov rax, QWORD PTR [rbp - 72]
    mov QWORD PTR [rbp - 24], rax
    lea rax, [rip + _str_0]
    mov rdx, 0
    mov QWORD PTR [rbp - 40], rax
    mov QWORD PTR [rbp - 32], rdx
    mov r11, QWORD PTR [rbp - 24]
    sub rsp, 16
    mov QWORD PTR [rsp], r11
    mov rax, QWORD PTR [rbp - 40]
    mov rdx, QWORD PTR [rbp - 32]
    call __rt_str_persist
    mov r11, QWORD PTR [rsp]
    add rsp, 16
    sub rsp, 16
    mov QWORD PTR [rsp], rax
    mov QWORD PTR [rsp + 8], rdx
    sub rsp, 16
    mov QWORD PTR [rsp], r11
    mov rax, QWORD PTR [r11 + 8]
    call __rt_heap_free_safe
    mov r11, QWORD PTR [rsp]
    add rsp, 16
    mov rax, QWORD PTR [rsp]
    mov rdx, QWORD PTR [rsp + 8]
    add rsp, 16
    mov QWORD PTR [r11 + 8], rax
    mov QWORD PTR [r11 + 16], rdx
    mov rax, QWORD PTR [rbp - 72]
    mov QWORD PTR [rbp - 56], rax
    mov rax, 0
    mov rbx, rax
    mov r11, QWORD PTR [rbp - 56]
    sub rsp, 16
    mov QWORD PTR [rsp], r11
    mov rax, rbx
    mov r11, QWORD PTR [rsp]
    add rsp, 16
    mov QWORD PTR [r11 + 24], rax
    mov QWORD PTR [r11 + 32], 0
    jmp _class_propinit_80_epilogue
_class_propinit_80_epilogue:
    # restore callee-saved registers used by the register allocator
    mov rbx, QWORD PTR [rbp - 80]
    add rsp, 96
    pop rbp
    ret

.globl _class_propinit_82
_class_propinit_82:
    # prologue
    push rbp
    mov rbp, rsp
    sub rsp, 96
    mov r10, QWORD PTR [rip + _concat_off]
    mov QWORD PTR [rbp - 88], r10
    # save callee-saved registers used by the register allocator
    mov QWORD PTR [rbp - 80], rbx
    # param $this from rdi
    mov QWORD PTR [rbp - 72], rdi
_eir__class_propinit_82_entry_0:
    mov rax, QWORD PTR [rbp - 72]
    mov QWORD PTR [rbp - 24], rax
    lea rax, [rip + _str_0]
    mov rdx, 0
    mov QWORD PTR [rbp - 40], rax
    mov QWORD PTR [rbp - 32], rdx
    mov r11, QWORD PTR [rbp - 24]
    sub rsp, 16
    mov QWORD PTR [rsp], r11
    mov rax, QWORD PTR [rbp - 40]
    mov rdx, QWORD PTR [rbp - 32]
    call __rt_str_persist
    mov r11, QWORD PTR [rsp]
    add rsp, 16
    sub rsp, 16
    mov QWORD PTR [rsp], rax
    mov QWORD PTR [rsp + 8], rdx
    sub rsp, 16
    mov QWORD PTR [rsp], r11
    mov rax, QWORD PTR [r11 + 8]
    call __rt_heap_free_safe
    mov r11, QWORD PTR [rsp]
    add rsp, 16
    mov rax, QWORD PTR [rsp]
    mov rdx, QWORD PTR [rsp + 8]
    add rsp, 16
    mov QWORD PTR [r11 + 8], rax
    mov QWORD PTR [r11 + 16], rdx
    mov rax, QWORD PTR [rbp - 72]
    mov QWORD PTR [rbp - 56], rax
    mov rax, 0
    mov rbx, rax
    mov r11, QWORD PTR [rbp - 56]
    sub rsp, 16
    mov QWORD PTR [rsp], r11
    mov rax, rbx
    mov r11, QWORD PTR [rsp]
    add rsp, 16
    mov QWORD PTR [r11 + 24], rax
    mov QWORD PTR [r11 + 32], 0
    jmp _class_propinit_82_epilogue
_class_propinit_82_epilogue:
    # restore callee-saved registers used by the register allocator
    mov rbx, QWORD PTR [rbp - 80]
    add rsp, 96
    pop rbp
    ret

.globl _class_propinit_83
_class_propinit_83:
    # prologue
    push rbp
    mov rbp, rsp
    sub rsp, 96
    mov r10, QWORD PTR [rip + _concat_off]
    mov QWORD PTR [rbp - 88], r10
    # save callee-saved registers used by the register allocator
    mov QWORD PTR [rbp - 80], rbx
    # param $this from rdi
    mov QWORD PTR [rbp - 72], rdi
_eir__class_propinit_83_entry_0:
    mov rax, QWORD PTR [rbp - 72]
    mov QWORD PTR [rbp - 24], rax
    lea rax, [rip + _str_0]
    mov rdx, 0
    mov QWORD PTR [rbp - 40], rax
    mov QWORD PTR [rbp - 32], rdx
    mov r11, QWORD PTR [rbp - 24]
    sub rsp, 16
    mov QWORD PTR [rsp], r11
    mov rax, QWORD PTR [rbp - 40]
    mov rdx, QWORD PTR [rbp - 32]
    call __rt_str_persist
    mov r11, QWORD PTR [rsp]
    add rsp, 16
    sub rsp, 16
    mov QWORD PTR [rsp], rax
    mov QWORD PTR [rsp + 8], rdx
    sub rsp, 16
    mov QWORD PTR [rsp], r11
    mov rax, QWORD PTR [r11 + 8]
    call __rt_heap_free_safe
    mov r11, QWORD PTR [rsp]
    add rsp, 16
    mov rax, QWORD PTR [rsp]
    mov rdx, QWORD PTR [rsp + 8]
    add rsp, 16
    mov QWORD PTR [r11 + 8], rax
    mov QWORD PTR [r11 + 16], rdx
    mov rax, QWORD PTR [rbp - 72]
    mov QWORD PTR [rbp - 56], rax
    mov rax, 0
    mov rbx, rax
    mov r11, QWORD PTR [rbp - 56]
    sub rsp, 16
    mov QWORD PTR [rsp], r11
    mov rax, rbx
    mov r11, QWORD PTR [rsp]
    add rsp, 16
    mov QWORD PTR [r11 + 24], rax
    mov QWORD PTR [r11 + 32], 0
    jmp _class_propinit_83_epilogue
_class_propinit_83_epilogue:
    # restore callee-saved registers used by the register allocator
    mov rbx, QWORD PTR [rbp - 80]
    add rsp, 96
    pop rbp
    ret

.globl _class_propinit_85
_class_propinit_85:
    # prologue
    push rbp
    mov rbp, rsp
    sub rsp, 96
    mov r10, QWORD PTR [rip + _concat_off]
    mov QWORD PTR [rbp - 88], r10
    # save callee-saved registers used by the register allocator
    mov QWORD PTR [rbp - 80], rbx
    # param $this from rdi
    mov QWORD PTR [rbp - 72], rdi
_eir__class_propinit_85_entry_0:
    mov rax, QWORD PTR [rbp - 72]
    mov QWORD PTR [rbp - 24], rax
    lea rax, [rip + _str_0]
    mov rdx, 0
    mov QWORD PTR [rbp - 40], rax
    mov QWORD PTR [rbp - 32], rdx
    mov r11, QWORD PTR [rbp - 24]
    sub rsp, 16
    mov QWORD PTR [rsp], r11
    mov rax, QWORD PTR [rbp - 40]
    mov rdx, QWORD PTR [rbp - 32]
    call __rt_str_persist
    mov r11, QWORD PTR [rsp]
    add rsp, 16
    sub rsp, 16
    mov QWORD PTR [rsp], rax
    mov QWORD PTR [rsp + 8], rdx
    sub rsp, 16
    mov QWORD PTR [rsp], r11
    mov rax, QWORD PTR [r11 + 8]
    call __rt_heap_free_safe
    mov r11, QWORD PTR [rsp]
    add rsp, 16
    mov rax, QWORD PTR [rsp]
    mov rdx, QWORD PTR [rsp + 8]
    add rsp, 16
    mov QWORD PTR [r11 + 8], rax
    mov QWORD PTR [r11 + 16], rdx
    mov rax, QWORD PTR [rbp - 72]
    mov QWORD PTR [rbp - 56], rax
    mov rax, 0
    mov rbx, rax
    mov r11, QWORD PTR [rbp - 56]
    sub rsp, 16
    mov QWORD PTR [rsp], r11
    mov rax, rbx
    mov r11, QWORD PTR [rsp]
    add rsp, 16
    mov QWORD PTR [r11 + 24], rax
    mov QWORD PTR [r11 + 32], 0
    jmp _class_propinit_85_epilogue
_class_propinit_85_epilogue:
    # restore callee-saved registers used by the register allocator
    mov rbx, QWORD PTR [rbp - 80]
    add rsp, 96
    pop rbp
    ret

.globl _method_AIC_N_Components_N_Domain_N_HeadAsset__u__u_construct
_method_AIC_N_Components_N_Domain_N_HeadAsset__u__u_construct:
    # prologue
    push rbp
    mov rbp, rsp
    sub rsp, 160
    mov r10, QWORD PTR [rip + _concat_off]
    mov QWORD PTR [rbp - 160], r10
    # param $this from rdi
    mov QWORD PTR [rbp - 104], rdi
    # param $type from rsi
    mov QWORD PTR [rbp - 112], rsi
    # param $url from rdx,rcx
    mov QWORD PTR [rbp - 128], rdx
    mov QWORD PTR [rbp - 120], rcx
    # param $integrity from r8
    mov QWORD PTR [rbp - 136], r8
    # param $crossorigin from r9
    mov QWORD PTR [rbp - 144], r9
    # param $mode from caller stack +16
    mov r10, QWORD PTR [rbp + 16]
    mov QWORD PTR [rbp - 152], r10
_eir_AIC_Components_Domain_HeadAsset____construct_entry_0:
    # @src line=45 col=30
    mov r10, QWORD PTR [rbp - 160]
    mov QWORD PTR [rip + _concat_off], r10
    # @src line=45 col=30
    mov rax, QWORD PTR [rbp - 104]
    mov QWORD PTR [rbp - 8], rax
    # @src line=45 col=30
    mov rax, QWORD PTR [rbp - 112]
    mov QWORD PTR [rbp - 16], rax
    # @src line=45 col=30
    mov r11, QWORD PTR [rbp - 8]
    sub rsp, 16
    mov QWORD PTR [rsp], r11
    mov rax, QWORD PTR [rbp - 16]
    sub rsp, 16
    mov QWORD PTR [rsp], rax
    call __rt_incref
    mov rax, QWORD PTR [rsp]
    add rsp, 16
    mov r11, QWORD PTR [rsp]
    add rsp, 16
    sub rsp, 16
    mov QWORD PTR [rsp], rax
    sub rsp, 16
    mov QWORD PTR [rsp], r11
    mov rax, QWORD PTR [r11 + 8]
    call __rt_decref_object
    mov r11, QWORD PTR [rsp]
    add rsp, 16
    mov rax, QWORD PTR [rsp]
    add rsp, 16
    mov QWORD PTR [r11 + 8], rax
    mov QWORD PTR [r11 + 16], 0
    # @src line=46 col=23
    mov r10, QWORD PTR [rbp - 160]
    mov QWORD PTR [rip + _concat_off], r10
    # @src line=46 col=23
    mov rax, QWORD PTR [rbp - 104]
    mov QWORD PTR [rbp - 24], rax
    # @src line=46 col=23
    mov rax, QWORD PTR [rbp - 128]
    mov rdx, QWORD PTR [rbp - 120]
    mov QWORD PTR [rbp - 40], rax
    mov QWORD PTR [rbp - 32], rdx
    # @src line=46 col=23
    mov r11, QWORD PTR [rbp - 24]
    sub rsp, 16
    mov QWORD PTR [rsp], r11
    mov rax, QWORD PTR [rbp - 40]
    mov rdx, QWORD PTR [rbp - 32]
    call __rt_str_persist
    mov r11, QWORD PTR [rsp]
    add rsp, 16
    sub rsp, 16
    mov QWORD PTR [rsp], rax
    mov QWORD PTR [rsp + 8], rdx
    sub rsp, 16
    mov QWORD PTR [rsp], r11
    mov rax, QWORD PTR [r11 + 24]
    call __rt_heap_free_safe
    mov r11, QWORD PTR [rsp]
    add rsp, 16
    mov rax, QWORD PTR [rsp]
    mov rdx, QWORD PTR [rsp + 8]
    add rsp, 16
    mov QWORD PTR [r11 + 24], rax
    mov QWORD PTR [r11 + 32], rdx
    # @src line=47 col=24
    mov r10, QWORD PTR [rbp - 160]
    mov QWORD PTR [rip + _concat_off], r10
    # @src line=47 col=24
    mov rax, QWORD PTR [rbp - 104]
    mov QWORD PTR [rbp - 48], rax
    # @src line=47 col=24
    mov rax, QWORD PTR [rbp - 136]
    mov QWORD PTR [rbp - 56], rax
    # @src line=47 col=24
    mov r11, QWORD PTR [rbp - 48]
    sub rsp, 16
    mov QWORD PTR [rsp], r11
    mov rax, QWORD PTR [rbp - 56]
    sub rsp, 16
    mov QWORD PTR [rsp], rax
    call __rt_incref
    mov rax, QWORD PTR [rsp]
    add rsp, 16
    mov r11, QWORD PTR [rsp]
    add rsp, 16
    sub rsp, 16
    mov QWORD PTR [rsp], rax
    sub rsp, 16
    mov QWORD PTR [rsp], r11
    mov rax, QWORD PTR [r11 + 40]
    call __rt_decref_mixed
    mov r11, QWORD PTR [rsp]
    add rsp, 16
    mov rax, QWORD PTR [rsp]
    add rsp, 16
    mov QWORD PTR [r11 + 40], rax
    mov QWORD PTR [r11 + 48], 0
    # @src line=48 col=24
    mov r10, QWORD PTR [rbp - 160]
    mov QWORD PTR [rip + _concat_off], r10
    # @src line=48 col=24
    mov rax, QWORD PTR [rbp - 104]
    mov QWORD PTR [rbp - 64], rax
    # @src line=48 col=24
    mov rax, QWORD PTR [rbp - 144]
    mov QWORD PTR [rbp - 72], rax
    # @src line=48 col=24
    mov r11, QWORD PTR [rbp - 64]
    sub rsp, 16
    mov QWORD PTR [rsp], r11
    mov rax, QWORD PTR [rbp - 72]
    sub rsp, 16
    mov QWORD PTR [rsp], rax
    call __rt_incref
    mov rax, QWORD PTR [rsp]
    add rsp, 16
    mov r11, QWORD PTR [rsp]
    add rsp, 16
    sub rsp, 16
    mov QWORD PTR [rsp], rax
    sub rsp, 16
    mov QWORD PTR [rsp], r11
    mov rax, QWORD PTR [r11 + 56]
    call __rt_decref_mixed
    mov r11, QWORD PTR [rsp]
    add rsp, 16
    mov rax, QWORD PTR [rsp]
    add rsp, 16
    mov QWORD PTR [r11 + 56], rax
    mov QWORD PTR [r11 + 64], 0
    # @src line=49 col=30
    mov r10, QWORD PTR [rbp - 160]
    mov QWORD PTR [rip + _concat_off], r10
    # @src line=49 col=30
    mov rax, QWORD PTR [rbp - 104]
    mov QWORD PTR [rbp - 80], rax
    # @src line=49 col=30
    mov rax, QWORD PTR [rbp - 152]
    mov QWORD PTR [rbp - 88], rax
    # @src line=49 col=30
    mov r11, QWORD PTR [rbp - 80]
    sub rsp, 16
    mov QWORD PTR [rsp], r11
    mov rax, QWORD PTR [rbp - 88]
    sub rsp, 16
    mov QWORD PTR [rsp], rax
    call __rt_incref
    mov rax, QWORD PTR [rsp]
    add rsp, 16
    mov r11, QWORD PTR [rsp]
    add rsp, 16
    sub rsp, 16
    mov QWORD PTR [rsp], rax
    sub rsp, 16
    mov QWORD PTR [rsp], r11
    mov rax, QWORD PTR [r11 + 72]
    call __rt_decref_object
    mov r11, QWORD PTR [rsp]
    add rsp, 16
    mov rax, QWORD PTR [rsp]
    add rsp, 16
    mov QWORD PTR [r11 + 72], rax
    mov QWORD PTR [r11 + 80], 0
    jmp _method_AIC_N_Components_N_Domain_N_HeadAsset__u__u_construct_epilogue
_method_AIC_N_Components_N_Domain_N_HeadAsset__u__u_construct_epilogue:
    add rsp, 160
    pop rbp
    ret

.globl _method_AIC_N_Components_N_Domain_N_HeadAsset_dedupkey
_method_AIC_N_Components_N_Domain_N_HeadAsset_dedupkey:
    # prologue
    push rbp
    mov rbp, rsp
    sub rsp, 224
    mov r10, QWORD PTR [rip + _concat_off]
    mov QWORD PTR [rbp - 224], r10
    # param $this from rdi
    mov QWORD PTR [rbp - 216], rdi
_eir_AIC_Components_Domain_HeadAsset__dedupKey_entry_0:
    # @src line=64 col=9
    mov r10, QWORD PTR [rbp - 224]
    mov QWORD PTR [rip + _concat_off], r10
    # @src line=64 col=16
    mov rax, QWORD PTR [rbp - 216]
    mov QWORD PTR [rbp - 8], rax
    # @src line=64 col=21
    mov r11, QWORD PTR [rbp - 8]
    mov r10, QWORD PTR [r11 + 16]
    mov rcx, 9223372036854775805
    cmp r10, rcx
    jne _eir_AIC_Components_Domain_HeadAsset__dedupKey_typed_prop_initialized_0
    lea rsi, [rip + _str_3]
    mov edx, 110
    mov edi, 2
    mov eax, 1
    syscall
    mov edi, 1
    mov eax, 60
    syscall
_eir_AIC_Components_Domain_HeadAsset__dedupKey_typed_prop_initialized_0:
    mov rax, QWORD PTR [r11 + 8]
    mov QWORD PTR [rbp - 16], rax
    # @src line=64 col=27
    mov r11, QWORD PTR [rbp - 16]
    mov r10, QWORD PTR [r11 + 16]
    mov rcx, 9223372036854775805
    cmp r10, rcx
    jne _eir_AIC_Components_Domain_HeadAsset__dedupKey_typed_prop_initialized_1
    lea rsi, [rip + _str_4]
    mov edx, 115
    mov edi, 2
    mov eax, 1
    syscall
    mov edi, 1
    mov eax, 60
    syscall
_eir_AIC_Components_Domain_HeadAsset__dedupKey_typed_prop_initialized_1:
    mov rax, QWORD PTR [r11 + 8]
    mov rdx, QWORD PTR [r11 + 16]
    mov QWORD PTR [rbp - 32], rax
    mov QWORD PTR [rbp - 24], rdx
    # @src line=64 col=37
    lea rax, [rip + _str_5]
    mov rdx, 1
    mov QWORD PTR [rbp - 48], rax
    mov QWORD PTR [rbp - 40], rdx
    # @src line=64 col=35
    mov rax, QWORD PTR [rbp - 32]
    mov rdx, QWORD PTR [rbp - 24]
    mov rdi, QWORD PTR [rbp - 48]
    mov rsi, QWORD PTR [rbp - 40]
    call __rt_concat
    mov QWORD PTR [rbp - 64], rax
    mov QWORD PTR [rbp - 56], rdx
    # @src line=64 col=43
    mov rax, QWORD PTR [rbp - 216]
    mov QWORD PTR [rbp - 72], rax
    # @src line=64 col=48
    mov r11, QWORD PTR [rbp - 72]
    mov r10, QWORD PTR [r11 + 80]
    mov rcx, 9223372036854775805
    cmp r10, rcx
    jne _eir_AIC_Components_Domain_HeadAsset__dedupKey_typed_prop_initialized_2
    lea rsi, [rip + _str_6]
    mov edx, 110
    mov edi, 2
    mov eax, 1
    syscall
    mov edi, 1
    mov eax, 60
    syscall
_eir_AIC_Components_Domain_HeadAsset__dedupKey_typed_prop_initialized_2:
    mov rax, QWORD PTR [r11 + 72]
    mov QWORD PTR [rbp - 80], rax
    # @src line=64 col=54
    mov r11, QWORD PTR [rbp - 80]
    mov r10, QWORD PTR [r11 + 16]
    mov rcx, 9223372036854775805
    cmp r10, rcx
    jne _eir_AIC_Components_Domain_HeadAsset__dedupKey_typed_prop_initialized_3
    lea rsi, [rip + _str_7]
    mov edx, 115
    mov edi, 2
    mov eax, 1
    syscall
    mov edi, 1
    mov eax, 60
    syscall
_eir_AIC_Components_Domain_HeadAsset__dedupKey_typed_prop_initialized_3:
    mov rax, QWORD PTR [r11 + 8]
    mov rdx, QWORD PTR [r11 + 16]
    mov QWORD PTR [rbp - 96], rax
    mov QWORD PTR [rbp - 88], rdx
    # @src line=64 col=41
    mov rax, QWORD PTR [rbp - 64]
    mov rdx, QWORD PTR [rbp - 56]
    mov rdi, QWORD PTR [rbp - 96]
    mov rsi, QWORD PTR [rbp - 88]
    call __rt_concat
    mov QWORD PTR [rbp - 112], rax
    mov QWORD PTR [rbp - 104], rdx
    # @src line=64 col=41
    # @src line=64 col=64
    lea rax, [rip + _str_5]
    mov rdx, 1
    mov QWORD PTR [rbp - 128], rax
    mov QWORD PTR [rbp - 120], rdx
    # @src line=64 col=62
    mov rax, QWORD PTR [rbp - 112]
    mov rdx, QWORD PTR [rbp - 104]
    mov rdi, QWORD PTR [rbp - 128]
    mov rsi, QWORD PTR [rbp - 120]
    call __rt_concat
    mov QWORD PTR [rbp - 144], rax
    mov QWORD PTR [rbp - 136], rdx
    # @src line=64 col=62
    # @src line=64 col=70
    mov rax, QWORD PTR [rbp - 216]
    mov QWORD PTR [rbp - 152], rax
    # @src line=64 col=75
    mov r11, QWORD PTR [rbp - 152]
    mov r10, QWORD PTR [r11 + 32]
    mov rcx, 9223372036854775805
    cmp r10, rcx
    jne _eir_AIC_Components_Domain_HeadAsset__dedupKey_typed_prop_initialized_4
    lea rsi, [rip + _str_8]
    mov edx, 109
    mov edi, 2
    mov eax, 1
    syscall
    mov edi, 1
    mov eax, 60
    syscall
_eir_AIC_Components_Domain_HeadAsset__dedupKey_typed_prop_initialized_4:
    mov rax, QWORD PTR [r11 + 24]
    mov rdx, QWORD PTR [r11 + 32]
    mov QWORD PTR [rbp - 168], rax
    mov QWORD PTR [rbp - 160], rdx
    # @src line=64 col=68
    mov rax, QWORD PTR [rbp - 144]
    mov rdx, QWORD PTR [rbp - 136]
    mov rdi, QWORD PTR [rbp - 168]
    mov rsi, QWORD PTR [rbp - 160]
    call __rt_concat
    mov QWORD PTR [rbp - 184], rax
    mov QWORD PTR [rbp - 176], rdx
    # @src line=64 col=68
    # @src line=64 col=9
    mov rax, QWORD PTR [rbp - 184]
    mov rdx, QWORD PTR [rbp - 176]
    call __rt_str_persist
    mov QWORD PTR [rbp - 200], rax
    mov QWORD PTR [rbp - 192], rdx
    mov rax, QWORD PTR [rbp - 200]
    mov rdx, QWORD PTR [rbp - 192]
    jmp _method_AIC_N_Components_N_Domain_N_HeadAsset_dedupkey_epilogue
_method_AIC_N_Components_N_Domain_N_HeadAsset_dedupkey_epilogue:
    add rsp, 224
    pop rbp
    ret

.globl _method_ReflectionAttribute__u__u_construct
_method_ReflectionAttribute__u__u_construct:
    # prologue
    push rbp
    mov rbp, rsp
    sub rsp, 16
    mov r10, QWORD PTR [rip + _concat_off]
    mov QWORD PTR [rbp - 16], r10
    # param $this from rdi
    mov QWORD PTR [rbp - 8], rdi
_eir_ReflectionAttribute____construct_entry_0:
    jmp _method_ReflectionAttribute__u__u_construct_epilogue
_method_ReflectionAttribute__u__u_construct_epilogue:
    add rsp, 16
    pop rbp
    ret

.globl _method_ReflectionAttribute_getname
_method_ReflectionAttribute_getname:
    # prologue
    push rbp
    mov rbp, rsp
    sub rsp, 64
    mov r10, QWORD PTR [rip + _concat_off]
    mov QWORD PTR [rbp - 64], r10
    # param $this from rdi
    mov QWORD PTR [rbp - 56], rdi
_eir_ReflectionAttribute__getName_entry_0:
    mov rax, QWORD PTR [rbp - 56]
    mov QWORD PTR [rbp - 8], rax
    mov r11, QWORD PTR [rbp - 8]
    mov r10, QWORD PTR [r11 + 16]
    mov rcx, 9223372036854775805
    cmp r10, rcx
    jne _eir_ReflectionAttribute__getName_typed_prop_initialized_0
    lea rsi, [rip + _str_9]
    mov edx, 100
    mov edi, 2
    mov eax, 1
    syscall
    mov edi, 1
    mov eax, 60
    syscall
_eir_ReflectionAttribute__getName_typed_prop_initialized_0:
    mov rax, QWORD PTR [r11 + 8]
    mov rdx, QWORD PTR [r11 + 16]
    mov QWORD PTR [rbp - 24], rax
    mov QWORD PTR [rbp - 16], rdx
    mov rax, QWORD PTR [rbp - 24]
    mov rdx, QWORD PTR [rbp - 16]
    call __rt_str_persist
    mov QWORD PTR [rbp - 40], rax
    mov QWORD PTR [rbp - 32], rdx
    mov rax, QWORD PTR [rbp - 40]
    mov rdx, QWORD PTR [rbp - 32]
    jmp _method_ReflectionAttribute_getname_epilogue
_method_ReflectionAttribute_getname_epilogue:
    add rsp, 64
    pop rbp
    ret

.globl _method_ReflectionAttribute_getarguments
_method_ReflectionAttribute_getarguments:
    # prologue
    push rbp
    mov rbp, rsp
    sub rsp, 32
    mov r10, QWORD PTR [rip + _concat_off]
    mov QWORD PTR [rbp - 32], r10
    # param $this from rdi
    mov QWORD PTR [rbp - 24], rdi
_eir_ReflectionAttribute__getArguments_entry_0:
    mov rdi, 16
    mov rsi, 7
    call __rt_hash_new
    mov QWORD PTR [rbp - 8], rax
    mov rax, QWORD PTR [rbp - 8]
    jmp _method_ReflectionAttribute_getarguments_epilogue
_method_ReflectionAttribute_getarguments_epilogue:
    add rsp, 32
    pop rbp
    ret

.globl _method_ReflectionAttribute_newinstance
_method_ReflectionAttribute_newinstance:
    # prologue
    push rbp
    mov rbp, rsp
    sub rsp, 48
    mov r10, QWORD PTR [rip + _concat_off]
    mov QWORD PTR [rbp - 40], r10
    # save callee-saved registers used by the register allocator
    mov QWORD PTR [rbp - 32], rbx
    # param $this from rdi
    mov QWORD PTR [rbp - 24], rdi
_eir_ReflectionAttribute__newInstance_entry_0:
    mov rax, 9223372036854775806
    mov rbx, rax
    mov rax, rbx
    mov rdi, rax
    xor rsi, rsi
    mov rax, 8
    call __rt_mixed_from_value
    mov QWORD PTR [rbp - 16], rax
    mov rax, QWORD PTR [rbp - 16]
    jmp _method_ReflectionAttribute_newinstance_epilogue
_method_ReflectionAttribute_newinstance_epilogue:
    # restore callee-saved registers used by the register allocator
    mov rbx, QWORD PTR [rbp - 32]
    add rsp, 48
    pop rbp
    ret

.globl _method_ReflectionClass__u__u_construct
_method_ReflectionClass__u__u_construct:
    # prologue
    push rbp
    mov rbp, rsp
    sub rsp, 32
    mov r10, QWORD PTR [rip + _concat_off]
    mov QWORD PTR [rbp - 32], r10
    # param $this from rdi
    mov QWORD PTR [rbp - 8], rdi
    # param $class_name from rsi,rdx
    mov QWORD PTR [rbp - 24], rsi
    mov QWORD PTR [rbp - 16], rdx
_eir_ReflectionClass____construct_entry_0:
    jmp _method_ReflectionClass__u__u_construct_epilogue
_method_ReflectionClass__u__u_construct_epilogue:
    add rsp, 32
    pop rbp
    ret

.globl _method_ReflectionClass_getname
_method_ReflectionClass_getname:
    # prologue
    push rbp
    mov rbp, rsp
    sub rsp, 64
    mov r10, QWORD PTR [rip + _concat_off]
    mov QWORD PTR [rbp - 64], r10
    # param $this from rdi
    mov QWORD PTR [rbp - 56], rdi
_eir_ReflectionClass__getName_entry_0:
    mov rax, QWORD PTR [rbp - 56]
    mov QWORD PTR [rbp - 8], rax
    mov r11, QWORD PTR [rbp - 8]
    mov r10, QWORD PTR [r11 + 16]
    mov rcx, 9223372036854775805
    cmp r10, rcx
    jne _eir_ReflectionClass__getName_typed_prop_initialized_0
    lea rsi, [rip + _str_10]
    mov edx, 96
    mov edi, 2
    mov eax, 1
    syscall
    mov edi, 1
    mov eax, 60
    syscall
_eir_ReflectionClass__getName_typed_prop_initialized_0:
    mov rax, QWORD PTR [r11 + 8]
    mov rdx, QWORD PTR [r11 + 16]
    mov QWORD PTR [rbp - 24], rax
    mov QWORD PTR [rbp - 16], rdx
    mov rax, QWORD PTR [rbp - 24]
    mov rdx, QWORD PTR [rbp - 16]
    call __rt_str_persist
    mov QWORD PTR [rbp - 40], rax
    mov QWORD PTR [rbp - 32], rdx
    mov rax, QWORD PTR [rbp - 40]
    mov rdx, QWORD PTR [rbp - 32]
    jmp _method_ReflectionClass_getname_epilogue
_method_ReflectionClass_getname_epilogue:
    add rsp, 64
    pop rbp
    ret

.globl _method_ReflectionClass_getattributes
_method_ReflectionClass_getattributes:
    # prologue
    push rbp
    mov rbp, rsp
    sub rsp, 48
    mov r10, QWORD PTR [rip + _concat_off]
    mov QWORD PTR [rbp - 48], r10
    # param $this from rdi
    mov QWORD PTR [rbp - 40], rdi
_eir_ReflectionClass__getAttributes_entry_0:
    mov rax, QWORD PTR [rbp - 40]
    mov QWORD PTR [rbp - 8], rax
    mov r11, QWORD PTR [rbp - 8]
    mov r10, QWORD PTR [r11 + 32]
    mov rcx, 9223372036854775805
    cmp r10, rcx
    jne _eir_ReflectionClass__getAttributes_typed_prop_initialized_0
    lea rsi, [rip + _str_11]
    mov edx, 97
    mov edi, 2
    mov eax, 1
    syscall
    mov edi, 1
    mov eax, 60
    syscall
_eir_ReflectionClass__getAttributes_typed_prop_initialized_0:
    mov rax, QWORD PTR [r11 + 24]
    mov QWORD PTR [rbp - 16], rax
    mov rax, QWORD PTR [rbp - 16]
    sub rsp, 16
    mov QWORD PTR [rsp], rax
    call __rt_incref
    mov rax, QWORD PTR [rsp]
    add rsp, 16
    mov QWORD PTR [rbp - 24], rax
    mov rax, QWORD PTR [rbp - 24]
    jmp _method_ReflectionClass_getattributes_epilogue
_method_ReflectionClass_getattributes_epilogue:
    add rsp, 48
    pop rbp
    ret

.globl _method_ReflectionMethod__u__u_construct
_method_ReflectionMethod__u__u_construct:
    # prologue
    push rbp
    mov rbp, rsp
    sub rsp, 48
    mov r10, QWORD PTR [rip + _concat_off]
    mov QWORD PTR [rbp - 48], r10
    # param $this from rdi
    mov QWORD PTR [rbp - 8], rdi
    # param $class_name from rsi,rdx
    mov QWORD PTR [rbp - 24], rsi
    mov QWORD PTR [rbp - 16], rdx
    # param $method_name from rcx,r8
    mov QWORD PTR [rbp - 40], rcx
    mov QWORD PTR [rbp - 32], r8
_eir_ReflectionMethod____construct_entry_0:
    jmp _method_ReflectionMethod__u__u_construct_epilogue
_method_ReflectionMethod__u__u_construct_epilogue:
    add rsp, 48
    pop rbp
    ret

.globl _method_ReflectionMethod_getattributes
_method_ReflectionMethod_getattributes:
    # prologue
    push rbp
    mov rbp, rsp
    sub rsp, 48
    mov r10, QWORD PTR [rip + _concat_off]
    mov QWORD PTR [rbp - 48], r10
    # param $this from rdi
    mov QWORD PTR [rbp - 40], rdi
_eir_ReflectionMethod__getAttributes_entry_0:
    mov rax, QWORD PTR [rbp - 40]
    mov QWORD PTR [rbp - 8], rax
    mov r11, QWORD PTR [rbp - 8]
    mov r10, QWORD PTR [r11 + 16]
    mov rcx, 9223372036854775805
    cmp r10, rcx
    jne _eir_ReflectionMethod__getAttributes_typed_prop_initialized_0
    lea rsi, [rip + _str_12]
    mov edx, 98
    mov edi, 2
    mov eax, 1
    syscall
    mov edi, 1
    mov eax, 60
    syscall
_eir_ReflectionMethod__getAttributes_typed_prop_initialized_0:
    mov rax, QWORD PTR [r11 + 8]
    mov QWORD PTR [rbp - 16], rax
    mov rax, QWORD PTR [rbp - 16]
    sub rsp, 16
    mov QWORD PTR [rsp], rax
    call __rt_incref
    mov rax, QWORD PTR [rsp]
    add rsp, 16
    mov QWORD PTR [rbp - 24], rax
    mov rax, QWORD PTR [rbp - 24]
    jmp _method_ReflectionMethod_getattributes_epilogue
_method_ReflectionMethod_getattributes_epilogue:
    add rsp, 48
    pop rbp
    ret

.globl _method_ReflectionProperty__u__u_construct
_method_ReflectionProperty__u__u_construct:
    # prologue
    push rbp
    mov rbp, rsp
    sub rsp, 48
    mov r10, QWORD PTR [rip + _concat_off]
    mov QWORD PTR [rbp - 48], r10
    # param $this from rdi
    mov QWORD PTR [rbp - 8], rdi
    # param $class_name from rsi,rdx
    mov QWORD PTR [rbp - 24], rsi
    mov QWORD PTR [rbp - 16], rdx
    # param $property_name from rcx,r8
    mov QWORD PTR [rbp - 40], rcx
    mov QWORD PTR [rbp - 32], r8
_eir_ReflectionProperty____construct_entry_0:
    jmp _method_ReflectionProperty__u__u_construct_epilogue
_method_ReflectionProperty__u__u_construct_epilogue:
    add rsp, 48
    pop rbp
    ret

.globl _method_ReflectionProperty_getattributes
_method_ReflectionProperty_getattributes:
    # prologue
    push rbp
    mov rbp, rsp
    sub rsp, 48
    mov r10, QWORD PTR [rip + _concat_off]
    mov QWORD PTR [rbp - 48], r10
    # param $this from rdi
    mov QWORD PTR [rbp - 40], rdi
_eir_ReflectionProperty__getAttributes_entry_0:
    mov rax, QWORD PTR [rbp - 40]
    mov QWORD PTR [rbp - 8], rax
    mov r11, QWORD PTR [rbp - 8]
    mov r10, QWORD PTR [r11 + 16]
    mov rcx, 9223372036854775805
    cmp r10, rcx
    jne _eir_ReflectionProperty__getAttributes_typed_prop_initialized_0
    lea rsi, [rip + _str_13]
    mov edx, 100
    mov edi, 2
    mov eax, 1
    syscall
    mov edi, 1
    mov eax, 60
    syscall
_eir_ReflectionProperty__getAttributes_typed_prop_initialized_0:
    mov rax, QWORD PTR [r11 + 8]
    mov QWORD PTR [rbp - 16], rax
    mov rax, QWORD PTR [rbp - 16]
    sub rsp, 16
    mov QWORD PTR [rsp], rax
    call __rt_incref
    mov rax, QWORD PTR [rsp]
    add rsp, 16
    mov QWORD PTR [rbp - 24], rax
    mov rax, QWORD PTR [rbp - 24]
    jmp _method_ReflectionProperty_getattributes_epilogue
_method_ReflectionProperty_getattributes_epilogue:
    add rsp, 48
    pop rbp
    ret

.globl _method_ReflectionFunction__u__u_construct
_method_ReflectionFunction__u__u_construct:
    # prologue
    push rbp
    mov rbp, rsp
    sub rsp, 32
    mov r10, QWORD PTR [rip + _concat_off]
    mov QWORD PTR [rbp - 32], r10
    # param $this from rdi
    mov QWORD PTR [rbp - 8], rdi
    # param $name from rsi,rdx
    mov QWORD PTR [rbp - 24], rsi
    mov QWORD PTR [rbp - 16], rdx
_eir_ReflectionFunction____construct_entry_0:
    jmp _method_ReflectionFunction__u__u_construct_epilogue
_method_ReflectionFunction__u__u_construct_epilogue:
    add rsp, 32
    pop rbp
    ret

.globl _method_ReflectionFunction_getname
_method_ReflectionFunction_getname:
    # prologue
    push rbp
    mov rbp, rsp
    sub rsp, 64
    mov r10, QWORD PTR [rip + _concat_off]
    mov QWORD PTR [rbp - 64], r10
    # param $this from rdi
    mov QWORD PTR [rbp - 56], rdi
_eir_ReflectionFunction__getName_entry_0:
    mov rax, QWORD PTR [rbp - 56]
    mov QWORD PTR [rbp - 8], rax
    mov r11, QWORD PTR [rbp - 8]
    mov r10, QWORD PTR [r11 + 16]
    mov rcx, 9223372036854775805
    cmp r10, rcx
    jne _eir_ReflectionFunction__getName_typed_prop_initialized_0
    lea rsi, [rip + _str_14]
    mov edx, 99
    mov edi, 2
    mov eax, 1
    syscall
    mov edi, 1
    mov eax, 60
    syscall
_eir_ReflectionFunction__getName_typed_prop_initialized_0:
    mov rax, QWORD PTR [r11 + 8]
    mov rdx, QWORD PTR [r11 + 16]
    mov QWORD PTR [rbp - 24], rax
    mov QWORD PTR [rbp - 16], rdx
    mov rax, QWORD PTR [rbp - 24]
    mov rdx, QWORD PTR [rbp - 16]
    call __rt_str_persist
    mov QWORD PTR [rbp - 40], rax
    mov QWORD PTR [rbp - 32], rdx
    mov rax, QWORD PTR [rbp - 40]
    mov rdx, QWORD PTR [rbp - 32]
    jmp _method_ReflectionFunction_getname_epilogue
_method_ReflectionFunction_getname_epilogue:
    add rsp, 64
    pop rbp
    ret

.globl _method_ReflectionFunction_getshortname
_method_ReflectionFunction_getshortname:
    # prologue
    push rbp
    mov rbp, rsp
    sub rsp, 64
    mov r10, QWORD PTR [rip + _concat_off]
    mov QWORD PTR [rbp - 64], r10
    # param $this from rdi
    mov QWORD PTR [rbp - 56], rdi
_eir_ReflectionFunction__getShortName_entry_0:
    mov rax, QWORD PTR [rbp - 56]
    mov QWORD PTR [rbp - 8], rax
    mov r11, QWORD PTR [rbp - 8]
    mov r10, QWORD PTR [r11 + 32]
    mov rcx, 9223372036854775805
    cmp r10, rcx
    jne _eir_ReflectionFunction__getShortName_typed_prop_initialized_0
    lea rsi, [rip + _str_15]
    mov edx, 100
    mov edi, 2
    mov eax, 1
    syscall
    mov edi, 1
    mov eax, 60
    syscall
_eir_ReflectionFunction__getShortName_typed_prop_initialized_0:
    mov rax, QWORD PTR [r11 + 24]
    mov rdx, QWORD PTR [r11 + 32]
    mov QWORD PTR [rbp - 24], rax
    mov QWORD PTR [rbp - 16], rdx
    mov rax, QWORD PTR [rbp - 24]
    mov rdx, QWORD PTR [rbp - 16]
    call __rt_str_persist
    mov QWORD PTR [rbp - 40], rax
    mov QWORD PTR [rbp - 32], rdx
    mov rax, QWORD PTR [rbp - 40]
    mov rdx, QWORD PTR [rbp - 32]
    jmp _method_ReflectionFunction_getshortname_epilogue
_method_ReflectionFunction_getshortname_epilogue:
    add rsp, 64
    pop rbp
    ret

.globl _method_ReflectionFunction_getnumberofparameters
_method_ReflectionFunction_getnumberofparameters:
    # prologue
    push rbp
    mov rbp, rsp
    sub rsp, 48
    mov r10, QWORD PTR [rip + _concat_off]
    mov QWORD PTR [rbp - 40], r10
    # save callee-saved registers used by the register allocator
    mov QWORD PTR [rbp - 32], rbx
    # param $this from rdi
    mov QWORD PTR [rbp - 24], rdi
_eir_ReflectionFunction__getNumberOfParameters_entry_0:
    mov rax, QWORD PTR [rbp - 24]
    mov QWORD PTR [rbp - 8], rax
    mov r11, QWORD PTR [rbp - 8]
    mov r10, QWORD PTR [r11 + 48]
    mov rcx, 9223372036854775805
    cmp r10, rcx
    jne _eir_ReflectionFunction__getNumberOfParameters_typed_prop_initialized_0
    lea rsi, [rip + _str_16]
    mov edx, 105
    mov edi, 2
    mov eax, 1
    syscall
    mov edi, 1
    mov eax, 60
    syscall
_eir_ReflectionFunction__getNumberOfParameters_typed_prop_initialized_0:
    mov rax, QWORD PTR [r11 + 40]
    mov rbx, rax
    mov rax, rbx
    jmp _method_ReflectionFunction_getnumberofparameters_epilogue
_method_ReflectionFunction_getnumberofparameters_epilogue:
    # restore callee-saved registers used by the register allocator
    mov rbx, QWORD PTR [rbp - 32]
    add rsp, 48
    pop rbp
    ret

.globl _method_ReflectionFunction_getnumberofrequiredparameters
_method_ReflectionFunction_getnumberofrequiredparameters:
    # prologue
    push rbp
    mov rbp, rsp
    sub rsp, 48
    mov r10, QWORD PTR [rip + _concat_off]
    mov QWORD PTR [rbp - 40], r10
    # save callee-saved registers used by the register allocator
    mov QWORD PTR [rbp - 32], rbx
    # param $this from rdi
    mov QWORD PTR [rbp - 24], rdi
_eir_ReflectionFunction__getNumberOfRequiredParameters_entry_0:
    mov rax, QWORD PTR [rbp - 24]
    mov QWORD PTR [rbp - 8], rax
    mov r11, QWORD PTR [rbp - 8]
    mov r10, QWORD PTR [r11 + 64]
    mov rcx, 9223372036854775805
    cmp r10, rcx
    jne _eir_ReflectionFunction__getNumberOfRequiredParameters_typed_prop_initialized_0
    lea rsi, [rip + _str_17]
    mov edx, 107
    mov edi, 2
    mov eax, 1
    syscall
    mov edi, 1
    mov eax, 60
    syscall
_eir_ReflectionFunction__getNumberOfRequiredParameters_typed_prop_initialized_0:
    mov rax, QWORD PTR [r11 + 56]
    mov rbx, rax
    mov rax, rbx
    jmp _method_ReflectionFunction_getnumberofrequiredparameters_epilogue
_method_ReflectionFunction_getnumberofrequiredparameters_epilogue:
    # restore callee-saved registers used by the register allocator
    mov rbx, QWORD PTR [rbp - 32]
    add rsp, 48
    pop rbp
    ret

.globl _method_ReflectionFunction_getparameters
_method_ReflectionFunction_getparameters:
    # prologue
    push rbp
    mov rbp, rsp
    sub rsp, 48
    mov r10, QWORD PTR [rip + _concat_off]
    mov QWORD PTR [rbp - 48], r10
    # param $this from rdi
    mov QWORD PTR [rbp - 40], rdi
_eir_ReflectionFunction__getParameters_entry_0:
    mov rax, QWORD PTR [rbp - 40]
    mov QWORD PTR [rbp - 8], rax
    mov r11, QWORD PTR [rbp - 8]
    mov r10, QWORD PTR [r11 + 80]
    mov rcx, 9223372036854775805
    cmp r10, rcx
    jne _eir_ReflectionFunction__getParameters_typed_prop_initialized_0
    lea rsi, [rip + _str_18]
    mov edx, 101
    mov edi, 2
    mov eax, 1
    syscall
    mov edi, 1
    mov eax, 60
    syscall
_eir_ReflectionFunction__getParameters_typed_prop_initialized_0:
    mov rax, QWORD PTR [r11 + 72]
    mov QWORD PTR [rbp - 16], rax
    mov rax, QWORD PTR [rbp - 16]
    sub rsp, 16
    mov QWORD PTR [rsp], rax
    call __rt_incref
    mov rax, QWORD PTR [rsp]
    add rsp, 16
    mov QWORD PTR [rbp - 24], rax
    mov rax, QWORD PTR [rbp - 24]
    jmp _method_ReflectionFunction_getparameters_epilogue
_method_ReflectionFunction_getparameters_epilogue:
    add rsp, 48
    pop rbp
    ret

.globl _method_ReflectionParameter_getname
_method_ReflectionParameter_getname:
    # prologue
    push rbp
    mov rbp, rsp
    sub rsp, 64
    mov r10, QWORD PTR [rip + _concat_off]
    mov QWORD PTR [rbp - 64], r10
    # param $this from rdi
    mov QWORD PTR [rbp - 56], rdi
_eir_ReflectionParameter__getName_entry_0:
    mov rax, QWORD PTR [rbp - 56]
    mov QWORD PTR [rbp - 8], rax
    mov r11, QWORD PTR [rbp - 8]
    mov r10, QWORD PTR [r11 + 16]
    mov rcx, 9223372036854775805
    cmp r10, rcx
    jne _eir_ReflectionParameter__getName_typed_prop_initialized_0
    lea rsi, [rip + _str_19]
    mov edx, 100
    mov edi, 2
    mov eax, 1
    syscall
    mov edi, 1
    mov eax, 60
    syscall
_eir_ReflectionParameter__getName_typed_prop_initialized_0:
    mov rax, QWORD PTR [r11 + 8]
    mov rdx, QWORD PTR [r11 + 16]
    mov QWORD PTR [rbp - 24], rax
    mov QWORD PTR [rbp - 16], rdx
    mov rax, QWORD PTR [rbp - 24]
    mov rdx, QWORD PTR [rbp - 16]
    call __rt_str_persist
    mov QWORD PTR [rbp - 40], rax
    mov QWORD PTR [rbp - 32], rdx
    mov rax, QWORD PTR [rbp - 40]
    mov rdx, QWORD PTR [rbp - 32]
    jmp _method_ReflectionParameter_getname_epilogue
_method_ReflectionParameter_getname_epilogue:
    add rsp, 64
    pop rbp
    ret

.globl _method_ReflectionParameter_getposition
_method_ReflectionParameter_getposition:
    # prologue
    push rbp
    mov rbp, rsp
    sub rsp, 48
    mov r10, QWORD PTR [rip + _concat_off]
    mov QWORD PTR [rbp - 40], r10
    # save callee-saved registers used by the register allocator
    mov QWORD PTR [rbp - 32], rbx
    # param $this from rdi
    mov QWORD PTR [rbp - 24], rdi
_eir_ReflectionParameter__getPosition_entry_0:
    mov rax, QWORD PTR [rbp - 24]
    mov QWORD PTR [rbp - 8], rax
    mov r11, QWORD PTR [rbp - 8]
    mov r10, QWORD PTR [r11 + 32]
    mov rcx, 9223372036854775805
    cmp r10, rcx
    jne _eir_ReflectionParameter__getPosition_typed_prop_initialized_0
    lea rsi, [rip + _str_20]
    mov edx, 104
    mov edi, 2
    mov eax, 1
    syscall
    mov edi, 1
    mov eax, 60
    syscall
_eir_ReflectionParameter__getPosition_typed_prop_initialized_0:
    mov rax, QWORD PTR [r11 + 24]
    mov rbx, rax
    mov rax, rbx
    jmp _method_ReflectionParameter_getposition_epilogue
_method_ReflectionParameter_getposition_epilogue:
    # restore callee-saved registers used by the register allocator
    mov rbx, QWORD PTR [rbp - 32]
    add rsp, 48
    pop rbp
    ret

.globl _method_ReflectionParameter_isoptional
_method_ReflectionParameter_isoptional:
    # prologue
    push rbp
    mov rbp, rsp
    sub rsp, 48
    mov r10, QWORD PTR [rip + _concat_off]
    mov QWORD PTR [rbp - 40], r10
    # save callee-saved registers used by the register allocator
    mov QWORD PTR [rbp - 32], rbx
    # param $this from rdi
    mov QWORD PTR [rbp - 24], rdi
_eir_ReflectionParameter__isOptional_entry_0:
    mov rax, QWORD PTR [rbp - 24]
    mov QWORD PTR [rbp - 8], rax
    mov r11, QWORD PTR [rbp - 8]
    mov r10, QWORD PTR [r11 + 48]
    mov rcx, 9223372036854775805
    cmp r10, rcx
    jne _eir_ReflectionParameter__isOptional_typed_prop_initialized_0
    lea rsi, [rip + _str_21]
    mov edx, 104
    mov edi, 2
    mov eax, 1
    syscall
    mov edi, 1
    mov eax, 60
    syscall
_eir_ReflectionParameter__isOptional_typed_prop_initialized_0:
    mov rax, QWORD PTR [r11 + 40]
    mov rbx, rax
    mov rax, rbx
    jmp _method_ReflectionParameter_isoptional_epilogue
_method_ReflectionParameter_isoptional_epilogue:
    # restore callee-saved registers used by the register allocator
    mov rbx, QWORD PTR [rbp - 32]
    add rsp, 48
    pop rbp
    ret

.globl _method_ReflectionParameter_isvariadic
_method_ReflectionParameter_isvariadic:
    # prologue
    push rbp
    mov rbp, rsp
    sub rsp, 48
    mov r10, QWORD PTR [rip + _concat_off]
    mov QWORD PTR [rbp - 40], r10
    # save callee-saved registers used by the register allocator
    mov QWORD PTR [rbp - 32], rbx
    # param $this from rdi
    mov QWORD PTR [rbp - 24], rdi
_eir_ReflectionParameter__isVariadic_entry_0:
    mov rax, QWORD PTR [rbp - 24]
    mov QWORD PTR [rbp - 8], rax
    mov r11, QWORD PTR [rbp - 8]
    mov r10, QWORD PTR [r11 + 64]
    mov rcx, 9223372036854775805
    cmp r10, rcx
    jne _eir_ReflectionParameter__isVariadic_typed_prop_initialized_0
    lea rsi, [rip + _str_22]
    mov edx, 104
    mov edi, 2
    mov eax, 1
    syscall
    mov edi, 1
    mov eax, 60
    syscall
_eir_ReflectionParameter__isVariadic_typed_prop_initialized_0:
    mov rax, QWORD PTR [r11 + 56]
    mov rbx, rax
    mov rax, rbx
    jmp _method_ReflectionParameter_isvariadic_epilogue
_method_ReflectionParameter_isvariadic_epilogue:
    # restore callee-saved registers used by the register allocator
    mov rbx, QWORD PTR [rbp - 32]
    add rsp, 48
    pop rbp
    ret

.globl _method_ReflectionParameter_hastype
_method_ReflectionParameter_hastype:
    # prologue
    push rbp
    mov rbp, rsp
    sub rsp, 48
    mov r10, QWORD PTR [rip + _concat_off]
    mov QWORD PTR [rbp - 40], r10
    # save callee-saved registers used by the register allocator
    mov QWORD PTR [rbp - 32], rbx
    # param $this from rdi
    mov QWORD PTR [rbp - 24], rdi
_eir_ReflectionParameter__hasType_entry_0:
    mov rax, QWORD PTR [rbp - 24]
    mov QWORD PTR [rbp - 8], rax
    mov r11, QWORD PTR [rbp - 8]
    mov r10, QWORD PTR [r11 + 80]
    mov rcx, 9223372036854775805
    cmp r10, rcx
    jne _eir_ReflectionParameter__hasType_typed_prop_initialized_0
    lea rsi, [rip + _str_23]
    mov edx, 104
    mov edi, 2
    mov eax, 1
    syscall
    mov edi, 1
    mov eax, 60
    syscall
_eir_ReflectionParameter__hasType_typed_prop_initialized_0:
    mov rax, QWORD PTR [r11 + 72]
    mov rbx, rax
    mov rax, rbx
    jmp _method_ReflectionParameter_hastype_epilogue
_method_ReflectionParameter_hastype_epilogue:
    # restore callee-saved registers used by the register allocator
    mov rbx, QWORD PTR [rbp - 32]
    add rsp, 48
    pop rbp
    ret

.globl _method_ReflectionParameter_gettype
_method_ReflectionParameter_gettype:
    # prologue
    push rbp
    mov rbp, rsp
    sub rsp, 32
    mov r10, QWORD PTR [rip + _concat_off]
    mov QWORD PTR [rbp - 32], r10
    # param $this from rdi
    mov QWORD PTR [rbp - 24], rdi
_eir_ReflectionParameter__getType_entry_0:
    mov rax, QWORD PTR [rbp - 24]
    mov QWORD PTR [rbp - 8], rax
    mov r11, QWORD PTR [rbp - 8]
    mov r10, QWORD PTR [r11 + 96]
    mov rcx, 9223372036854775805
    cmp r10, rcx
    jne _eir_ReflectionParameter__getType_typed_prop_initialized_0
    lea rsi, [rip + _str_24]
    mov edx, 100
    mov edi, 2
    mov eax, 1
    syscall
    mov edi, 1
    mov eax, 60
    syscall
_eir_ReflectionParameter__getType_typed_prop_initialized_0:
    mov rax, QWORD PTR [r11 + 88]
    sub rsp, 16
    mov QWORD PTR [rsp], rax
    call __rt_incref
    mov rax, QWORD PTR [rsp]
    add rsp, 16
    mov QWORD PTR [rbp - 16], rax
    mov rax, QWORD PTR [rbp - 16]
    jmp _method_ReflectionParameter_gettype_epilogue
_method_ReflectionParameter_gettype_epilogue:
    add rsp, 32
    pop rbp
    ret

.globl _method_ReflectionNamedType_getname
_method_ReflectionNamedType_getname:
    # prologue
    push rbp
    mov rbp, rsp
    sub rsp, 64
    mov r10, QWORD PTR [rip + _concat_off]
    mov QWORD PTR [rbp - 64], r10
    # param $this from rdi
    mov QWORD PTR [rbp - 56], rdi
_eir_ReflectionNamedType__getName_entry_0:
    mov rax, QWORD PTR [rbp - 56]
    mov QWORD PTR [rbp - 8], rax
    mov r11, QWORD PTR [rbp - 8]
    mov r10, QWORD PTR [r11 + 16]
    mov rcx, 9223372036854775805
    cmp r10, rcx
    jne _eir_ReflectionNamedType__getName_typed_prop_initialized_0
    lea rsi, [rip + _str_25]
    mov edx, 100
    mov edi, 2
    mov eax, 1
    syscall
    mov edi, 1
    mov eax, 60
    syscall
_eir_ReflectionNamedType__getName_typed_prop_initialized_0:
    mov rax, QWORD PTR [r11 + 8]
    mov rdx, QWORD PTR [r11 + 16]
    mov QWORD PTR [rbp - 24], rax
    mov QWORD PTR [rbp - 16], rdx
    mov rax, QWORD PTR [rbp - 24]
    mov rdx, QWORD PTR [rbp - 16]
    call __rt_str_persist
    mov QWORD PTR [rbp - 40], rax
    mov QWORD PTR [rbp - 32], rdx
    mov rax, QWORD PTR [rbp - 40]
    mov rdx, QWORD PTR [rbp - 32]
    jmp _method_ReflectionNamedType_getname_epilogue
_method_ReflectionNamedType_getname_epilogue:
    add rsp, 64
    pop rbp
    ret

.globl _method_ReflectionNamedType_allowsnull
_method_ReflectionNamedType_allowsnull:
    # prologue
    push rbp
    mov rbp, rsp
    sub rsp, 48
    mov r10, QWORD PTR [rip + _concat_off]
    mov QWORD PTR [rbp - 40], r10
    # save callee-saved registers used by the register allocator
    mov QWORD PTR [rbp - 32], rbx
    # param $this from rdi
    mov QWORD PTR [rbp - 24], rdi
_eir_ReflectionNamedType__allowsNull_entry_0:
    mov rax, QWORD PTR [rbp - 24]
    mov QWORD PTR [rbp - 8], rax
    mov r11, QWORD PTR [rbp - 8]
    mov r10, QWORD PTR [r11 + 32]
    mov rcx, 9223372036854775805
    cmp r10, rcx
    jne _eir_ReflectionNamedType__allowsNull_typed_prop_initialized_0
    lea rsi, [rip + _str_26]
    mov edx, 107
    mov edi, 2
    mov eax, 1
    syscall
    mov edi, 1
    mov eax, 60
    syscall
_eir_ReflectionNamedType__allowsNull_typed_prop_initialized_0:
    mov rax, QWORD PTR [r11 + 24]
    mov rbx, rax
    mov rax, rbx
    jmp _method_ReflectionNamedType_allowsnull_epilogue
_method_ReflectionNamedType_allowsnull_epilogue:
    # restore callee-saved registers used by the register allocator
    mov rbx, QWORD PTR [rbp - 32]
    add rsp, 48
    pop rbp
    ret

.globl _method_ReflectionNamedType_isbuiltin
_method_ReflectionNamedType_isbuiltin:
    # prologue
    push rbp
    mov rbp, rsp
    sub rsp, 48
    mov r10, QWORD PTR [rip + _concat_off]
    mov QWORD PTR [rbp - 40], r10
    # save callee-saved registers used by the register allocator
    mov QWORD PTR [rbp - 32], rbx
    # param $this from rdi
    mov QWORD PTR [rbp - 24], rdi
_eir_ReflectionNamedType__isBuiltin_entry_0:
    mov rax, QWORD PTR [rbp - 24]
    mov QWORD PTR [rbp - 8], rax
    mov r11, QWORD PTR [rbp - 8]
    mov r10, QWORD PTR [r11 + 48]
    mov rcx, 9223372036854775805
    cmp r10, rcx
    jne _eir_ReflectionNamedType__isBuiltin_typed_prop_initialized_0
    lea rsi, [rip + _str_27]
    mov edx, 103
    mov edi, 2
    mov eax, 1
    syscall
    mov edi, 1
    mov eax, 60
    syscall
_eir_ReflectionNamedType__isBuiltin_typed_prop_initialized_0:
    mov rax, QWORD PTR [r11 + 40]
    mov rbx, rax
    mov rax, rbx
    jmp _method_ReflectionNamedType_isbuiltin_epilogue
_method_ReflectionNamedType_isbuiltin_epilogue:
    # restore callee-saved registers used by the register allocator
    mov rbx, QWORD PTR [rbp - 32]
    add rsp, 48
    pop rbp
    ret

.globl main
main:
    # prologue
    push rbp
    mov rbp, rsp
    sub rsp, 416
    mov r10, QWORD PTR [rip + _concat_off]
    mov QWORD PTR [rbp - 416], r10
    # save callee-saved registers used by the register allocator
    mov QWORD PTR [rbp - 408], rbx
    # save argc/argv to globals
    mov QWORD PTR [rip + _global_argc], rdi
    mov QWORD PTR [rip + _global_argv], rsi
    mov QWORD PTR [rbp - 392], 0
    mov QWORD PTR [rbp - 400], 0
    # initialize enum singleton AIC\Components\Domain\HeadAssetMode::default
    mov rax, 40
    call __rt_heap_alloc
    mov r10, 0x454c504800000004
    mov QWORD PTR [rax - 8], r10
    mov r10, 89
    mov QWORD PTR [rax], r10
    mov QWORD PTR [rax + 8], 0
    mov QWORD PTR [rax + 16], 0
    mov QWORD PTR [rax + 24], 0
    mov QWORD PTR [rax + 32], 0
    lea r10, [rip + _str_28]
    mov QWORD PTR [rax + 8], r10
    mov r10, 7
    mov QWORD PTR [rax + 16], r10
    lea r10, [rip + _str_28]
    mov QWORD PTR [rax + 24], r10
    mov r10, 7
    mov QWORD PTR [rax + 32], r10
    mov QWORD PTR [rip + _enum_case_AIC_N_Components_N_Domain_N_HeadAssetMode_default], rax
    # initialize enum singleton AIC\Components\Domain\HeadAssetMode::Module
    mov rax, 40
    call __rt_heap_alloc
    mov r10, 0x454c504800000004
    mov QWORD PTR [rax - 8], r10
    mov r10, 89
    mov QWORD PTR [rax], r10
    mov QWORD PTR [rax + 8], 0
    mov QWORD PTR [rax + 16], 0
    mov QWORD PTR [rax + 24], 0
    mov QWORD PTR [rax + 32], 0
    lea r10, [rip + _str_29]
    mov QWORD PTR [rax + 8], r10
    mov r10, 6
    mov QWORD PTR [rax + 16], r10
    lea r10, [rip + _str_30]
    mov QWORD PTR [rax + 24], r10
    mov r10, 6
    mov QWORD PTR [rax + 32], r10
    mov QWORD PTR [rip + _enum_case_AIC_N_Components_N_Domain_N_HeadAssetMode_Module], rax
    # initialize enum singleton AIC\Components\Domain\HeadAssetMode::Defer
    mov rax, 40
    call __rt_heap_alloc
    mov r10, 0x454c504800000004
    mov QWORD PTR [rax - 8], r10
    mov r10, 89
    mov QWORD PTR [rax], r10
    mov QWORD PTR [rax + 8], 0
    mov QWORD PTR [rax + 16], 0
    mov QWORD PTR [rax + 24], 0
    mov QWORD PTR [rax + 32], 0
    lea r10, [rip + _str_31]
    mov QWORD PTR [rax + 8], r10
    mov r10, 5
    mov QWORD PTR [rax + 16], r10
    lea r10, [rip + _str_32]
    mov QWORD PTR [rax + 24], r10
    mov r10, 5
    mov QWORD PTR [rax + 32], r10
    mov QWORD PTR [rip + _enum_case_AIC_N_Components_N_Domain_N_HeadAssetMode_Defer], rax
    # initialize enum singleton AIC\Components\Domain\HeadAssetMode::Async
    mov rax, 40
    call __rt_heap_alloc
    mov r10, 0x454c504800000004
    mov QWORD PTR [rax - 8], r10
    mov r10, 89
    mov QWORD PTR [rax], r10
    mov QWORD PTR [rax + 8], 0
    mov QWORD PTR [rax + 16], 0
    mov QWORD PTR [rax + 24], 0
    mov QWORD PTR [rax + 32], 0
    lea r10, [rip + _str_33]
    mov QWORD PTR [rax + 8], r10
    mov r10, 5
    mov QWORD PTR [rax + 16], r10
    lea r10, [rip + _str_34]
    mov QWORD PTR [rax + 24], r10
    mov r10, 5
    mov QWORD PTR [rax + 32], r10
    mov QWORD PTR [rip + _enum_case_AIC_N_Components_N_Domain_N_HeadAssetMode_Async], rax
    # initialize enum singleton AIC\Components\Domain\HeadAssetType::Css
    mov rax, 40
    call __rt_heap_alloc
    mov r10, 0x454c504800000004
    mov QWORD PTR [rax - 8], r10
    mov r10, 88
    mov QWORD PTR [rax], r10
    mov QWORD PTR [rax + 8], 0
    mov QWORD PTR [rax + 16], 0
    mov QWORD PTR [rax + 24], 0
    mov QWORD PTR [rax + 32], 0
    lea r10, [rip + _str_35]
    mov QWORD PTR [rax + 8], r10
    mov r10, 3
    mov QWORD PTR [rax + 16], r10
    lea r10, [rip + _str_36]
    mov QWORD PTR [rax + 24], r10
    mov r10, 3
    mov QWORD PTR [rax + 32], r10
    mov QWORD PTR [rip + _enum_case_AIC_N_Components_N_Domain_N_HeadAssetType_Css], rax
    # initialize enum singleton AIC\Components\Domain\HeadAssetType::Js
    mov rax, 40
    call __rt_heap_alloc
    mov r10, 0x454c504800000004
    mov QWORD PTR [rax - 8], r10
    mov r10, 88
    mov QWORD PTR [rax], r10
    mov QWORD PTR [rax + 8], 0
    mov QWORD PTR [rax + 16], 0
    mov QWORD PTR [rax + 24], 0
    mov QWORD PTR [rax + 32], 0
    lea r10, [rip + _str_37]
    mov QWORD PTR [rax + 8], r10
    mov r10, 2
    mov QWORD PTR [rax + 16], r10
    lea r10, [rip + _str_38]
    mov QWORD PTR [rax + 24], r10
    mov r10, 2
    mov QWORD PTR [rax + 32], r10
    mov QWORD PTR [rip + _enum_case_AIC_N_Components_N_Domain_N_HeadAssetType_Js], rax
_eir_main_entry_0:
    # @src line=18 col=1
    mov r10, QWORD PTR [rbp - 416]
    mov QWORD PTR [rip + _concat_off], r10
    # @src line=18 col=1
    # @src line=31 col=1
    mov r10, QWORD PTR [rbp - 416]
    mov QWORD PTR [rip + _concat_off], r10
    # @src line=31 col=1
    # @src line=25 col=1
    mov r10, QWORD PTR [rbp - 416]
    mov QWORD PTR [rip + _concat_off], r10
    # @src line=25 col=1
    # @src line=2 col=1
    mov r10, QWORD PTR [rbp - 416]
    mov QWORD PTR [rip + _concat_off], r10
    # @src line=2 col=1
    mov rax, QWORD PTR [rip + _include_once_ec694a5f6a7cb541]
    test rax, rax
    jne _eir_main_include_once_already_0
    mov rax, 1
    mov QWORD PTR [rip + _include_once_ec694a5f6a7cb541], rax
    jmp _eir_main_include_once_done_1
_eir_main_include_once_already_0:
    mov rax, 0
_eir_main_include_once_done_1:
    mov rbx, rax
    mov rax, rbx
    test rax, rax
    jne _eir_main_include_once_body_1
    jmp _eir_main_include_once_after_2
_eir_main_include_once_body_1:
    # @src line=7 col=1
    mov r10, QWORD PTR [rbp - 416]
    mov QWORD PTR [rip + _concat_off], r10
    # @src line=7 col=1
    mov rax, QWORD PTR [rip + _include_once_9ed3b5f87c95977a]
    test rax, rax
    jne _eir_main_include_once_already_2
    mov rax, 1
    mov QWORD PTR [rip + _include_once_9ed3b5f87c95977a], rax
    jmp _eir_main_include_once_done_3
_eir_main_include_once_already_2:
    mov rax, 0
_eir_main_include_once_done_3:
    mov rbx, rax
    mov rax, rbx
    test rax, rax
    jne _eir_main_include_once_after_4
    jmp _eir_main_include_once_after_4
_eir_main_include_once_after_2:
    # @src line=6 col=1
    mov r10, QWORD PTR [rbp - 416]
    mov QWORD PTR [rip + _concat_off], r10
    # @src line=6 col=11
    mov rdi, 4
    mov rsi, 8
    call __rt_array_new
    sub rsp, 16
    mov QWORD PTR [rsp], r12
    mov r10, QWORD PTR [rax - 8]
    mov r12, 0xffffffff000080ff
    and r10, r12
    mov r12, 6
    shl r12, 8
    or r10, r12
    mov QWORD PTR [rax - 8], r10
    mov r12, QWORD PTR [rsp]
    add rsp, 16
    mov QWORD PTR [rbp - 40], rax
    # @src line=7 col=19
    mov rax, QWORD PTR [rip + _enum_case_AIC_N_Components_N_Domain_N_HeadAssetType_Css]
    mov QWORD PTR [rbp - 48], rax
    # @src line=7 col=39
    lea rax, [rip + _str_39]
    mov rdx, 14
    mov QWORD PTR [rbp - 64], rax
    mov QWORD PTR [rbp - 56], rdx
    # @src line=47 col=37
    mov rax, 9223372036854775806
    mov rbx, rax
    # @src line=48 col=39
    mov rax, 9223372036854775806
    mov QWORD PTR [rbp - 80], rax
    # @src line=49 col=38
    mov rax, QWORD PTR [rip + _enum_case_AIC_N_Components_N_Domain_N_HeadAssetMode_default]
    mov QWORD PTR [rbp - 88], rax
    # @src line=7 col=5
    mov rax, 88
    call __rt_heap_alloc
    mov r10, 0x454c504800000004
    mov QWORD PTR [rax - 8], r10
    mov r10, 30
    mov QWORD PTR [rax], r10
    mov QWORD PTR [rax + 8], 0
    mov QWORD PTR [rax + 16], 0
    mov QWORD PTR [rax + 24], 0
    mov QWORD PTR [rax + 32], 0
    mov QWORD PTR [rax + 40], 0
    mov QWORD PTR [rax + 48], 0
    mov QWORD PTR [rax + 56], 0
    mov QWORD PTR [rax + 64], 0
    mov QWORD PTR [rax + 72], 0
    mov QWORD PTR [rax + 80], 0
    mov r10, 9223372036854775805
    mov QWORD PTR [rax + 16], r10
    mov QWORD PTR [rax + 32], r10
    mov QWORD PTR [rax + 48], r10
    mov QWORD PTR [rax + 64], r10
    mov QWORD PTR [rax + 80], r10
    mov QWORD PTR [rbp - 96], rax
    mov rax, QWORD PTR [rbp - 96]
    sub rsp, 16
    mov QWORD PTR [rsp], rax
    mov rax, QWORD PTR [rbp - 48]
    sub rsp, 16
    mov QWORD PTR [rsp], rax
    mov rax, QWORD PTR [rbp - 64]
    mov rdx, QWORD PTR [rbp - 56]
    sub rsp, 16
    mov QWORD PTR [rsp], rax
    mov QWORD PTR [rsp + 8], rdx
    mov rax, rbx
    mov rdi, rax
    xor rsi, rsi
    mov rax, 8
    call __rt_mixed_from_value
    sub rsp, 16
    mov QWORD PTR [rsp], rax
    mov rax, QWORD PTR [rbp - 80]
    mov rdi, rax
    xor rsi, rsi
    mov rax, 8
    call __rt_mixed_from_value
    sub rsp, 16
    mov QWORD PTR [rsp], rax
    mov rax, QWORD PTR [rbp - 88]
    sub rsp, 16
    mov QWORD PTR [rsp], rax
    sub rsp, 32
    mov rdi, QWORD PTR [rsp + 112]
    mov rsi, QWORD PTR [rsp + 96]
    mov rdx, QWORD PTR [rsp + 80]
    mov rcx, QWORD PTR [rsp + 88]
    mov r8, QWORD PTR [rsp + 64]
    mov r9, QWORD PTR [rsp + 48]
    mov r10, QWORD PTR [rsp + 32]
    mov QWORD PTR [rsp], r10
    mov r10, QWORD PTR [rsp]
    mov QWORD PTR [rsp + 112], r10
    add rsp, 112
    call _method_AIC_N_Components_N_Domain_N_HeadAsset__u__u_construct
    add rsp, 16
    # @src line=7 col=5
    mov r11, QWORD PTR [rbp - 40]
    mov rsi, QWORD PTR [rbp - 96]
    mov rdi, r11
    call __rt_array_push_refcounted
    mov QWORD PTR [rbp - 40], rax
    # @src line=7 col=5
    mov rax, QWORD PTR [rbp - 96]
    call __rt_decref_object
    # @src line=8 col=19
    mov rax, QWORD PTR [rip + _enum_case_AIC_N_Components_N_Domain_N_HeadAssetType_Js]
    mov QWORD PTR [rbp - 104], rax
    # @src line=8 col=38
    lea rax, [rip + _str_40]
    mov rdx, 13
    mov QWORD PTR [rbp - 120], rax
    mov QWORD PTR [rbp - 112], rdx
    # @src line=8 col=55
    mov rax, 9223372036854775806
    mov rbx, rax
    # @src line=8 col=61
    mov rax, 9223372036854775806
    mov QWORD PTR [rbp - 136], rax
    # @src line=8 col=67
    mov rax, QWORD PTR [rip + _enum_case_AIC_N_Components_N_Domain_N_HeadAssetMode_Module]
    mov QWORD PTR [rbp - 144], rax
    # @src line=8 col=5
    mov rax, 88
    call __rt_heap_alloc
    mov r10, 0x454c504800000004
    mov QWORD PTR [rax - 8], r10
    mov r10, 30
    mov QWORD PTR [rax], r10
    mov QWORD PTR [rax + 8], 0
    mov QWORD PTR [rax + 16], 0
    mov QWORD PTR [rax + 24], 0
    mov QWORD PTR [rax + 32], 0
    mov QWORD PTR [rax + 40], 0
    mov QWORD PTR [rax + 48], 0
    mov QWORD PTR [rax + 56], 0
    mov QWORD PTR [rax + 64], 0
    mov QWORD PTR [rax + 72], 0
    mov QWORD PTR [rax + 80], 0
    mov r10, 9223372036854775805
    mov QWORD PTR [rax + 16], r10
    mov QWORD PTR [rax + 32], r10
    mov QWORD PTR [rax + 48], r10
    mov QWORD PTR [rax + 64], r10
    mov QWORD PTR [rax + 80], r10
    mov QWORD PTR [rbp - 152], rax
    mov rax, QWORD PTR [rbp - 152]
    sub rsp, 16
    mov QWORD PTR [rsp], rax
    mov rax, QWORD PTR [rbp - 104]
    sub rsp, 16
    mov QWORD PTR [rsp], rax
    mov rax, QWORD PTR [rbp - 120]
    mov rdx, QWORD PTR [rbp - 112]
    sub rsp, 16
    mov QWORD PTR [rsp], rax
    mov QWORD PTR [rsp + 8], rdx
    mov rax, rbx
    mov rdi, rax
    xor rsi, rsi
    mov rax, 8
    call __rt_mixed_from_value
    sub rsp, 16
    mov QWORD PTR [rsp], rax
    mov rax, QWORD PTR [rbp - 136]
    mov rdi, rax
    xor rsi, rsi
    mov rax, 8
    call __rt_mixed_from_value
    sub rsp, 16
    mov QWORD PTR [rsp], rax
    mov rax, QWORD PTR [rbp - 144]
    sub rsp, 16
    mov QWORD PTR [rsp], rax
    sub rsp, 32
    mov rdi, QWORD PTR [rsp + 112]
    mov rsi, QWORD PTR [rsp + 96]
    mov rdx, QWORD PTR [rsp + 80]
    mov rcx, QWORD PTR [rsp + 88]
    mov r8, QWORD PTR [rsp + 64]
    mov r9, QWORD PTR [rsp + 48]
    mov r10, QWORD PTR [rsp + 32]
    mov QWORD PTR [rsp], r10
    mov r10, QWORD PTR [rsp]
    mov QWORD PTR [rsp + 112], r10
    add rsp, 112
    call _method_AIC_N_Components_N_Domain_N_HeadAsset__u__u_construct
    add rsp, 16
    # @src line=8 col=5
    mov r11, QWORD PTR [rbp - 40]
    mov rsi, QWORD PTR [rbp - 152]
    mov rdi, r11
    call __rt_array_push_refcounted
    mov QWORD PTR [rbp - 40], rax
    # @src line=8 col=5
    mov rax, QWORD PTR [rbp - 152]
    call __rt_decref_object
    # @src line=9 col=19
    mov rax, QWORD PTR [rip + _enum_case_AIC_N_Components_N_Domain_N_HeadAssetType_Js]
    mov QWORD PTR [rbp - 160], rax
    # @src line=9 col=38
    lea rax, [rip + _str_41]
    mov rdx, 15
    mov QWORD PTR [rbp - 176], rax
    mov QWORD PTR [rbp - 168], rdx
    # @src line=9 col=57
    mov rax, 9223372036854775806
    mov rbx, rax
    # @src line=9 col=63
    mov rax, 9223372036854775806
    mov QWORD PTR [rbp - 192], rax
    # @src line=9 col=69
    mov rax, QWORD PTR [rip + _enum_case_AIC_N_Components_N_Domain_N_HeadAssetMode_Defer]
    mov QWORD PTR [rbp - 200], rax
    # @src line=9 col=5
    mov rax, 88
    call __rt_heap_alloc
    mov r10, 0x454c504800000004
    mov QWORD PTR [rax - 8], r10
    mov r10, 30
    mov QWORD PTR [rax], r10
    mov QWORD PTR [rax + 8], 0
    mov QWORD PTR [rax + 16], 0
    mov QWORD PTR [rax + 24], 0
    mov QWORD PTR [rax + 32], 0
    mov QWORD PTR [rax + 40], 0
    mov QWORD PTR [rax + 48], 0
    mov QWORD PTR [rax + 56], 0
    mov QWORD PTR [rax + 64], 0
    mov QWORD PTR [rax + 72], 0
    mov QWORD PTR [rax + 80], 0
    mov r10, 9223372036854775805
    mov QWORD PTR [rax + 16], r10
    mov QWORD PTR [rax + 32], r10
    mov QWORD PTR [rax + 48], r10
    mov QWORD PTR [rax + 64], r10
    mov QWORD PTR [rax + 80], r10
    mov QWORD PTR [rbp - 208], rax
    mov rax, QWORD PTR [rbp - 208]
    sub rsp, 16
    mov QWORD PTR [rsp], rax
    mov rax, QWORD PTR [rbp - 160]
    sub rsp, 16
    mov QWORD PTR [rsp], rax
    mov rax, QWORD PTR [rbp - 176]
    mov rdx, QWORD PTR [rbp - 168]
    sub rsp, 16
    mov QWORD PTR [rsp], rax
    mov QWORD PTR [rsp + 8], rdx
    mov rax, rbx
    mov rdi, rax
    xor rsi, rsi
    mov rax, 8
    call __rt_mixed_from_value
    sub rsp, 16
    mov QWORD PTR [rsp], rax
    mov rax, QWORD PTR [rbp - 192]
    mov rdi, rax
    xor rsi, rsi
    mov rax, 8
    call __rt_mixed_from_value
    sub rsp, 16
    mov QWORD PTR [rsp], rax
    mov rax, QWORD PTR [rbp - 200]
    sub rsp, 16
    mov QWORD PTR [rsp], rax
    sub rsp, 32
    mov rdi, QWORD PTR [rsp + 112]
    mov rsi, QWORD PTR [rsp + 96]
    mov rdx, QWORD PTR [rsp + 80]
    mov rcx, QWORD PTR [rsp + 88]
    mov r8, QWORD PTR [rsp + 64]
    mov r9, QWORD PTR [rsp + 48]
    mov r10, QWORD PTR [rsp + 32]
    mov QWORD PTR [rsp], r10
    mov r10, QWORD PTR [rsp]
    mov QWORD PTR [rsp + 112], r10
    add rsp, 112
    call _method_AIC_N_Components_N_Domain_N_HeadAsset__u__u_construct
    add rsp, 16
    # @src line=9 col=5
    mov r11, QWORD PTR [rbp - 40]
    mov rsi, QWORD PTR [rbp - 208]
    mov rdi, r11
    call __rt_array_push_refcounted
    mov QWORD PTR [rbp - 40], rax
    # @src line=9 col=5
    mov rax, QWORD PTR [rbp - 208]
    call __rt_decref_object
    # @src line=6 col=1
    mov rax, QWORD PTR [rbp - 40]
    sub rsp, 16
    mov QWORD PTR [rsp], rax
    call __rt_incref
    mov rax, QWORD PTR [rsp]
    add rsp, 16
    mov QWORD PTR [rbp - 216], rax
    # @src line=6 col=1
    mov rax, QWORD PTR [rbp - 216]
    mov QWORD PTR [rbp - 392], rax
    # @src line=6 col=1
    mov rax, QWORD PTR [rbp - 40]
    call __rt_decref_array
    # @src line=11 col=1
    mov r10, QWORD PTR [rbp - 416]
    mov QWORD PTR [rip + _concat_off], r10
    # @src line=11 col=10
    mov rax, QWORD PTR [rbp - 392]
    mov QWORD PTR [rbp - 224], rax
    # @src line=11 col=10
    mov rax, QWORD PTR [rbp - 224]
    mov QWORD PTR [rbp - 288], rax
    mov rax, -1
    mov QWORD PTR [rbp - 280], rax
    # @src line=11 col=10
    mov rax, 9223372036854775806
    mov rbx, rax
    # @src line=11 col=10
    mov rax, rbx
    mov rdi, rax
    xor rsi, rsi
    mov rax, 8
    call __rt_mixed_from_value
    mov QWORD PTR [rbp - 304], rax
    # @src line=11 col=10
    mov rax, QWORD PTR [rbp - 304]
    sub rsp, 16
    mov QWORD PTR [rsp], rax
    call __rt_incref
    mov rax, QWORD PTR [rsp]
    add rsp, 16
    mov QWORD PTR [rbp - 312], rax
    # @src line=11 col=10
    mov rax, QWORD PTR [rbp - 312]
    mov QWORD PTR [rbp - 400], rax
    # @src line=11 col=10
    mov rax, QWORD PTR [rbp - 304]
    call __rt_decref_mixed
    jmp _eir_main_foreach_next_9
_eir_main_include_once_body_3:
    ud2
_eir_main_include_once_after_4:
    # @src line=8 col=1
    mov r10, QWORD PTR [rbp - 416]
    mov QWORD PTR [rip + _concat_off], r10
    # @src line=8 col=1
    mov rax, QWORD PTR [rip + _include_once_66b98ae6e3ea3371]
    test rax, rax
    jne _eir_main_include_once_already_4
    mov rax, 1
    mov QWORD PTR [rip + _include_once_66b98ae6e3ea3371], rax
    jmp _eir_main_include_once_done_5
_eir_main_include_once_already_4:
    mov rax, 0
_eir_main_include_once_done_5:
    mov rbx, rax
    mov rax, rbx
    test rax, rax
    jne _eir_main_include_once_after_6
    jmp _eir_main_include_once_after_6
_eir_main_include_once_body_5:
    ud2
_eir_main_include_once_after_6:
    # @src line=9 col=1
    mov r10, QWORD PTR [rbp - 416]
    mov QWORD PTR [rip + _concat_off], r10
    # @src line=9 col=1
    mov rax, QWORD PTR [rip + _include_once_5814db2e10c69672]
    test rax, rax
    jne _eir_main_include_once_already_6
    mov rax, 1
    mov QWORD PTR [rip + _include_once_5814db2e10c69672], rax
    jmp _eir_main_include_once_done_7
_eir_main_include_once_already_6:
    mov rax, 0
_eir_main_include_once_done_7:
    mov rbx, rax
    mov rax, rbx
    test rax, rax
    jne _eir_main_include_once_after_2
    jmp _eir_main_include_once_after_2
_eir_main_include_once_body_7:
    ud2
_eir_main_include_once_after_8:
    ud2
_eir_main_foreach_next_9:
    # @src line=11 col=10
    mov r11, QWORD PTR [rbp - 288]
    mov r10, QWORD PTR [rbp - 280]
    add r10, 1
    mov rcx, QWORD PTR [r11]
    cmp r10, rcx
    setl al
    movzx rax, al
    jge _eir_main_iter_next_done_8
    mov QWORD PTR [rbp - 280], r10
_eir_main_iter_next_done_8:
    mov rbx, rax
    mov rax, rbx
    test rax, rax
    jne _eir_main_foreach_body_10
    jmp _eir_main_foreach_exit_11
_eir_main_foreach_body_10:
    # @src line=11 col=10
    mov r11, QWORD PTR [rbp - 288]
    mov r10, QWORD PTR [rbp - 280]
    lea r11, [r11 + 24]
    mov rax, QWORD PTR [r11 + r10 * 8]
    mov rdi, rax
    xor rsi, rsi
    mov rax, 6
    call __rt_mixed_from_value
    mov QWORD PTR [rbp - 328], rax
    # @src line=11 col=10
    mov rax, QWORD PTR [rbp - 400]
    mov QWORD PTR [rbp - 336], rax
    # @src line=11 col=10
    mov rax, QWORD PTR [rbp - 336]
    call __rt_decref_mixed
    # @src line=11 col=10
    mov rax, QWORD PTR [rbp - 328]
    sub rsp, 16
    mov QWORD PTR [rsp], rax
    call __rt_incref
    mov rax, QWORD PTR [rsp]
    add rsp, 16
    mov QWORD PTR [rbp - 344], rax
    # @src line=11 col=10
    mov rax, QWORD PTR [rbp - 344]
    mov QWORD PTR [rbp - 400], rax
    # @src line=11 col=10
    mov rax, QWORD PTR [rbp - 328]
    call __rt_decref_mixed
    # @src line=11 col=27
    mov r10, QWORD PTR [rbp - 416]
    mov QWORD PTR [rip + _concat_off], r10
    # @src line=11 col=27
    mov r10, QWORD PTR [rbp - 416]
    mov QWORD PTR [rip + _concat_off], r10
    # @src line=11 col=32
    mov rax, QWORD PTR [rbp - 400]
    mov QWORD PTR [rbp - 352], rax
    # @src line=11 col=34
    mov rax, QWORD PTR [rbp - 352]
    call __rt_mixed_unbox
    cmp rax, 6
    jne _eir_main_mixed_method_no_match_9
    mov r12, rdi
    mov r11, QWORD PTR [r12]
    mov r10, 30
    cmp r11, r10
    je _eir_main_mixed_method_AIC_Components_Domain_HeadAsset_11
    jmp _eir_main_mixed_method_no_match_9
_eir_main_mixed_method_AIC_Components_Domain_HeadAsset_11:
    mov rax, r12
    sub rsp, 16
    mov QWORD PTR [rsp], rax
    mov rdi, QWORD PTR [rsp]
    add rsp, 16
    mov r10, QWORD PTR [rdi]
    lea r11, [rip + _class_vtable_ptrs]
    mov r11, QWORD PTR [r11 + r10 * 8]
    mov r11, QWORD PTR [r11 + 8]
    call r11
    mov QWORD PTR [rbp - 368], rax
    mov QWORD PTR [rbp - 360], rdx
    jmp _eir_main_mixed_method_done_10
_eir_main_mixed_method_no_match_9:
    mov edi, 2
    lea rsi, [rip + _str_42]
    mov edx, 58
    mov eax, 1
    syscall
    mov edi, 1
    mov eax, 60
    syscall
_eir_main_mixed_method_done_10:
    # @src line=11 col=27
    mov rax, QWORD PTR [rbp - 368]
    mov rdx, QWORD PTR [rbp - 360]

    # echo
    mov rsi, rdx
    mov rdi, rax
    call __rt_stdout_write
    # @src line=11 col=27
    mov rax, QWORD PTR [rbp - 368]
    mov rdx, QWORD PTR [rbp - 360]
    call __rt_heap_free_safe
    # @src line=11 col=27
    mov r10, QWORD PTR [rbp - 416]
    mov QWORD PTR [rip + _concat_off], r10
    # @src line=11 col=48
    lea rax, [rip + _str_43]
    mov rdx, 1
    mov QWORD PTR [rbp - 384], rax
    mov QWORD PTR [rbp - 376], rdx
    # @src line=11 col=27
    mov rax, QWORD PTR [rbp - 384]
    mov rdx, QWORD PTR [rbp - 376]

    # echo
    mov rsi, rdx
    mov rdi, rax
    call __rt_stdout_write
    jmp _eir_main_foreach_next_9
_eir_main_foreach_exit_11:

    # epilogue + exit(0)
    # epilogue cleanup $assets
    mov rax, QWORD PTR [rbp - 392]
    test rax, rax
    je _eir_main_main_refcounted_cleanup_done_12
    call __rt_decref_array
_eir_main_main_refcounted_cleanup_done_12:
    # epilogue cleanup $a
    mov rax, QWORD PTR [rbp - 400]
    test rax, rax
    je _eir_main_main_refcounted_cleanup_done_13
    call __rt_decref_mixed
_eir_main_main_refcounted_cleanup_done_13:
    # restore callee-saved registers used by the register allocator
    mov rbx, QWORD PTR [rbp - 408]
    add rsp, 416
    pop rbp
    mov edi, 0
    mov eax, 60
    syscall
_static_Fiber_getcurrent:
    jmp __rt_fiber_get_current
_method_Fiber_getreturn:
    jmp __rt_fiber_get_return
_method_Fiber_isrunning:
    jmp __rt_fiber_state_eq
_method_Fiber_isstarted:
    jmp __rt_fiber_state_eq
_method_Fiber_issuspended:
    jmp __rt_fiber_state_eq
_method_Fiber_isterminated:
    jmp __rt_fiber_state_eq
_method_Fiber_resume:
    jmp __rt_fiber_resume
_method_Fiber_start:
    jmp __rt_fiber_start
_static_Fiber_suspend:
    jmp __rt_fiber_suspend
_method_Fiber_throw:
    jmp __rt_fiber_throw
_method_Generator_current:
    jmp __rt_gen_current
_method_Generator_getreturn:
    jmp __rt_gen_get_return
_method_Generator_key:
    jmp __rt_gen_key
_method_Generator_next:
    jmp __rt_gen_next
_method_Generator_rewind:
    jmp __rt_gen_rewind
_method_Generator_send:
    jmp __rt_gen_send
_method_Generator_throw:
    jmp __rt_gen_throw
_method_Generator_valid:
    jmp __rt_gen_valid
_method_SplDoublyLinkedList__u__u_serialize:
    jmp __rt_spl_dll_serialize_array
_method_SplDoublyLinkedList_add:
    jmp __rt_spl_dll_insert
_method_SplDoublyLinkedList_bottom:
    jmp __rt_spl_dll_bottom
_method_SplDoublyLinkedList_count:
    jmp __rt_spl_dll_count
_method_SplDoublyLinkedList_current:
    jmp __rt_spl_dll_current
_method_SplDoublyLinkedList_getiteratormode:
    jmp __rt_spl_dll_get_iterator_mode
_method_SplDoublyLinkedList_isempty:
    jmp __rt_spl_dll_is_empty
_method_SplDoublyLinkedList_key:
    jmp __rt_spl_dll_key
_method_SplDoublyLinkedList_next:
    jmp __rt_spl_dll_next
_method_SplDoublyLinkedList_offsetexists:
    jmp __rt_spl_dll_offset_exists
_method_SplDoublyLinkedList_offsetget:
    jmp __rt_spl_dll_offset_get
_method_SplDoublyLinkedList_offsetset:
    jmp __rt_spl_dll_offset_set
_method_SplDoublyLinkedList_offsetunset:
    jmp __rt_spl_dll_offset_unset
_method_SplDoublyLinkedList_pop:
    jmp __rt_spl_dll_pop
_method_SplDoublyLinkedList_prev:
    jmp __rt_spl_dll_prev
_method_SplDoublyLinkedList_push:
    jmp __rt_spl_dll_push
_method_SplDoublyLinkedList_rewind:
    jmp __rt_spl_dll_rewind
_method_SplDoublyLinkedList_serialize:
    jmp __rt_spl_dll_serialize
_method_SplDoublyLinkedList_setiteratormode:
    jmp __rt_spl_dll_set_iterator_mode
_method_SplDoublyLinkedList_shift:
    jmp __rt_spl_dll_shift
_method_SplDoublyLinkedList_top:
    jmp __rt_spl_dll_top
_method_SplDoublyLinkedList_unserialize:
    jmp __rt_spl_dll_unserialize
_method_SplDoublyLinkedList_unshift:
    jmp __rt_spl_dll_unshift
_method_SplDoublyLinkedList_valid:
    jmp __rt_spl_dll_valid
_method_SplFixedArray__u__u_construct:
    jmp __rt_spl_fixed_set_size
_method_SplFixedArray__u__u_unserialize:
    jmp __rt_spl_fixed_unserialize
_method_SplFixedArray_count:
    jmp __rt_spl_fixed_count
_static_SplFixedArray_fromarray:
    jmp __rt_spl_fixed_from_array
_method_SplFixedArray_getsize:
    jmp __rt_spl_fixed_count
_method_SplFixedArray_jsonserialize:
    jmp __rt_spl_fixed_to_array
_method_SplFixedArray_offsetexists:
    jmp __rt_spl_fixed_offset_exists
_method_SplFixedArray_offsetget:
    jmp __rt_spl_fixed_offset_get
_method_SplFixedArray_offsetset:
    jmp __rt_spl_fixed_offset_set
_method_SplFixedArray_offsetunset:
    jmp __rt_spl_fixed_offset_unset
_method_SplFixedArray_setsize:
    jmp __rt_spl_fixed_set_size
_method_SplFixedArray_toarray:
    jmp __rt_spl_fixed_to_array
_method_SplQueue_dequeue:
    jmp __rt_spl_dll_shift
_method_SplQueue_enqueue:
    jmp __rt_spl_dll_push

.data
.comm _include_once_ec694a5f6a7cb541, 8, 3
.comm _include_once_9ed3b5f87c95977a, 8, 3
.comm _include_once_66b98ae6e3ea3371, 8, 3
.comm _include_once_5814db2e10c69672, 8, 3
.globl _str_0
_str_0:
    .ascii ""
.globl _str_1
_str_1:
    .ascii "UTC"
.globl _str_3
_str_3:
    .ascii "Fatal error: Typed property AIC\\Components\\Domain\\HeadAsset::$type must not be accessed before initialization\n"
.globl _str_4
_str_4:
    .ascii "Fatal error: Typed property AIC\\Components\\Domain\\HeadAssetType::$value must not be accessed before initialization\n"
.globl _str_5
_str_5:
    .ascii ":"
.globl _str_6
_str_6:
    .ascii "Fatal error: Typed property AIC\\Components\\Domain\\HeadAsset::$mode must not be accessed before initialization\n"
.globl _str_7
_str_7:
    .ascii "Fatal error: Typed property AIC\\Components\\Domain\\HeadAssetMode::$value must not be accessed before initialization\n"
.globl _str_8
_str_8:
    .ascii "Fatal error: Typed property AIC\\Components\\Domain\\HeadAsset::$url must not be accessed before initialization\n"
.globl _str_9
_str_9:
    .ascii "Fatal error: Typed property ReflectionAttribute::$__name must not be accessed before initialization\n"
.globl _str_10
_str_10:
    .ascii "Fatal error: Typed property ReflectionClass::$__name must not be accessed before initialization\n"
.globl _str_11
_str_11:
    .ascii "Fatal error: Typed property ReflectionClass::$__attrs must not be accessed before initialization\n"
.globl _str_12
_str_12:
    .ascii "Fatal error: Typed property ReflectionMethod::$__attrs must not be accessed before initialization\n"
.globl _str_13
_str_13:
    .ascii "Fatal error: Typed property ReflectionProperty::$__attrs must not be accessed before initialization\n"
.globl _str_14
_str_14:
    .ascii "Fatal error: Typed property ReflectionFunction::$__name must not be accessed before initialization\n"
.globl _str_15
_str_15:
    .ascii "Fatal error: Typed property ReflectionFunction::$__short must not be accessed before initialization\n"
.globl _str_16
_str_16:
    .ascii "Fatal error: Typed property ReflectionFunction::$__num_params must not be accessed before initialization\n"
.globl _str_17
_str_17:
    .ascii "Fatal error: Typed property ReflectionFunction::$__num_required must not be accessed before initialization\n"
.globl _str_18
_str_18:
    .ascii "Fatal error: Typed property ReflectionFunction::$__params must not be accessed before initialization\n"
.globl _str_19
_str_19:
    .ascii "Fatal error: Typed property ReflectionParameter::$__name must not be accessed before initialization\n"
.globl _str_20
_str_20:
    .ascii "Fatal error: Typed property ReflectionParameter::$__position must not be accessed before initialization\n"
.globl _str_21
_str_21:
    .ascii "Fatal error: Typed property ReflectionParameter::$__optional must not be accessed before initialization\n"
.globl _str_22
_str_22:
    .ascii "Fatal error: Typed property ReflectionParameter::$__variadic must not be accessed before initialization\n"
.globl _str_23
_str_23:
    .ascii "Fatal error: Typed property ReflectionParameter::$__has_type must not be accessed before initialization\n"
.globl _str_24
_str_24:
    .ascii "Fatal error: Typed property ReflectionParameter::$__type must not be accessed before initialization\n"
.globl _str_25
_str_25:
    .ascii "Fatal error: Typed property ReflectionNamedType::$__name must not be accessed before initialization\n"
.globl _str_26
_str_26:
    .ascii "Fatal error: Typed property ReflectionNamedType::$__allows_null must not be accessed before initialization\n"
.globl _str_27
_str_27:
    .ascii "Fatal error: Typed property ReflectionNamedType::$__builtin must not be accessed before initialization\n"
.globl _str_28
_str_28:
    .ascii "default"
.globl _str_29
_str_29:
    .ascii "module"
.globl _str_30
_str_30:
    .ascii "Module"
.globl _str_31
_str_31:
    .ascii "defer"
.globl _str_32
_str_32:
    .ascii "Defer"
.globl _str_33
_str_33:
    .ascii "async"
.globl _str_34
_str_34:
    .ascii "Async"
.globl _str_35
_str_35:
    .ascii "css"
.globl _str_36
_str_36:
    .ascii "Css"
.globl _str_37
_str_37:
    .ascii "js"
.globl _str_38
_str_38:
    .ascii "Js"
.globl _str_39
_str_39:
    .ascii "/build/app.css"
.globl _str_40
_str_40:
    .ascii "/build/app.js"
.globl _str_41
_str_41:
    .ascii "/build/defer.js"
.globl _str_42
_str_42:
    .ascii "Fatal error: Call to a member function dedupKey() on null\n"
.globl _str_43
_str_43:
    .ascii "\n"
.p2align 3
.globl _float_2
_float_2:
    .quad 0x0000000000000000

.comm _enum_case_AIC_N_Components_N_Domain_N_HeadAssetMode_default, 8, 3
.comm _enum_case_AIC_N_Components_N_Domain_N_HeadAssetMode_Module, 8, 3
.comm _enum_case_AIC_N_Components_N_Domain_N_HeadAssetMode_Defer, 8, 3
.comm _enum_case_AIC_N_Components_N_Domain_N_HeadAssetMode_Async, 8, 3
.comm _enum_case_AIC_N_Components_N_Domain_N_HeadAssetType_Css, 8, 3
.comm _enum_case_AIC_N_Components_N_Domain_N_HeadAssetType_Js, 8, 3
.comm _enum_case_SortDirection_Ascending, 8, 3
.comm _enum_case_SortDirection_Descending, 8, 3
.data
.p2align 3
.globl _callable_user_fn_name_0
_callable_user_fn_name_0:
    .ascii "main"
.p2align 3
.globl _callable_user_function_count
_callable_user_function_count:
    .quad 1
.globl _callable_user_function_table
_callable_user_function_table:
    .quad _callable_user_fn_name_0
    .quad 4
    .quad 0
.p2align 3
.globl _instanceof_target_count
_instanceof_target_count:
    .quad 44
.globl _instanceof_target_entries
_instanceof_target_entries:
    .quad _instanceof_name_class_0
    .quad 9
    .quad 0
    .quad 0
    .quad _instanceof_name_class_abs_0
    .quad 10
    .quad 0
    .quad 0
    .quad _instanceof_name_class_6
    .quad 19
    .quad 6
    .quad 0
    .quad _instanceof_name_class_abs_6
    .quad 20
    .quad 6
    .quad 0
    .quad _instanceof_name_class_13
    .quad 18
    .quad 13
    .quad 0
    .quad _instanceof_name_class_abs_13
    .quad 19
    .quad 13
    .quad 0
    .quad _instanceof_name_class_14
    .quad 5
    .quad 14
    .quad 0
    .quad _instanceof_name_class_abs_14
    .quad 6
    .quad 14
    .quad 0
    .quad _instanceof_name_class_18
    .quad 16
    .quad 18
    .quad 0
    .quad _instanceof_name_class_abs_18
    .quad 17
    .quad 18
    .quad 0
    .quad _instanceof_name_class_24
    .quad 19
    .quad 24
    .quad 0
    .quad _instanceof_name_class_abs_24
    .quad 20
    .quad 24
    .quad 0
    .quad _instanceof_name_class_30
    .quad 31
    .quad 30
    .quad 0
    .quad _instanceof_name_class_abs_30
    .quad 32
    .quad 30
    .quad 0
    .quad _instanceof_name_class_37
    .quad 9
    .quad 37
    .quad 0
    .quad _instanceof_name_class_abs_37
    .quad 10
    .quad 37
    .quad 0
    .quad _instanceof_name_class_40
    .quad 18
    .quad 40
    .quad 0
    .quad _instanceof_name_class_abs_40
    .quad 19
    .quad 40
    .quad 0
    .quad _instanceof_name_class_41
    .quad 14
    .quad 41
    .quad 0
    .quad _instanceof_name_class_abs_41
    .quad 15
    .quad 41
    .quad 0
    .quad _instanceof_name_class_42
    .quad 19
    .quad 42
    .quad 0
    .quad _instanceof_name_class_abs_42
    .quad 20
    .quad 42
    .quad 0
    .quad _instanceof_name_class_46
    .quad 16
    .quad 46
    .quad 0
    .quad _instanceof_name_class_abs_46
    .quad 17
    .quad 46
    .quad 0
    .quad _instanceof_name_class_47
    .quad 13
    .quad 47
    .quad 0
    .quad _instanceof_name_class_abs_47
    .quad 14
    .quad 47
    .quad 0
    .quad _instanceof_name_class_54
    .quad 15
    .quad 54
    .quad 0
    .quad _instanceof_name_class_abs_54
    .quad 16
    .quad 54
    .quad 0
    .quad _instanceof_name_class_60
    .quad 19
    .quad 60
    .quad 0
    .quad _instanceof_name_class_abs_60
    .quad 20
    .quad 60
    .quad 0
    .quad _instanceof_name_class_75
    .quad 20
    .quad 75
    .quad 0
    .quad _instanceof_name_class_abs_75
    .quad 21
    .quad 75
    .quad 0
    .quad _instanceof_name_class_78
    .quad 10
    .quad 78
    .quad 0
    .quad _instanceof_name_class_abs_78
    .quad 11
    .quad 78
    .quad 0
    .quad _instanceof_name_class_80
    .quad 24
    .quad 80
    .quad 0
    .quad _instanceof_name_class_abs_80
    .quad 25
    .quad 80
    .quad 0
    .quad _instanceof_name_class_88
    .quad 35
    .quad 88
    .quad 0
    .quad _instanceof_name_class_abs_88
    .quad 36
    .quad 88
    .quad 0
    .quad _instanceof_name_class_89
    .quad 35
    .quad 89
    .quad 0
    .quad _instanceof_name_class_abs_89
    .quad 36
    .quad 89
    .quad 0
    .quad _instanceof_name_interface_8
    .quad 9
    .quad 8
    .quad 1
    .quad _instanceof_name_interface_abs_8
    .quad 10
    .quad 8
    .quad 1
    .quad _instanceof_name_interface_12
    .quad 10
    .quad 12
    .quad 1
    .quad _instanceof_name_interface_abs_12
    .quad 11
    .quad 12
    .quad 1
.globl _instanceof_name_class_0
_instanceof_name_class_0:
    .ascii "Exception"
.globl _instanceof_name_class_abs_0
_instanceof_name_class_abs_0:
    .ascii "\\Exception"
.globl _instanceof_name_class_6
_instanceof_name_class_6:
    .ascii "ReflectionParameter"
.globl _instanceof_name_class_abs_6
_instanceof_name_class_abs_6:
    .ascii "\\ReflectionParameter"
.globl _instanceof_name_class_13
_instanceof_name_class_13:
    .ascii "ReflectionFunction"
.globl _instanceof_name_class_abs_13
_instanceof_name_class_abs_13:
    .ascii "\\ReflectionFunction"
.globl _instanceof_name_class_14
_instanceof_name_class_14:
    .ascii "Error"
.globl _instanceof_name_class_abs_14
_instanceof_name_class_abs_14:
    .ascii "\\Error"
.globl _instanceof_name_class_18
_instanceof_name_class_18:
    .ascii "ReflectionMethod"
.globl _instanceof_name_class_abs_18
_instanceof_name_class_abs_18:
    .ascii "\\ReflectionMethod"
.globl _instanceof_name_class_24
_instanceof_name_class_24:
    .ascii "ReflectionNamedType"
.globl _instanceof_name_class_abs_24
_instanceof_name_class_abs_24:
    .ascii "\\ReflectionNamedType"
.globl _instanceof_name_class_30
_instanceof_name_class_30:
    .ascii "AIC\\Components\\Domain\\HeadAsset"
.globl _instanceof_name_class_abs_30
_instanceof_name_class_abs_30:
    .ascii "\\AIC\\Components\\Domain\\HeadAsset"
.globl _instanceof_name_class_37
_instanceof_name_class_37:
    .ascii "TypeError"
.globl _instanceof_name_class_abs_37
_instanceof_name_class_abs_37:
    .ascii "\\TypeError"
.globl _instanceof_name_class_40
_instanceof_name_class_40:
    .ascii "ReflectionProperty"
.globl _instanceof_name_class_abs_40
_instanceof_name_class_abs_40:
    .ascii "\\ReflectionProperty"
.globl _instanceof_name_class_41
_instanceof_name_class_41:
    .ascii "LogicException"
.globl _instanceof_name_class_abs_41
_instanceof_name_class_abs_41:
    .ascii "\\LogicException"
.globl _instanceof_name_class_42
_instanceof_name_class_42:
    .ascii "OutOfRangeException"
.globl _instanceof_name_class_abs_42
_instanceof_name_class_abs_42:
    .ascii "\\OutOfRangeException"
.globl _instanceof_name_class_46
_instanceof_name_class_46:
    .ascii "RuntimeException"
.globl _instanceof_name_class_abs_46
_instanceof_name_class_abs_46:
    .ascii "\\RuntimeException"
.globl _instanceof_name_class_47
_instanceof_name_class_47:
    .ascii "JsonException"
.globl _instanceof_name_class_abs_47
_instanceof_name_class_abs_47:
    .ascii "\\JsonException"
.globl _instanceof_name_class_54
_instanceof_name_class_54:
    .ascii "ReflectionClass"
.globl _instanceof_name_class_abs_54
_instanceof_name_class_abs_54:
    .ascii "\\ReflectionClass"
.globl _instanceof_name_class_60
_instanceof_name_class_60:
    .ascii "ReflectionAttribute"
.globl _instanceof_name_class_abs_60
_instanceof_name_class_abs_60:
    .ascii "\\ReflectionAttribute"
.globl _instanceof_name_class_75
_instanceof_name_class_75:
    .ascii "OutOfBoundsException"
.globl _instanceof_name_class_abs_75
_instanceof_name_class_abs_75:
    .ascii "\\OutOfBoundsException"
.globl _instanceof_name_class_78
_instanceof_name_class_78:
    .ascii "ValueError"
.globl _instanceof_name_class_abs_78
_instanceof_name_class_abs_78:
    .ascii "\\ValueError"
.globl _instanceof_name_class_80
_instanceof_name_class_80:
    .ascii "InvalidArgumentException"
.globl _instanceof_name_class_abs_80
_instanceof_name_class_abs_80:
    .ascii "\\InvalidArgumentException"
.globl _instanceof_name_class_88
_instanceof_name_class_88:
    .ascii "AIC\\Components\\Domain\\HeadAssetType"
.globl _instanceof_name_class_abs_88
_instanceof_name_class_abs_88:
    .ascii "\\AIC\\Components\\Domain\\HeadAssetType"
.globl _instanceof_name_class_89
_instanceof_name_class_89:
    .ascii "AIC\\Components\\Domain\\HeadAssetMode"
.globl _instanceof_name_class_abs_89
_instanceof_name_class_abs_89:
    .ascii "\\AIC\\Components\\Domain\\HeadAssetMode"
.globl _instanceof_name_interface_8
_instanceof_name_interface_8:
    .ascii "Throwable"
.globl _instanceof_name_interface_abs_8
_instanceof_name_interface_abs_8:
    .ascii "\\Throwable"
.globl _instanceof_name_interface_12
_instanceof_name_interface_12:
    .ascii "Stringable"
.globl _instanceof_name_interface_abs_12
_instanceof_name_interface_abs_12:
    .ascii "\\Stringable"
    .p2align 3
.p2align 3
.globl _class_name_count
_class_name_count:
    .quad 90
.globl _class_name_entries
_class_name_entries:
    .quad _class_name_0
    .quad 9
    .quad _class_name_missing
    .quad 0
    .quad _class_name_missing
    .quad 0
    .quad _class_name_missing
    .quad 0
    .quad _class_name_missing
    .quad 0
    .quad _class_name_missing
    .quad 0
    .quad _class_name_6
    .quad 19
    .quad _class_name_missing
    .quad 0
    .quad _class_name_missing
    .quad 0
    .quad _class_name_missing
    .quad 0
    .quad _class_name_missing
    .quad 0
    .quad _class_name_missing
    .quad 0
    .quad _class_name_missing
    .quad 0
    .quad _class_name_13
    .quad 18
    .quad _class_name_14
    .quad 5
    .quad _class_name_missing
    .quad 0
    .quad _class_name_missing
    .quad 0
    .quad _class_name_missing
    .quad 0
    .quad _class_name_18
    .quad 16
    .quad _class_name_missing
    .quad 0
    .quad _class_name_missing
    .quad 0
    .quad _class_name_missing
    .quad 0
    .quad _class_name_missing
    .quad 0
    .quad _class_name_missing
    .quad 0
    .quad _class_name_24
    .quad 19
    .quad _class_name_missing
    .quad 0
    .quad _class_name_missing
    .quad 0
    .quad _class_name_missing
    .quad 0
    .quad _class_name_missing
    .quad 0
    .quad _class_name_missing
    .quad 0
    .quad _class_name_30
    .quad 31
    .quad _class_name_missing
    .quad 0
    .quad _class_name_missing
    .quad 0
    .quad _class_name_missing
    .quad 0
    .quad _class_name_missing
    .quad 0
    .quad _class_name_missing
    .quad 0
    .quad _class_name_missing
    .quad 0
    .quad _class_name_37
    .quad 9
    .quad _class_name_missing
    .quad 0
    .quad _class_name_missing
    .quad 0
    .quad _class_name_40
    .quad 18
    .quad _class_name_41
    .quad 14
    .quad _class_name_42
    .quad 19
    .quad _class_name_missing
    .quad 0
    .quad _class_name_missing
    .quad 0
    .quad _class_name_missing
    .quad 0
    .quad _class_name_46
    .quad 16
    .quad _class_name_47
    .quad 13
    .quad _class_name_missing
    .quad 0
    .quad _class_name_missing
    .quad 0
    .quad _class_name_missing
    .quad 0
    .quad _class_name_missing
    .quad 0
    .quad _class_name_missing
    .quad 0
    .quad _class_name_missing
    .quad 0
    .quad _class_name_54
    .quad 15
    .quad _class_name_missing
    .quad 0
    .quad _class_name_missing
    .quad 0
    .quad _class_name_missing
    .quad 0
    .quad _class_name_missing
    .quad 0
    .quad _class_name_missing
    .quad 0
    .quad _class_name_60
    .quad 19
    .quad _class_name_missing
    .quad 0
    .quad _class_name_missing
    .quad 0
    .quad _class_name_missing
    .quad 0
    .quad _class_name_missing
    .quad 0
    .quad _class_name_missing
    .quad 0
    .quad _class_name_missing
    .quad 0
    .quad _class_name_missing
    .quad 0
    .quad _class_name_missing
    .quad 0
    .quad _class_name_missing
    .quad 0
    .quad _class_name_missing
    .quad 0
    .quad _class_name_missing
    .quad 0
    .quad _class_name_missing
    .quad 0
    .quad _class_name_missing
    .quad 0
    .quad _class_name_missing
    .quad 0
    .quad _class_name_75
    .quad 20
    .quad _class_name_missing
    .quad 0
    .quad _class_name_missing
    .quad 0
    .quad _class_name_78
    .quad 10
    .quad _class_name_missing
    .quad 0
    .quad _class_name_80
    .quad 24
    .quad _class_name_missing
    .quad 0
    .quad _class_name_missing
    .quad 0
    .quad _class_name_missing
    .quad 0
    .quad _class_name_missing
    .quad 0
    .quad _class_name_missing
    .quad 0
    .quad _class_name_missing
    .quad 0
    .quad _class_name_missing
    .quad 0
    .quad _class_name_88
    .quad 35
    .quad _class_name_89
    .quad 35
.globl _class_name_missing
_class_name_missing:
    .byte 0
.globl _class_name_0
_class_name_0:
    .ascii "Exception"
.globl _class_name_6
_class_name_6:
    .ascii "ReflectionParameter"
.globl _class_name_13
_class_name_13:
    .ascii "ReflectionFunction"
.globl _class_name_14
_class_name_14:
    .ascii "Error"
.globl _class_name_18
_class_name_18:
    .ascii "ReflectionMethod"
.globl _class_name_24
_class_name_24:
    .ascii "ReflectionNamedType"
.globl _class_name_30
_class_name_30:
    .ascii "AIC\\Components\\Domain\\HeadAsset"
.globl _class_name_37
_class_name_37:
    .ascii "TypeError"
.globl _class_name_40
_class_name_40:
    .ascii "ReflectionProperty"
.globl _class_name_41
_class_name_41:
    .ascii "LogicException"
.globl _class_name_42
_class_name_42:
    .ascii "OutOfRangeException"
.globl _class_name_46
_class_name_46:
    .ascii "RuntimeException"
.globl _class_name_47
_class_name_47:
    .ascii "JsonException"
.globl _class_name_54
_class_name_54:
    .ascii "ReflectionClass"
.globl _class_name_60
_class_name_60:
    .ascii "ReflectionAttribute"
.globl _class_name_75
_class_name_75:
    .ascii "OutOfBoundsException"
.globl _class_name_78
_class_name_78:
    .ascii "ValueError"
.globl _class_name_80
_class_name_80:
    .ascii "InvalidArgumentException"
.globl _class_name_88
_class_name_88:
    .ascii "AIC\\Components\\Domain\\HeadAssetType"
.globl _class_name_89
_class_name_89:
    .ascii "AIC\\Components\\Domain\\HeadAssetMode"
    .p2align 3
.globl _fiber_class_id
_fiber_class_id:
    .quad 38
.globl _fiber_error_class_id
_fiber_error_class_id:
    .quad 39
.globl _generator_class_id
_generator_class_id:
    .quad 49
.globl _spl_dll_class_id
_spl_dll_class_id:
    .quad 33
.globl _spl_stack_class_id
_spl_stack_class_id:
    .quad 50
.globl _spl_queue_class_id
_spl_queue_class_id:
    .quad 61
.globl _spl_fixed_array_class_id
_spl_fixed_array_class_id:
    .quad 10
.globl _spl_logic_exception_class_id
_spl_logic_exception_class_id:
    .quad 41
.globl _spl_runtime_exception_class_id
_spl_runtime_exception_class_id:
    .quad 46
.globl _spl_out_of_range_exception_class_id
_spl_out_of_range_exception_class_id:
    .quad 42
.globl _spl_out_of_bounds_exception_class_id
_spl_out_of_bounds_exception_class_id:
    .quad 75
.globl _spl_invalid_argument_exception_class_id
_spl_invalid_argument_exception_class_id:
    .quad 80
.globl _spl_type_error_class_id
_spl_type_error_class_id:
    .quad 37
.globl _spl_value_error_class_id
_spl_value_error_class_id:
    .quad 78
.globl _interface_count
_interface_count:
    .quad 2
.globl _interface_method_ptrs
_interface_method_ptrs:
    .quad _interface_methods_8
    .quad _interface_methods_12
.globl _class_interface_ptrs
_class_interface_ptrs:
    .quad _class_interfaces_0
    .quad _class_interfaces_missing
    .quad _class_interfaces_missing
    .quad _class_interfaces_missing
    .quad _class_interfaces_missing
    .quad _class_interfaces_missing
    .quad _class_interfaces_6
    .quad _class_interfaces_missing
    .quad _class_interfaces_missing
    .quad _class_interfaces_missing
    .quad _class_interfaces_missing
    .quad _class_interfaces_missing
    .quad _class_interfaces_missing
    .quad _class_interfaces_13
    .quad _class_interfaces_14
    .quad _class_interfaces_missing
    .quad _class_interfaces_missing
    .quad _class_interfaces_missing
    .quad _class_interfaces_18
    .quad _class_interfaces_missing
    .quad _class_interfaces_missing
    .quad _class_interfaces_missing
    .quad _class_interfaces_missing
    .quad _class_interfaces_missing
    .quad _class_interfaces_24
    .quad _class_interfaces_missing
    .quad _class_interfaces_missing
    .quad _class_interfaces_missing
    .quad _class_interfaces_missing
    .quad _class_interfaces_missing
    .quad _class_interfaces_30
    .quad _class_interfaces_missing
    .quad _class_interfaces_missing
    .quad _class_interfaces_missing
    .quad _class_interfaces_missing
    .quad _class_interfaces_missing
    .quad _class_interfaces_missing
    .quad _class_interfaces_37
    .quad _class_interfaces_missing
    .quad _class_interfaces_missing
    .quad _class_interfaces_40
    .quad _class_interfaces_41
    .quad _class_interfaces_42
    .quad _class_interfaces_missing
    .quad _class_interfaces_missing
    .quad _class_interfaces_missing
    .quad _class_interfaces_46
    .quad _class_interfaces_47
    .quad _class_interfaces_missing
    .quad _class_interfaces_missing
    .quad _class_interfaces_missing
    .quad _class_interfaces_missing
    .quad _class_interfaces_missing
    .quad _class_interfaces_missing
    .quad _class_interfaces_54
    .quad _class_interfaces_missing
    .quad _class_interfaces_missing
    .quad _class_interfaces_missing
    .quad _class_interfaces_missing
    .quad _class_interfaces_missing
    .quad _class_interfaces_60
    .quad _class_interfaces_missing
    .quad _class_interfaces_missing
    .quad _class_interfaces_missing
    .quad _class_interfaces_missing
    .quad _class_interfaces_missing
    .quad _class_interfaces_missing
    .quad _class_interfaces_missing
    .quad _class_interfaces_missing
    .quad _class_interfaces_missing
    .quad _class_interfaces_missing
    .quad _class_interfaces_missing
    .quad _class_interfaces_missing
    .quad _class_interfaces_missing
    .quad _class_interfaces_missing
    .quad _class_interfaces_75
    .quad _class_interfaces_missing
    .quad _class_interfaces_missing
    .quad _class_interfaces_78
    .quad _class_interfaces_missing
    .quad _class_interfaces_80
    .quad _class_interfaces_missing
    .quad _class_interfaces_missing
    .quad _class_interfaces_missing
    .quad _class_interfaces_missing
    .quad _class_interfaces_missing
    .quad _class_interfaces_missing
    .quad _class_interfaces_missing
    .quad _class_interfaces_88
    .quad _class_interfaces_89
.globl _class_json_desc_ptrs
_class_json_desc_ptrs:
    .quad _class_json_desc_0
    .quad _class_json_desc_missing
    .quad _class_json_desc_missing
    .quad _class_json_desc_missing
    .quad _class_json_desc_missing
    .quad _class_json_desc_missing
    .quad _class_json_desc_6
    .quad _class_json_desc_missing
    .quad _class_json_desc_missing
    .quad _class_json_desc_missing
    .quad _class_json_desc_missing
    .quad _class_json_desc_missing
    .quad _class_json_desc_missing
    .quad _class_json_desc_13
    .quad _class_json_desc_14
    .quad _class_json_desc_missing
    .quad _class_json_desc_missing
    .quad _class_json_desc_missing
    .quad _class_json_desc_18
    .quad _class_json_desc_missing
    .quad _class_json_desc_missing
    .quad _class_json_desc_missing
    .quad _class_json_desc_missing
    .quad _class_json_desc_missing
    .quad _class_json_desc_24
    .quad _class_json_desc_missing
    .quad _class_json_desc_missing
    .quad _class_json_desc_missing
    .quad _class_json_desc_missing
    .quad _class_json_desc_missing
    .quad _class_json_desc_30
    .quad _class_json_desc_missing
    .quad _class_json_desc_missing
    .quad _class_json_desc_missing
    .quad _class_json_desc_missing
    .quad _class_json_desc_missing
    .quad _class_json_desc_missing
    .quad _class_json_desc_37
    .quad _class_json_desc_missing
    .quad _class_json_desc_missing
    .quad _class_json_desc_40
    .quad _class_json_desc_41
    .quad _class_json_desc_42
    .quad _class_json_desc_missing
    .quad _class_json_desc_missing
    .quad _class_json_desc_missing
    .quad _class_json_desc_46
    .quad _class_json_desc_47
    .quad _class_json_desc_missing
    .quad _class_json_desc_missing
    .quad _class_json_desc_missing
    .quad _class_json_desc_missing
    .quad _class_json_desc_missing
    .quad _class_json_desc_missing
    .quad _class_json_desc_54
    .quad _class_json_desc_missing
    .quad _class_json_desc_missing
    .quad _class_json_desc_missing
    .quad _class_json_desc_missing
    .quad _class_json_desc_missing
    .quad _class_json_desc_60
    .quad _class_json_desc_missing
    .quad _class_json_desc_missing
    .quad _class_json_desc_missing
    .quad _class_json_desc_missing
    .quad _class_json_desc_missing
    .quad _class_json_desc_missing
    .quad _class_json_desc_missing
    .quad _class_json_desc_missing
    .quad _class_json_desc_missing
    .quad _class_json_desc_missing
    .quad _class_json_desc_missing
    .quad _class_json_desc_missing
    .quad _class_json_desc_missing
    .quad _class_json_desc_missing
    .quad _class_json_desc_75
    .quad _class_json_desc_missing
    .quad _class_json_desc_missing
    .quad _class_json_desc_78
    .quad _class_json_desc_missing
    .quad _class_json_desc_80
    .quad _class_json_desc_missing
    .quad _class_json_desc_missing
    .quad _class_json_desc_missing
    .quad _class_json_desc_missing
    .quad _class_json_desc_missing
    .quad _class_json_desc_missing
    .quad _class_json_desc_missing
    .quad _class_json_desc_88
    .quad _class_json_desc_89
.globl _json_exception_class_id
_json_exception_class_id:
    .quad 47
.globl _class_parent_ids
_class_parent_ids:
    .quad -1
    .quad -1
    .quad -1
    .quad -1
    .quad -1
    .quad -1
    .quad -1
    .quad -1
    .quad -1
    .quad -1
    .quad -1
    .quad -1
    .quad -1
    .quad -1
    .quad -1
    .quad -1
    .quad -1
    .quad -1
    .quad -1
    .quad -1
    .quad -1
    .quad -1
    .quad -1
    .quad -1
    .quad -1
    .quad -1
    .quad -1
    .quad -1
    .quad -1
    .quad -1
    .quad -1
    .quad -1
    .quad -1
    .quad -1
    .quad -1
    .quad -1
    .quad -1
    .quad 14
    .quad -1
    .quad -1
    .quad -1
    .quad 0
    .quad 41
    .quad -1
    .quad -1
    .quad -1
    .quad 0
    .quad 46
    .quad -1
    .quad -1
    .quad -1
    .quad -1
    .quad -1
    .quad -1
    .quad -1
    .quad -1
    .quad -1
    .quad -1
    .quad -1
    .quad -1
    .quad -1
    .quad -1
    .quad -1
    .quad -1
    .quad -1
    .quad -1
    .quad -1
    .quad -1
    .quad -1
    .quad -1
    .quad -1
    .quad -1
    .quad -1
    .quad -1
    .quad -1
    .quad 46
    .quad -1
    .quad -1
    .quad 14
    .quad -1
    .quad 41
    .quad -1
    .quad -1
    .quad -1
    .quad -1
    .quad -1
    .quad -1
    .quad -1
    .quad -1
    .quad -1
.globl _class_gc_desc_count
_class_gc_desc_count:
    .quad 90
.globl _class_gc_desc_ptrs
_class_gc_desc_ptrs:
    .quad _class_gc_desc_0
    .quad _class_gc_desc_missing
    .quad _class_gc_desc_missing
    .quad _class_gc_desc_missing
    .quad _class_gc_desc_missing
    .quad _class_gc_desc_missing
    .quad _class_gc_desc_6
    .quad _class_gc_desc_missing
    .quad _class_gc_desc_missing
    .quad _class_gc_desc_missing
    .quad _class_gc_desc_missing
    .quad _class_gc_desc_missing
    .quad _class_gc_desc_missing
    .quad _class_gc_desc_13
    .quad _class_gc_desc_14
    .quad _class_gc_desc_missing
    .quad _class_gc_desc_missing
    .quad _class_gc_desc_missing
    .quad _class_gc_desc_18
    .quad _class_gc_desc_missing
    .quad _class_gc_desc_missing
    .quad _class_gc_desc_missing
    .quad _class_gc_desc_missing
    .quad _class_gc_desc_missing
    .quad _class_gc_desc_24
    .quad _class_gc_desc_missing
    .quad _class_gc_desc_missing
    .quad _class_gc_desc_missing
    .quad _class_gc_desc_missing
    .quad _class_gc_desc_missing
    .quad _class_gc_desc_30
    .quad _class_gc_desc_missing
    .quad _class_gc_desc_missing
    .quad _class_gc_desc_missing
    .quad _class_gc_desc_missing
    .quad _class_gc_desc_missing
    .quad _class_gc_desc_missing
    .quad _class_gc_desc_37
    .quad _class_gc_desc_missing
    .quad _class_gc_desc_missing
    .quad _class_gc_desc_40
    .quad _class_gc_desc_41
    .quad _class_gc_desc_42
    .quad _class_gc_desc_missing
    .quad _class_gc_desc_missing
    .quad _class_gc_desc_missing
    .quad _class_gc_desc_46
    .quad _class_gc_desc_47
    .quad _class_gc_desc_missing
    .quad _class_gc_desc_missing
    .quad _class_gc_desc_missing
    .quad _class_gc_desc_missing
    .quad _class_gc_desc_missing
    .quad _class_gc_desc_missing
    .quad _class_gc_desc_54
    .quad _class_gc_desc_missing
    .quad _class_gc_desc_missing
    .quad _class_gc_desc_missing
    .quad _class_gc_desc_missing
    .quad _class_gc_desc_missing
    .quad _class_gc_desc_60
    .quad _class_gc_desc_missing
    .quad _class_gc_desc_missing
    .quad _class_gc_desc_missing
    .quad _class_gc_desc_missing
    .quad _class_gc_desc_missing
    .quad _class_gc_desc_missing
    .quad _class_gc_desc_missing
    .quad _class_gc_desc_missing
    .quad _class_gc_desc_missing
    .quad _class_gc_desc_missing
    .quad _class_gc_desc_missing
    .quad _class_gc_desc_missing
    .quad _class_gc_desc_missing
    .quad _class_gc_desc_missing
    .quad _class_gc_desc_75
    .quad _class_gc_desc_missing
    .quad _class_gc_desc_missing
    .quad _class_gc_desc_78
    .quad _class_gc_desc_missing
    .quad _class_gc_desc_80
    .quad _class_gc_desc_missing
    .quad _class_gc_desc_missing
    .quad _class_gc_desc_missing
    .quad _class_gc_desc_missing
    .quad _class_gc_desc_missing
    .quad _class_gc_desc_missing
    .quad _class_gc_desc_missing
    .quad _class_gc_desc_88
    .quad _class_gc_desc_89
.globl _class_vtable_ptrs
_class_vtable_ptrs:
    .quad _class_vtable_0
    .quad _class_vtable_missing
    .quad _class_vtable_missing
    .quad _class_vtable_missing
    .quad _class_vtable_missing
    .quad _class_vtable_missing
    .quad _class_vtable_6
    .quad _class_vtable_missing
    .quad _class_vtable_missing
    .quad _class_vtable_missing
    .quad _class_vtable_missing
    .quad _class_vtable_missing
    .quad _class_vtable_missing
    .quad _class_vtable_13
    .quad _class_vtable_14
    .quad _class_vtable_missing
    .quad _class_vtable_missing
    .quad _class_vtable_missing
    .quad _class_vtable_18
    .quad _class_vtable_missing
    .quad _class_vtable_missing
    .quad _class_vtable_missing
    .quad _class_vtable_missing
    .quad _class_vtable_missing
    .quad _class_vtable_24
    .quad _class_vtable_missing
    .quad _class_vtable_missing
    .quad _class_vtable_missing
    .quad _class_vtable_missing
    .quad _class_vtable_missing
    .quad _class_vtable_30
    .quad _class_vtable_missing
    .quad _class_vtable_missing
    .quad _class_vtable_missing
    .quad _class_vtable_missing
    .quad _class_vtable_missing
    .quad _class_vtable_missing
    .quad _class_vtable_37
    .quad _class_vtable_missing
    .quad _class_vtable_missing
    .quad _class_vtable_40
    .quad _class_vtable_41
    .quad _class_vtable_42
    .quad _class_vtable_missing
    .quad _class_vtable_missing
    .quad _class_vtable_missing
    .quad _class_vtable_46
    .quad _class_vtable_47
    .quad _class_vtable_missing
    .quad _class_vtable_missing
    .quad _class_vtable_missing
    .quad _class_vtable_missing
    .quad _class_vtable_missing
    .quad _class_vtable_missing
    .quad _class_vtable_54
    .quad _class_vtable_missing
    .quad _class_vtable_missing
    .quad _class_vtable_missing
    .quad _class_vtable_missing
    .quad _class_vtable_missing
    .quad _class_vtable_60
    .quad _class_vtable_missing
    .quad _class_vtable_missing
    .quad _class_vtable_missing
    .quad _class_vtable_missing
    .quad _class_vtable_missing
    .quad _class_vtable_missing
    .quad _class_vtable_missing
    .quad _class_vtable_missing
    .quad _class_vtable_missing
    .quad _class_vtable_missing
    .quad _class_vtable_missing
    .quad _class_vtable_missing
    .quad _class_vtable_missing
    .quad _class_vtable_missing
    .quad _class_vtable_75
    .quad _class_vtable_missing
    .quad _class_vtable_missing
    .quad _class_vtable_78
    .quad _class_vtable_missing
    .quad _class_vtable_80
    .quad _class_vtable_missing
    .quad _class_vtable_missing
    .quad _class_vtable_missing
    .quad _class_vtable_missing
    .quad _class_vtable_missing
    .quad _class_vtable_missing
    .quad _class_vtable_missing
    .quad _class_vtable_88
    .quad _class_vtable_89
.globl _class_destruct_count
_class_destruct_count:
    .quad 90
.globl _class_destruct_ptrs
_class_destruct_ptrs:
    .quad 0
    .quad 0
    .quad 0
    .quad 0
    .quad 0
    .quad 0
    .quad 0
    .quad 0
    .quad 0
    .quad 0
    .quad 0
    .quad 0
    .quad 0
    .quad 0
    .quad 0
    .quad 0
    .quad 0
    .quad 0
    .quad 0
    .quad 0
    .quad 0
    .quad 0
    .quad 0
    .quad 0
    .quad 0
    .quad 0
    .quad 0
    .quad 0
    .quad 0
    .quad 0
    .quad 0
    .quad 0
    .quad 0
    .quad 0
    .quad 0
    .quad 0
    .quad 0
    .quad 0
    .quad 0
    .quad 0
    .quad 0
    .quad 0
    .quad 0
    .quad 0
    .quad 0
    .quad 0
    .quad 0
    .quad 0
    .quad 0
    .quad 0
    .quad 0
    .quad 0
    .quad 0
    .quad 0
    .quad 0
    .quad 0
    .quad 0
    .quad 0
    .quad 0
    .quad 0
    .quad 0
    .quad 0
    .quad 0
    .quad 0
    .quad 0
    .quad 0
    .quad 0
    .quad 0
    .quad 0
    .quad 0
    .quad 0
    .quad 0
    .quad 0
    .quad 0
    .quad 0
    .quad 0
    .quad 0
    .quad 0
    .quad 0
    .quad 0
    .quad 0
    .quad 0
    .quad 0
    .quad 0
    .quad 0
    .quad 0
    .quad 0
    .quad 0
    .quad 0
    .quad 0
.globl _class_propinit_ptrs
_class_propinit_ptrs:
    .quad _class_propinit_0
    .quad 0
    .quad 0
    .quad 0
    .quad 0
    .quad 0
    .quad _class_propinit_6
    .quad 0
    .quad 0
    .quad 0
    .quad 0
    .quad 0
    .quad 0
    .quad _class_propinit_13
    .quad _class_propinit_14
    .quad 0
    .quad 0
    .quad 0
    .quad _class_propinit_18
    .quad 0
    .quad 0
    .quad 0
    .quad 0
    .quad 0
    .quad _class_propinit_24
    .quad 0
    .quad 0
    .quad 0
    .quad 0
    .quad 0
    .quad 0
    .quad 0
    .quad 0
    .quad 0
    .quad 0
    .quad 0
    .quad 0
    .quad _class_propinit_37
    .quad 0
    .quad 0
    .quad _class_propinit_40
    .quad _class_propinit_41
    .quad _class_propinit_42
    .quad 0
    .quad 0
    .quad 0
    .quad _class_propinit_46
    .quad _class_propinit_47
    .quad 0
    .quad 0
    .quad 0
    .quad 0
    .quad 0
    .quad 0
    .quad _class_propinit_54
    .quad 0
    .quad 0
    .quad 0
    .quad 0
    .quad 0
    .quad _class_propinit_60
    .quad 0
    .quad 0
    .quad 0
    .quad 0
    .quad 0
    .quad 0
    .quad 0
    .quad 0
    .quad 0
    .quad 0
    .quad 0
    .quad 0
    .quad 0
    .quad 0
    .quad _class_propinit_75
    .quad 0
    .quad 0
    .quad _class_propinit_78
    .quad 0
    .quad _class_propinit_80
    .quad 0
    .quad 0
    .quad 0
    .quad 0
    .quad 0
    .quad 0
    .quad 0
    .quad 0
    .quad 0
.globl _class_static_vtable_ptrs
_class_static_vtable_ptrs:
    .quad _class_static_vtable_0
    .quad _class_static_vtable_missing
    .quad _class_static_vtable_missing
    .quad _class_static_vtable_missing
    .quad _class_static_vtable_missing
    .quad _class_static_vtable_missing
    .quad _class_static_vtable_6
    .quad _class_static_vtable_missing
    .quad _class_static_vtable_missing
    .quad _class_static_vtable_missing
    .quad _class_static_vtable_missing
    .quad _class_static_vtable_missing
    .quad _class_static_vtable_missing
    .quad _class_static_vtable_13
    .quad _class_static_vtable_14
    .quad _class_static_vtable_missing
    .quad _class_static_vtable_missing
    .quad _class_static_vtable_missing
    .quad _class_static_vtable_18
    .quad _class_static_vtable_missing
    .quad _class_static_vtable_missing
    .quad _class_static_vtable_missing
    .quad _class_static_vtable_missing
    .quad _class_static_vtable_missing
    .quad _class_static_vtable_24
    .quad _class_static_vtable_missing
    .quad _class_static_vtable_missing
    .quad _class_static_vtable_missing
    .quad _class_static_vtable_missing
    .quad _class_static_vtable_missing
    .quad _class_static_vtable_30
    .quad _class_static_vtable_missing
    .quad _class_static_vtable_missing
    .quad _class_static_vtable_missing
    .quad _class_static_vtable_missing
    .quad _class_static_vtable_missing
    .quad _class_static_vtable_missing
    .quad _class_static_vtable_37
    .quad _class_static_vtable_missing
    .quad _class_static_vtable_missing
    .quad _class_static_vtable_40
    .quad _class_static_vtable_41
    .quad _class_static_vtable_42
    .quad _class_static_vtable_missing
    .quad _class_static_vtable_missing
    .quad _class_static_vtable_missing
    .quad _class_static_vtable_46
    .quad _class_static_vtable_47
    .quad _class_static_vtable_missing
    .quad _class_static_vtable_missing
    .quad _class_static_vtable_missing
    .quad _class_static_vtable_missing
    .quad _class_static_vtable_missing
    .quad _class_static_vtable_missing
    .quad _class_static_vtable_54
    .quad _class_static_vtable_missing
    .quad _class_static_vtable_missing
    .quad _class_static_vtable_missing
    .quad _class_static_vtable_missing
    .quad _class_static_vtable_missing
    .quad _class_static_vtable_60
    .quad _class_static_vtable_missing
    .quad _class_static_vtable_missing
    .quad _class_static_vtable_missing
    .quad _class_static_vtable_missing
    .quad _class_static_vtable_missing
    .quad _class_static_vtable_missing
    .quad _class_static_vtable_missing
    .quad _class_static_vtable_missing
    .quad _class_static_vtable_missing
    .quad _class_static_vtable_missing
    .quad _class_static_vtable_missing
    .quad _class_static_vtable_missing
    .quad _class_static_vtable_missing
    .quad _class_static_vtable_missing
    .quad _class_static_vtable_75
    .quad _class_static_vtable_missing
    .quad _class_static_vtable_missing
    .quad _class_static_vtable_78
    .quad _class_static_vtable_missing
    .quad _class_static_vtable_80
    .quad _class_static_vtable_missing
    .quad _class_static_vtable_missing
    .quad _class_static_vtable_missing
    .quad _class_static_vtable_missing
    .quad _class_static_vtable_missing
    .quad _class_static_vtable_missing
    .quad _class_static_vtable_missing
    .quad _class_static_vtable_88
    .quad _class_static_vtable_89
.globl _class_callable_method_ptrs
_class_callable_method_ptrs:
    .quad _class_callable_methods_0
    .quad _class_callable_methods_missing
    .quad _class_callable_methods_missing
    .quad _class_callable_methods_missing
    .quad _class_callable_methods_missing
    .quad _class_callable_methods_missing
    .quad _class_callable_methods_6
    .quad _class_callable_methods_missing
    .quad _class_callable_methods_missing
    .quad _class_callable_methods_missing
    .quad _class_callable_methods_missing
    .quad _class_callable_methods_missing
    .quad _class_callable_methods_missing
    .quad _class_callable_methods_13
    .quad _class_callable_methods_14
    .quad _class_callable_methods_missing
    .quad _class_callable_methods_missing
    .quad _class_callable_methods_missing
    .quad _class_callable_methods_18
    .quad _class_callable_methods_missing
    .quad _class_callable_methods_missing
    .quad _class_callable_methods_missing
    .quad _class_callable_methods_missing
    .quad _class_callable_methods_missing
    .quad _class_callable_methods_24
    .quad _class_callable_methods_missing
    .quad _class_callable_methods_missing
    .quad _class_callable_methods_missing
    .quad _class_callable_methods_missing
    .quad _class_callable_methods_missing
    .quad _class_callable_methods_30
    .quad _class_callable_methods_missing
    .quad _class_callable_methods_missing
    .quad _class_callable_methods_missing
    .quad _class_callable_methods_missing
    .quad _class_callable_methods_missing
    .quad _class_callable_methods_missing
    .quad _class_callable_methods_37
    .quad _class_callable_methods_missing
    .quad _class_callable_methods_missing
    .quad _class_callable_methods_40
    .quad _class_callable_methods_41
    .quad _class_callable_methods_42
    .quad _class_callable_methods_missing
    .quad _class_callable_methods_missing
    .quad _class_callable_methods_missing
    .quad _class_callable_methods_46
    .quad _class_callable_methods_47
    .quad _class_callable_methods_missing
    .quad _class_callable_methods_missing
    .quad _class_callable_methods_missing
    .quad _class_callable_methods_missing
    .quad _class_callable_methods_missing
    .quad _class_callable_methods_missing
    .quad _class_callable_methods_54
    .quad _class_callable_methods_missing
    .quad _class_callable_methods_missing
    .quad _class_callable_methods_missing
    .quad _class_callable_methods_missing
    .quad _class_callable_methods_missing
    .quad _class_callable_methods_60
    .quad _class_callable_methods_missing
    .quad _class_callable_methods_missing
    .quad _class_callable_methods_missing
    .quad _class_callable_methods_missing
    .quad _class_callable_methods_missing
    .quad _class_callable_methods_missing
    .quad _class_callable_methods_missing
    .quad _class_callable_methods_missing
    .quad _class_callable_methods_missing
    .quad _class_callable_methods_missing
    .quad _class_callable_methods_missing
    .quad _class_callable_methods_missing
    .quad _class_callable_methods_missing
    .quad _class_callable_methods_missing
    .quad _class_callable_methods_75
    .quad _class_callable_methods_missing
    .quad _class_callable_methods_missing
    .quad _class_callable_methods_78
    .quad _class_callable_methods_missing
    .quad _class_callable_methods_80
    .quad _class_callable_methods_missing
    .quad _class_callable_methods_missing
    .quad _class_callable_methods_missing
    .quad _class_callable_methods_missing
    .quad _class_callable_methods_missing
    .quad _class_callable_methods_missing
    .quad _class_callable_methods_missing
    .quad _class_callable_methods_88
    .quad _class_callable_methods_89
.p2align 3
.globl _user_wrapper_vtable_ptrs
_user_wrapper_vtable_ptrs:
    .quad _user_wrapper_vtable_missing
    .quad _user_wrapper_vtable_missing
    .quad _user_wrapper_vtable_missing
    .quad _user_wrapper_vtable_missing
    .quad _user_wrapper_vtable_missing
    .quad _user_wrapper_vtable_missing
    .quad _user_wrapper_vtable_missing
    .quad _user_wrapper_vtable_missing
    .quad _user_wrapper_vtable_missing
    .quad _user_wrapper_vtable_missing
    .quad _user_wrapper_vtable_missing
    .quad _user_wrapper_vtable_missing
    .quad _user_wrapper_vtable_missing
    .quad _user_wrapper_vtable_missing
    .quad _user_wrapper_vtable_missing
    .quad _user_wrapper_vtable_missing
    .quad _user_wrapper_vtable_missing
    .quad _user_wrapper_vtable_missing
    .quad _user_wrapper_vtable_missing
    .quad _user_wrapper_vtable_missing
    .quad _user_wrapper_vtable_missing
    .quad _user_wrapper_vtable_missing
    .quad _user_wrapper_vtable_missing
    .quad _user_wrapper_vtable_missing
    .quad _user_wrapper_vtable_missing
    .quad _user_wrapper_vtable_missing
    .quad _user_wrapper_vtable_missing
    .quad _user_wrapper_vtable_missing
    .quad _user_wrapper_vtable_missing
    .quad _user_wrapper_vtable_missing
    .quad _user_wrapper_vtable_missing
    .quad _user_wrapper_vtable_missing
    .quad _user_wrapper_vtable_missing
    .quad _user_wrapper_vtable_missing
    .quad _user_wrapper_vtable_missing
    .quad _user_wrapper_vtable_missing
    .quad _user_wrapper_vtable_missing
    .quad _user_wrapper_vtable_missing
    .quad _user_wrapper_vtable_missing
    .quad _user_wrapper_vtable_missing
    .quad _user_wrapper_vtable_missing
    .quad _user_wrapper_vtable_missing
    .quad _user_wrapper_vtable_missing
    .quad _user_wrapper_vtable_missing
    .quad _user_wrapper_vtable_missing
    .quad _user_wrapper_vtable_missing
    .quad _user_wrapper_vtable_missing
    .quad _user_wrapper_vtable_missing
    .quad _user_wrapper_vtable_missing
    .quad _user_wrapper_vtable_missing
    .quad _user_wrapper_vtable_missing
    .quad _user_wrapper_vtable_missing
    .quad _user_wrapper_vtable_missing
    .quad _user_wrapper_vtable_missing
    .quad _user_wrapper_vtable_missing
    .quad _user_wrapper_vtable_missing
    .quad _user_wrapper_vtable_missing
    .quad _user_wrapper_vtable_missing
    .quad _user_wrapper_vtable_missing
    .quad _user_wrapper_vtable_missing
    .quad _user_wrapper_vtable_missing
    .quad _user_wrapper_vtable_missing
    .quad _user_wrapper_vtable_missing
    .quad _user_wrapper_vtable_missing
    .quad _user_wrapper_vtable_missing
    .quad _user_wrapper_vtable_missing
    .quad _user_wrapper_vtable_missing
    .quad _user_wrapper_vtable_missing
    .quad _user_wrapper_vtable_missing
    .quad _user_wrapper_vtable_missing
    .quad _user_wrapper_vtable_missing
    .quad _user_wrapper_vtable_missing
    .quad _user_wrapper_vtable_missing
    .quad _user_wrapper_vtable_missing
    .quad _user_wrapper_vtable_missing
    .quad _user_wrapper_vtable_missing
    .quad _user_wrapper_vtable_missing
    .quad _user_wrapper_vtable_missing
    .quad _user_wrapper_vtable_missing
    .quad _user_wrapper_vtable_missing
    .quad _user_wrapper_vtable_missing
    .quad _user_wrapper_vtable_missing
    .quad _user_wrapper_vtable_missing
    .quad _user_wrapper_vtable_missing
    .quad _user_wrapper_vtable_missing
    .quad _user_wrapper_vtable_missing
    .quad _user_wrapper_vtable_missing
    .quad _user_wrapper_vtable_missing
    .quad _user_wrapper_vtable_missing
    .quad _user_wrapper_vtable_missing
.p2align 3
.globl _user_filter_vtable_ptrs
_user_filter_vtable_ptrs:
    .quad _user_filter_vtable_missing
    .quad _user_filter_vtable_missing
    .quad _user_filter_vtable_missing
    .quad _user_filter_vtable_missing
    .quad _user_filter_vtable_missing
    .quad _user_filter_vtable_missing
    .quad _user_filter_vtable_missing
    .quad _user_filter_vtable_missing
    .quad _user_filter_vtable_missing
    .quad _user_filter_vtable_missing
    .quad _user_filter_vtable_missing
    .quad _user_filter_vtable_missing
    .quad _user_filter_vtable_missing
    .quad _user_filter_vtable_missing
    .quad _user_filter_vtable_missing
    .quad _user_filter_vtable_missing
    .quad _user_filter_vtable_missing
    .quad _user_filter_vtable_missing
    .quad _user_filter_vtable_missing
    .quad _user_filter_vtable_missing
    .quad _user_filter_vtable_missing
    .quad _user_filter_vtable_missing
    .quad _user_filter_vtable_missing
    .quad _user_filter_vtable_missing
    .quad _user_filter_vtable_missing
    .quad _user_filter_vtable_missing
    .quad _user_filter_vtable_missing
    .quad _user_filter_vtable_missing
    .quad _user_filter_vtable_missing
    .quad _user_filter_vtable_missing
    .quad _user_filter_vtable_missing
    .quad _user_filter_vtable_missing
    .quad _user_filter_vtable_missing
    .quad _user_filter_vtable_missing
    .quad _user_filter_vtable_missing
    .quad _user_filter_vtable_missing
    .quad _user_filter_vtable_missing
    .quad _user_filter_vtable_missing
    .quad _user_filter_vtable_missing
    .quad _user_filter_vtable_missing
    .quad _user_filter_vtable_missing
    .quad _user_filter_vtable_missing
    .quad _user_filter_vtable_missing
    .quad _user_filter_vtable_missing
    .quad _user_filter_vtable_missing
    .quad _user_filter_vtable_missing
    .quad _user_filter_vtable_missing
    .quad _user_filter_vtable_missing
    .quad _user_filter_vtable_missing
    .quad _user_filter_vtable_missing
    .quad _user_filter_vtable_missing
    .quad _user_filter_vtable_missing
    .quad _user_filter_vtable_missing
    .quad _user_filter_vtable_missing
    .quad _user_filter_vtable_missing
    .quad _user_filter_vtable_missing
    .quad _user_filter_vtable_missing
    .quad _user_filter_vtable_missing
    .quad _user_filter_vtable_missing
    .quad _user_filter_vtable_missing
    .quad _user_filter_vtable_missing
    .quad _user_filter_vtable_missing
    .quad _user_filter_vtable_missing
    .quad _user_filter_vtable_missing
    .quad _user_filter_vtable_missing
    .quad _user_filter_vtable_missing
    .quad _user_filter_vtable_missing
    .quad _user_filter_vtable_missing
    .quad _user_filter_vtable_missing
    .quad _user_filter_vtable_missing
    .quad _user_filter_vtable_missing
    .quad _user_filter_vtable_missing
    .quad _user_filter_vtable_missing
    .quad _user_filter_vtable_missing
    .quad _user_filter_vtable_missing
    .quad _user_filter_vtable_missing
    .quad _user_filter_vtable_missing
    .quad _user_filter_vtable_missing
    .quad _user_filter_vtable_missing
    .quad _user_filter_vtable_missing
    .quad _user_filter_vtable_missing
    .quad _user_filter_vtable_missing
    .quad _user_filter_vtable_missing
    .quad _user_filter_vtable_missing
    .quad _user_filter_vtable_missing
    .quad _user_filter_vtable_missing
    .quad _user_filter_vtable_missing
    .quad _user_filter_vtable_missing
    .quad _user_filter_vtable_missing
    .quad _user_filter_vtable_missing
.globl _class_interfaces_missing
_class_interfaces_missing:
    .quad 0
.globl _class_gc_desc_missing
_class_gc_desc_missing:
    .byte 0
    .p2align 3
.globl _class_json_desc_missing
_class_json_desc_missing:
    .quad 0
    .quad 0
    .quad 0
    .p2align 3
.globl _class_vtable_missing
_class_vtable_missing:
    .quad 0
    .p2align 3
.globl _class_static_vtable_missing
_class_static_vtable_missing:
    .quad 0
    .p2align 3
.globl _class_callable_methods_missing
_class_callable_methods_missing:
    .quad 0
    .p2align 3
.globl _user_wrapper_vtable_missing
_user_wrapper_vtable_missing:
    .quad 0
    .quad 0
    .quad 0
    .quad 0
    .quad 0
    .quad 0
    .quad 0
    .quad 0
    .quad 0
    .quad 0
    .quad 0
    .quad 0
    .quad 0
    .quad 0
    .quad 0
    .quad 0
    .quad 0
    .quad 0
    .quad 0
    .quad 0
    .quad 0
    .quad 0
    .quad 0
    .p2align 3
.globl _user_filter_vtable_missing
_user_filter_vtable_missing:
    .quad 0
    .quad 0
    .quad 0
    .quad 0
    .quad 0
.p2align 3
.globl _class_callable_static_class_name_88
_class_callable_static_class_name_88:
    .ascii "AIC\\Components\\Domain\\HeadAssetType"
.globl _class_callable_static_method_name_88_cases
_class_callable_static_method_name_88_cases:
    .ascii "cases"
.globl _class_callable_static_method_name_88_from
_class_callable_static_method_name_88_from:
    .ascii "from"
.globl _class_callable_static_method_name_88_tryfrom
_class_callable_static_method_name_88_tryfrom:
    .ascii "tryfrom"
.globl _class_callable_static_class_name_89
_class_callable_static_class_name_89:
    .ascii "AIC\\Components\\Domain\\HeadAssetMode"
.globl _class_callable_static_method_name_89_cases
_class_callable_static_method_name_89_cases:
    .ascii "cases"
.globl _class_callable_static_method_name_89_from
_class_callable_static_method_name_89_from:
    .ascii "from"
.globl _class_callable_static_method_name_89_tryfrom
_class_callable_static_method_name_89_tryfrom:
    .ascii "tryfrom"
.p2align 3
.globl _class_callable_static_method_count
_class_callable_static_method_count:
    .quad 6
.globl _class_callable_static_method_table
_class_callable_static_method_table:
    .quad _class_callable_static_class_name_88
    .quad 35
    .quad _class_callable_static_method_name_88_cases
    .quad 5
    .quad _class_callable_static_class_name_88
    .quad 35
    .quad _class_callable_static_method_name_88_from
    .quad 4
    .quad _class_callable_static_class_name_88
    .quad 35
    .quad _class_callable_static_method_name_88_tryfrom
    .quad 7
    .quad _class_callable_static_class_name_89
    .quad 35
    .quad _class_callable_static_method_name_89_cases
    .quad 5
    .quad _class_callable_static_class_name_89
    .quad 35
    .quad _class_callable_static_method_name_89_from
    .quad 4
    .quad _class_callable_static_class_name_89
    .quad 35
    .quad _class_callable_static_method_name_89_tryfrom
    .quad 7
.p2align 3
.globl _class_by_name_str_0
_class_by_name_str_0:
    .ascii "Exception"
.globl _class_by_name_str_6
_class_by_name_str_6:
    .ascii "ReflectionParameter"
.globl _class_by_name_str_13
_class_by_name_str_13:
    .ascii "ReflectionFunction"
.globl _class_by_name_str_14
_class_by_name_str_14:
    .ascii "Error"
.globl _class_by_name_str_18
_class_by_name_str_18:
    .ascii "ReflectionMethod"
.globl _class_by_name_str_24
_class_by_name_str_24:
    .ascii "ReflectionNamedType"
.globl _class_by_name_str_30
_class_by_name_str_30:
    .ascii "AIC\\Components\\Domain\\HeadAsset"
.globl _class_by_name_str_37
_class_by_name_str_37:
    .ascii "TypeError"
.globl _class_by_name_str_40
_class_by_name_str_40:
    .ascii "ReflectionProperty"
.globl _class_by_name_str_41
_class_by_name_str_41:
    .ascii "LogicException"
.globl _class_by_name_str_42
_class_by_name_str_42:
    .ascii "OutOfRangeException"
.globl _class_by_name_str_46
_class_by_name_str_46:
    .ascii "RuntimeException"
.globl _class_by_name_str_47
_class_by_name_str_47:
    .ascii "JsonException"
.globl _class_by_name_str_54
_class_by_name_str_54:
    .ascii "ReflectionClass"
.globl _class_by_name_str_60
_class_by_name_str_60:
    .ascii "ReflectionAttribute"
.globl _class_by_name_str_75
_class_by_name_str_75:
    .ascii "OutOfBoundsException"
.globl _class_by_name_str_78
_class_by_name_str_78:
    .ascii "ValueError"
.globl _class_by_name_str_80
_class_by_name_str_80:
    .ascii "InvalidArgumentException"
.globl _class_by_name_str_88
_class_by_name_str_88:
    .ascii "AIC\\Components\\Domain\\HeadAssetType"
.globl _class_by_name_str_89
_class_by_name_str_89:
    .ascii "AIC\\Components\\Domain\\HeadAssetMode"
.p2align 3
.globl _classes_by_name_count
_classes_by_name_count:
    .quad 20
.globl _classes_by_name
_classes_by_name:
    .quad _class_by_name_str_0
    .quad 9
    .quad 0
    .quad 40
    .quad _class_by_name_str_6
    .quad 19
    .quad 6
    .quad 104
    .quad _class_by_name_str_13
    .quad 18
    .quad 13
    .quad 88
    .quad _class_by_name_str_14
    .quad 5
    .quad 14
    .quad 40
    .quad _class_by_name_str_18
    .quad 16
    .quad 18
    .quad 24
    .quad _class_by_name_str_24
    .quad 19
    .quad 24
    .quad 56
    .quad _class_by_name_str_30
    .quad 31
    .quad 30
    .quad 88
    .quad _class_by_name_str_37
    .quad 9
    .quad 37
    .quad 40
    .quad _class_by_name_str_40
    .quad 18
    .quad 40
    .quad 24
    .quad _class_by_name_str_41
    .quad 14
    .quad 41
    .quad 40
    .quad _class_by_name_str_42
    .quad 19
    .quad 42
    .quad 40
    .quad _class_by_name_str_46
    .quad 16
    .quad 46
    .quad 40
    .quad _class_by_name_str_47
    .quad 13
    .quad 47
    .quad 40
    .quad _class_by_name_str_54
    .quad 15
    .quad 54
    .quad 40
    .quad _class_by_name_str_60
    .quad 19
    .quad 60
    .quad 56
    .quad _class_by_name_str_75
    .quad 20
    .quad 75
    .quad 40
    .quad _class_by_name_str_78
    .quad 10
    .quad 78
    .quad 40
    .quad _class_by_name_str_80
    .quad 24
    .quad 80
    .quad 40
    .quad _class_by_name_str_88
    .quad 35
    .quad 88
    .quad 40
    .quad _class_by_name_str_89
    .quad 35
    .quad 89
    .quad 40
.p2align 3
.globl _class_attribute_count
_class_attribute_count:
    .quad 90
.globl _class_attribute_ptrs
_class_attribute_ptrs:
    .quad _class_attributes_missing
    .quad _class_attributes_missing
    .quad _class_attributes_missing
    .quad _class_attributes_missing
    .quad _class_attributes_missing
    .quad _class_attributes_missing
    .quad _class_attributes_missing
    .quad _class_attributes_missing
    .quad _class_attributes_missing
    .quad _class_attributes_missing
    .quad _class_attributes_missing
    .quad _class_attributes_missing
    .quad _class_attributes_missing
    .quad _class_attributes_missing
    .quad _class_attributes_missing
    .quad _class_attributes_missing
    .quad _class_attributes_missing
    .quad _class_attributes_missing
    .quad _class_attributes_missing
    .quad _class_attributes_missing
    .quad _class_attributes_missing
    .quad _class_attributes_missing
    .quad _class_attributes_missing
    .quad _class_attributes_missing
    .quad _class_attributes_missing
    .quad _class_attributes_missing
    .quad _class_attributes_missing
    .quad _class_attributes_missing
    .quad _class_attributes_missing
    .quad _class_attributes_missing
    .quad _class_attributes_missing
    .quad _class_attributes_missing
    .quad _class_attributes_missing
    .quad _class_attributes_missing
    .quad _class_attributes_missing
    .quad _class_attributes_missing
    .quad _class_attributes_missing
    .quad _class_attributes_missing
    .quad _class_attributes_missing
    .quad _class_attributes_missing
    .quad _class_attributes_missing
    .quad _class_attributes_missing
    .quad _class_attributes_missing
    .quad _class_attributes_missing
    .quad _class_attributes_missing
    .quad _class_attributes_missing
    .quad _class_attributes_missing
    .quad _class_attributes_missing
    .quad _class_attributes_missing
    .quad _class_attributes_missing
    .quad _class_attributes_missing
    .quad _class_attributes_missing
    .quad _class_attributes_missing
    .quad _class_attributes_missing
    .quad _class_attributes_missing
    .quad _class_attributes_missing
    .quad _class_attributes_missing
    .quad _class_attributes_missing
    .quad _class_attributes_missing
    .quad _class_attributes_missing
    .quad _class_attributes_missing
    .quad _class_attributes_missing
    .quad _class_attributes_missing
    .quad _class_attributes_missing
    .quad _class_attributes_missing
    .quad _class_attributes_missing
    .quad _class_attributes_missing
    .quad _class_attributes_missing
    .quad _class_attributes_missing
    .quad _class_attributes_missing
    .quad _class_attributes_missing
    .quad _class_attributes_missing
    .quad _class_attributes_missing
    .quad _class_attributes_missing
    .quad _class_attributes_missing
    .quad _class_attributes_missing
    .quad _class_attributes_missing
    .quad _class_attributes_missing
    .quad _class_attributes_missing
    .quad _class_attributes_missing
    .quad _class_attributes_missing
    .quad _class_attributes_missing
    .quad _class_attributes_missing
    .quad _class_attributes_missing
    .quad _class_attributes_missing
    .quad _class_attributes_missing
    .quad _class_attributes_missing
    .quad _class_attributes_missing
    .quad _class_attributes_missing
    .quad _class_attributes_missing
.globl _class_attributes_missing
_class_attributes_missing:
    .quad 0
.globl _interface_methods_8
_interface_methods_8:
    .quad 8
    .quad 0
    .quad 1
    .quad 2
    .quad 3
    .quad 4
    .quad 5
    .quad 6
    .quad 7
.globl _interface_methods_12
_interface_methods_12:
    .quad 1
    .quad 0
.globl _class_interfaces_0
_class_interfaces_0:
    .quad 2
    .quad 8
    .quad _class_interface_impl_0_8
    .quad 12
    .quad _class_interface_impl_0_12
.globl _class_interface_impl_0_8
_class_interface_impl_0_8:
    .quad 0
    .quad 0
    .quad 0
    .quad 0
    .quad 0
    .quad 0
    .quad 0
    .quad 0
.globl _class_interface_impl_0_12
_class_interface_impl_0_12:
    .quad 0
.globl _class_json_pname_0_0
_class_json_pname_0_0:
    .ascii "message"
    .p2align 3
.globl _class_json_desc_0
_class_json_desc_0:
    .quad 0
    .quad 0
    .quad 1
    .quad _class_json_pname_0_0
    .quad 7
    .quad 0
    .quad 1
    .p2align 3
.globl _class_gc_desc_0
_class_gc_desc_0:
    .byte 1, 0
    .p2align 3
.globl _class_vtable_0
_class_vtable_0:
    .quad 0
    .quad 0
    .quad 0
    .quad 0
    .quad 0
    .quad 0
    .quad 0
    .quad 0
    .quad 0
    .p2align 3
.globl _class_static_vtable_0
_class_static_vtable_0:
    .quad 0
.globl _class_callable_method_name_0__u__u_construct
_class_callable_method_name_0__u__u_construct:
    .ascii "__construct"
.globl _class_callable_method_name_0__u__u_tostring
_class_callable_method_name_0__u__u_tostring:
    .ascii "__tostring"
.globl _class_callable_method_name_0_getcode
_class_callable_method_name_0_getcode:
    .ascii "getcode"
.globl _class_callable_method_name_0_getfile
_class_callable_method_name_0_getfile:
    .ascii "getfile"
.globl _class_callable_method_name_0_getline
_class_callable_method_name_0_getline:
    .ascii "getline"
.globl _class_callable_method_name_0_getmessage
_class_callable_method_name_0_getmessage:
    .ascii "getmessage"
.globl _class_callable_method_name_0_getprevious
_class_callable_method_name_0_getprevious:
    .ascii "getprevious"
.globl _class_callable_method_name_0_gettrace
_class_callable_method_name_0_gettrace:
    .ascii "gettrace"
.globl _class_callable_method_name_0_gettraceasstring
_class_callable_method_name_0_gettraceasstring:
    .ascii "gettraceasstring"
.p2align 3
.globl _class_callable_methods_0
_class_callable_methods_0:
    .quad 9
    .quad _class_callable_method_name_0__u__u_construct
    .quad 11
    .quad _class_callable_method_name_0__u__u_tostring
    .quad 10
    .quad _class_callable_method_name_0_getcode
    .quad 7
    .quad _class_callable_method_name_0_getfile
    .quad 7
    .quad _class_callable_method_name_0_getline
    .quad 7
    .quad _class_callable_method_name_0_getmessage
    .quad 10
    .quad _class_callable_method_name_0_getprevious
    .quad 11
    .quad _class_callable_method_name_0_gettrace
    .quad 8
    .quad _class_callable_method_name_0_gettraceasstring
    .quad 16
.globl _class_interfaces_6
_class_interfaces_6:
    .quad 0
    .p2align 3
.globl _class_json_desc_6
_class_json_desc_6:
    .quad 0
    .quad 0
    .quad 0
    .p2align 3
.globl _class_gc_desc_6
_class_gc_desc_6:
    .byte 1, 0, 3, 3, 3, 7
    .p2align 3
.globl _class_vtable_6
_class_vtable_6:
    .quad _method_ReflectionParameter_getname
    .quad _method_ReflectionParameter_getposition
    .quad _method_ReflectionParameter_isoptional
    .quad _method_ReflectionParameter_isvariadic
    .quad _method_ReflectionParameter_hastype
    .quad _method_ReflectionParameter_gettype
    .p2align 3
.globl _class_static_vtable_6
_class_static_vtable_6:
    .quad 0
.globl _class_callable_method_name_6_getname
_class_callable_method_name_6_getname:
    .ascii "getname"
.globl _class_callable_method_name_6_getposition
_class_callable_method_name_6_getposition:
    .ascii "getposition"
.globl _class_callable_method_name_6_gettype
_class_callable_method_name_6_gettype:
    .ascii "gettype"
.globl _class_callable_method_name_6_hastype
_class_callable_method_name_6_hastype:
    .ascii "hastype"
.globl _class_callable_method_name_6_isoptional
_class_callable_method_name_6_isoptional:
    .ascii "isoptional"
.globl _class_callable_method_name_6_isvariadic
_class_callable_method_name_6_isvariadic:
    .ascii "isvariadic"
.p2align 3
.globl _class_callable_methods_6
_class_callable_methods_6:
    .quad 6
    .quad _class_callable_method_name_6_getname
    .quad 7
    .quad _class_callable_method_name_6_getposition
    .quad 11
    .quad _class_callable_method_name_6_gettype
    .quad 7
    .quad _class_callable_method_name_6_hastype
    .quad 7
    .quad _class_callable_method_name_6_isoptional
    .quad 10
    .quad _class_callable_method_name_6_isvariadic
    .quad 10
.globl _class_interfaces_13
_class_interfaces_13:
    .quad 0
    .p2align 3
.globl _class_json_desc_13
_class_json_desc_13:
    .quad 0
    .quad 0
    .quad 0
    .p2align 3
.globl _class_gc_desc_13
_class_gc_desc_13:
    .byte 1, 1, 0, 0, 4
    .p2align 3
.globl _class_vtable_13
_class_vtable_13:
    .quad _method_ReflectionFunction__u__u_construct
    .quad _method_ReflectionFunction_getname
    .quad _method_ReflectionFunction_getshortname
    .quad _method_ReflectionFunction_getnumberofparameters
    .quad _method_ReflectionFunction_getnumberofrequiredparameters
    .quad _method_ReflectionFunction_getparameters
    .p2align 3
.globl _class_static_vtable_13
_class_static_vtable_13:
    .quad 0
.globl _class_callable_method_name_13__u__u_construct
_class_callable_method_name_13__u__u_construct:
    .ascii "__construct"
.globl _class_callable_method_name_13_getname
_class_callable_method_name_13_getname:
    .ascii "getname"
.globl _class_callable_method_name_13_getnumberofparameters
_class_callable_method_name_13_getnumberofparameters:
    .ascii "getnumberofparameters"
.globl _class_callable_method_name_13_getnumberofrequiredparameters
_class_callable_method_name_13_getnumberofrequiredparameters:
    .ascii "getnumberofrequiredparameters"
.globl _class_callable_method_name_13_getparameters
_class_callable_method_name_13_getparameters:
    .ascii "getparameters"
.globl _class_callable_method_name_13_getshortname
_class_callable_method_name_13_getshortname:
    .ascii "getshortname"
.p2align 3
.globl _class_callable_methods_13
_class_callable_methods_13:
    .quad 6
    .quad _class_callable_method_name_13__u__u_construct
    .quad 11
    .quad _class_callable_method_name_13_getname
    .quad 7
    .quad _class_callable_method_name_13_getnumberofparameters
    .quad 21
    .quad _class_callable_method_name_13_getnumberofrequiredparameters
    .quad 29
    .quad _class_callable_method_name_13_getparameters
    .quad 13
    .quad _class_callable_method_name_13_getshortname
    .quad 12
.globl _class_interfaces_14
_class_interfaces_14:
    .quad 2
    .quad 8
    .quad _class_interface_impl_14_8
    .quad 12
    .quad _class_interface_impl_14_12
.globl _class_interface_impl_14_8
_class_interface_impl_14_8:
    .quad 0
    .quad 0
    .quad 0
    .quad 0
    .quad 0
    .quad 0
    .quad 0
    .quad 0
.globl _class_interface_impl_14_12
_class_interface_impl_14_12:
    .quad 0
.globl _class_json_pname_14_0
_class_json_pname_14_0:
    .ascii "message"
    .p2align 3
.globl _class_json_desc_14
_class_json_desc_14:
    .quad 0
    .quad 0
    .quad 1
    .quad _class_json_pname_14_0
    .quad 7
    .quad 0
    .quad 1
    .p2align 3
.globl _class_gc_desc_14
_class_gc_desc_14:
    .byte 1, 0
    .p2align 3
.globl _class_vtable_14
_class_vtable_14:
    .quad 0
    .quad 0
    .quad 0
    .quad 0
    .quad 0
    .quad 0
    .quad 0
    .quad 0
    .quad 0
    .p2align 3
.globl _class_static_vtable_14
_class_static_vtable_14:
    .quad 0
.globl _class_callable_method_name_14__u__u_construct
_class_callable_method_name_14__u__u_construct:
    .ascii "__construct"
.globl _class_callable_method_name_14__u__u_tostring
_class_callable_method_name_14__u__u_tostring:
    .ascii "__tostring"
.globl _class_callable_method_name_14_getcode
_class_callable_method_name_14_getcode:
    .ascii "getcode"
.globl _class_callable_method_name_14_getfile
_class_callable_method_name_14_getfile:
    .ascii "getfile"
.globl _class_callable_method_name_14_getline
_class_callable_method_name_14_getline:
    .ascii "getline"
.globl _class_callable_method_name_14_getmessage
_class_callable_method_name_14_getmessage:
    .ascii "getmessage"
.globl _class_callable_method_name_14_getprevious
_class_callable_method_name_14_getprevious:
    .ascii "getprevious"
.globl _class_callable_method_name_14_gettrace
_class_callable_method_name_14_gettrace:
    .ascii "gettrace"
.globl _class_callable_method_name_14_gettraceasstring
_class_callable_method_name_14_gettraceasstring:
    .ascii "gettraceasstring"
.p2align 3
.globl _class_callable_methods_14
_class_callable_methods_14:
    .quad 9
    .quad _class_callable_method_name_14__u__u_construct
    .quad 11
    .quad _class_callable_method_name_14__u__u_tostring
    .quad 10
    .quad _class_callable_method_name_14_getcode
    .quad 7
    .quad _class_callable_method_name_14_getfile
    .quad 7
    .quad _class_callable_method_name_14_getline
    .quad 7
    .quad _class_callable_method_name_14_getmessage
    .quad 10
    .quad _class_callable_method_name_14_getprevious
    .quad 11
    .quad _class_callable_method_name_14_gettrace
    .quad 8
    .quad _class_callable_method_name_14_gettraceasstring
    .quad 16
.globl _class_interfaces_18
_class_interfaces_18:
    .quad 0
    .p2align 3
.globl _class_json_desc_18
_class_json_desc_18:
    .quad 0
    .quad 0
    .quad 0
    .p2align 3
.globl _class_gc_desc_18
_class_gc_desc_18:
    .byte 4
    .p2align 3
.globl _class_vtable_18
_class_vtable_18:
    .quad _method_ReflectionMethod__u__u_construct
    .quad _method_ReflectionMethod_getattributes
    .p2align 3
.globl _class_static_vtable_18
_class_static_vtable_18:
    .quad 0
.globl _class_callable_method_name_18__u__u_construct
_class_callable_method_name_18__u__u_construct:
    .ascii "__construct"
.globl _class_callable_method_name_18_getattributes
_class_callable_method_name_18_getattributes:
    .ascii "getattributes"
.p2align 3
.globl _class_callable_methods_18
_class_callable_methods_18:
    .quad 2
    .quad _class_callable_method_name_18__u__u_construct
    .quad 11
    .quad _class_callable_method_name_18_getattributes
    .quad 13
.globl _class_interfaces_24
_class_interfaces_24:
    .quad 0
    .p2align 3
.globl _class_json_desc_24
_class_json_desc_24:
    .quad 0
    .quad 0
    .quad 0
    .p2align 3
.globl _class_gc_desc_24
_class_gc_desc_24:
    .byte 1, 3, 3
    .p2align 3
.globl _class_vtable_24
_class_vtable_24:
    .quad _method_ReflectionNamedType_getname
    .quad _method_ReflectionNamedType_allowsnull
    .quad _method_ReflectionNamedType_isbuiltin
    .p2align 3
.globl _class_static_vtable_24
_class_static_vtable_24:
    .quad 0
.globl _class_callable_method_name_24_allowsnull
_class_callable_method_name_24_allowsnull:
    .ascii "allowsnull"
.globl _class_callable_method_name_24_getname
_class_callable_method_name_24_getname:
    .ascii "getname"
.globl _class_callable_method_name_24_isbuiltin
_class_callable_method_name_24_isbuiltin:
    .ascii "isbuiltin"
.p2align 3
.globl _class_callable_methods_24
_class_callable_methods_24:
    .quad 3
    .quad _class_callable_method_name_24_allowsnull
    .quad 10
    .quad _class_callable_method_name_24_getname
    .quad 7
    .quad _class_callable_method_name_24_isbuiltin
    .quad 9
.globl _class_interfaces_30
_class_interfaces_30:
    .quad 0
.globl _class_json_pname_30_0
_class_json_pname_30_0:
    .ascii "type"
.globl _class_json_pname_30_1
_class_json_pname_30_1:
    .ascii "url"
.globl _class_json_pname_30_2
_class_json_pname_30_2:
    .ascii "integrity"
.globl _class_json_pname_30_3
_class_json_pname_30_3:
    .ascii "crossorigin"
.globl _class_json_pname_30_4
_class_json_pname_30_4:
    .ascii "mode"
    .p2align 3
.globl _class_json_desc_30
_class_json_desc_30:
    .quad 0
    .quad 0
    .quad 5
    .quad _class_json_pname_30_0
    .quad 4
    .quad 0
    .quad 6
    .quad _class_json_pname_30_1
    .quad 3
    .quad 1
    .quad 1
    .quad _class_json_pname_30_2
    .quad 9
    .quad 2
    .quad 7
    .quad _class_json_pname_30_3
    .quad 11
    .quad 3
    .quad 7
    .quad _class_json_pname_30_4
    .quad 4
    .quad 4
    .quad 6
    .p2align 3
.globl _class_gc_desc_30
_class_gc_desc_30:
    .byte 6, 1, 7, 7, 6
    .p2align 3
.globl _class_vtable_30
_class_vtable_30:
    .quad _method_AIC_N_Components_N_Domain_N_HeadAsset__u__u_construct
    .quad _method_AIC_N_Components_N_Domain_N_HeadAsset_dedupkey
    .p2align 3
.globl _class_static_vtable_30
_class_static_vtable_30:
    .quad 0
.globl _class_callable_method_name_30__u__u_construct
_class_callable_method_name_30__u__u_construct:
    .ascii "__construct"
.globl _class_callable_method_name_30_dedupkey
_class_callable_method_name_30_dedupkey:
    .ascii "dedupkey"
.p2align 3
.globl _class_callable_methods_30
_class_callable_methods_30:
    .quad 2
    .quad _class_callable_method_name_30__u__u_construct
    .quad 11
    .quad _class_callable_method_name_30_dedupkey
    .quad 8
.globl _class_interfaces_37
_class_interfaces_37:
    .quad 2
    .quad 8
    .quad _class_interface_impl_37_8
    .quad 12
    .quad _class_interface_impl_37_12
.globl _class_interface_impl_37_8
_class_interface_impl_37_8:
    .quad 0
    .quad 0
    .quad 0
    .quad 0
    .quad 0
    .quad 0
    .quad 0
    .quad 0
.globl _class_interface_impl_37_12
_class_interface_impl_37_12:
    .quad 0
.globl _class_json_pname_37_0
_class_json_pname_37_0:
    .ascii "message"
    .p2align 3
.globl _class_json_desc_37
_class_json_desc_37:
    .quad 0
    .quad 0
    .quad 1
    .quad _class_json_pname_37_0
    .quad 7
    .quad 0
    .quad 1
    .p2align 3
.globl _class_gc_desc_37
_class_gc_desc_37:
    .byte 1, 0
    .p2align 3
.globl _class_vtable_37
_class_vtable_37:
    .quad 0
    .quad 0
    .quad 0
    .quad 0
    .quad 0
    .quad 0
    .quad 0
    .quad 0
    .quad 0
    .p2align 3
.globl _class_static_vtable_37
_class_static_vtable_37:
    .quad 0
.globl _class_callable_method_name_37__u__u_construct
_class_callable_method_name_37__u__u_construct:
    .ascii "__construct"
.globl _class_callable_method_name_37__u__u_tostring
_class_callable_method_name_37__u__u_tostring:
    .ascii "__tostring"
.globl _class_callable_method_name_37_getcode
_class_callable_method_name_37_getcode:
    .ascii "getcode"
.globl _class_callable_method_name_37_getfile
_class_callable_method_name_37_getfile:
    .ascii "getfile"
.globl _class_callable_method_name_37_getline
_class_callable_method_name_37_getline:
    .ascii "getline"
.globl _class_callable_method_name_37_getmessage
_class_callable_method_name_37_getmessage:
    .ascii "getmessage"
.globl _class_callable_method_name_37_getprevious
_class_callable_method_name_37_getprevious:
    .ascii "getprevious"
.globl _class_callable_method_name_37_gettrace
_class_callable_method_name_37_gettrace:
    .ascii "gettrace"
.globl _class_callable_method_name_37_gettraceasstring
_class_callable_method_name_37_gettraceasstring:
    .ascii "gettraceasstring"
.p2align 3
.globl _class_callable_methods_37
_class_callable_methods_37:
    .quad 9
    .quad _class_callable_method_name_37__u__u_construct
    .quad 11
    .quad _class_callable_method_name_37__u__u_tostring
    .quad 10
    .quad _class_callable_method_name_37_getcode
    .quad 7
    .quad _class_callable_method_name_37_getfile
    .quad 7
    .quad _class_callable_method_name_37_getline
    .quad 7
    .quad _class_callable_method_name_37_getmessage
    .quad 10
    .quad _class_callable_method_name_37_getprevious
    .quad 11
    .quad _class_callable_method_name_37_gettrace
    .quad 8
    .quad _class_callable_method_name_37_gettraceasstring
    .quad 16
.globl _class_interfaces_40
_class_interfaces_40:
    .quad 0
    .p2align 3
.globl _class_json_desc_40
_class_json_desc_40:
    .quad 0
    .quad 0
    .quad 0
    .p2align 3
.globl _class_gc_desc_40
_class_gc_desc_40:
    .byte 4
    .p2align 3
.globl _class_vtable_40
_class_vtable_40:
    .quad _method_ReflectionProperty__u__u_construct
    .quad _method_ReflectionProperty_getattributes
    .p2align 3
.globl _class_static_vtable_40
_class_static_vtable_40:
    .quad 0
.globl _class_callable_method_name_40__u__u_construct
_class_callable_method_name_40__u__u_construct:
    .ascii "__construct"
.globl _class_callable_method_name_40_getattributes
_class_callable_method_name_40_getattributes:
    .ascii "getattributes"
.p2align 3
.globl _class_callable_methods_40
_class_callable_methods_40:
    .quad 2
    .quad _class_callable_method_name_40__u__u_construct
    .quad 11
    .quad _class_callable_method_name_40_getattributes
    .quad 13
.globl _class_interfaces_41
_class_interfaces_41:
    .quad 2
    .quad 8
    .quad _class_interface_impl_41_8
    .quad 12
    .quad _class_interface_impl_41_12
.globl _class_interface_impl_41_8
_class_interface_impl_41_8:
    .quad 0
    .quad 0
    .quad 0
    .quad 0
    .quad 0
    .quad 0
    .quad 0
    .quad 0
.globl _class_interface_impl_41_12
_class_interface_impl_41_12:
    .quad 0
.globl _class_json_pname_41_0
_class_json_pname_41_0:
    .ascii "message"
    .p2align 3
.globl _class_json_desc_41
_class_json_desc_41:
    .quad 0
    .quad 0
    .quad 1
    .quad _class_json_pname_41_0
    .quad 7
    .quad 0
    .quad 1
    .p2align 3
.globl _class_gc_desc_41
_class_gc_desc_41:
    .byte 1, 0
    .p2align 3
.globl _class_vtable_41
_class_vtable_41:
    .quad 0
    .quad 0
    .quad 0
    .quad 0
    .quad 0
    .quad 0
    .quad 0
    .quad 0
    .quad 0
    .p2align 3
.globl _class_static_vtable_41
_class_static_vtable_41:
    .quad 0
.globl _class_callable_method_name_41__u__u_construct
_class_callable_method_name_41__u__u_construct:
    .ascii "__construct"
.globl _class_callable_method_name_41__u__u_tostring
_class_callable_method_name_41__u__u_tostring:
    .ascii "__tostring"
.globl _class_callable_method_name_41_getcode
_class_callable_method_name_41_getcode:
    .ascii "getcode"
.globl _class_callable_method_name_41_getfile
_class_callable_method_name_41_getfile:
    .ascii "getfile"
.globl _class_callable_method_name_41_getline
_class_callable_method_name_41_getline:
    .ascii "getline"
.globl _class_callable_method_name_41_getmessage
_class_callable_method_name_41_getmessage:
    .ascii "getmessage"
.globl _class_callable_method_name_41_getprevious
_class_callable_method_name_41_getprevious:
    .ascii "getprevious"
.globl _class_callable_method_name_41_gettrace
_class_callable_method_name_41_gettrace:
    .ascii "gettrace"
.globl _class_callable_method_name_41_gettraceasstring
_class_callable_method_name_41_gettraceasstring:
    .ascii "gettraceasstring"
.p2align 3
.globl _class_callable_methods_41
_class_callable_methods_41:
    .quad 9
    .quad _class_callable_method_name_41__u__u_construct
    .quad 11
    .quad _class_callable_method_name_41__u__u_tostring
    .quad 10
    .quad _class_callable_method_name_41_getcode
    .quad 7
    .quad _class_callable_method_name_41_getfile
    .quad 7
    .quad _class_callable_method_name_41_getline
    .quad 7
    .quad _class_callable_method_name_41_getmessage
    .quad 10
    .quad _class_callable_method_name_41_getprevious
    .quad 11
    .quad _class_callable_method_name_41_gettrace
    .quad 8
    .quad _class_callable_method_name_41_gettraceasstring
    .quad 16
.globl _class_interfaces_42
_class_interfaces_42:
    .quad 2
    .quad 8
    .quad _class_interface_impl_42_8
    .quad 12
    .quad _class_interface_impl_42_12
.globl _class_interface_impl_42_8
_class_interface_impl_42_8:
    .quad 0
    .quad 0
    .quad 0
    .quad 0
    .quad 0
    .quad 0
    .quad 0
    .quad 0
.globl _class_interface_impl_42_12
_class_interface_impl_42_12:
    .quad 0
.globl _class_json_pname_42_0
_class_json_pname_42_0:
    .ascii "message"
    .p2align 3
.globl _class_json_desc_42
_class_json_desc_42:
    .quad 0
    .quad 0
    .quad 1
    .quad _class_json_pname_42_0
    .quad 7
    .quad 0
    .quad 1
    .p2align 3
.globl _class_gc_desc_42
_class_gc_desc_42:
    .byte 1, 0
    .p2align 3
.globl _class_vtable_42
_class_vtable_42:
    .quad 0
    .quad 0
    .quad 0
    .quad 0
    .quad 0
    .quad 0
    .quad 0
    .quad 0
    .quad 0
    .p2align 3
.globl _class_static_vtable_42
_class_static_vtable_42:
    .quad 0
.globl _class_callable_method_name_42__u__u_construct
_class_callable_method_name_42__u__u_construct:
    .ascii "__construct"
.globl _class_callable_method_name_42__u__u_tostring
_class_callable_method_name_42__u__u_tostring:
    .ascii "__tostring"
.globl _class_callable_method_name_42_getcode
_class_callable_method_name_42_getcode:
    .ascii "getcode"
.globl _class_callable_method_name_42_getfile
_class_callable_method_name_42_getfile:
    .ascii "getfile"
.globl _class_callable_method_name_42_getline
_class_callable_method_name_42_getline:
    .ascii "getline"
.globl _class_callable_method_name_42_getmessage
_class_callable_method_name_42_getmessage:
    .ascii "getmessage"
.globl _class_callable_method_name_42_getprevious
_class_callable_method_name_42_getprevious:
    .ascii "getprevious"
.globl _class_callable_method_name_42_gettrace
_class_callable_method_name_42_gettrace:
    .ascii "gettrace"
.globl _class_callable_method_name_42_gettraceasstring
_class_callable_method_name_42_gettraceasstring:
    .ascii "gettraceasstring"
.p2align 3
.globl _class_callable_methods_42
_class_callable_methods_42:
    .quad 9
    .quad _class_callable_method_name_42__u__u_construct
    .quad 11
    .quad _class_callable_method_name_42__u__u_tostring
    .quad 10
    .quad _class_callable_method_name_42_getcode
    .quad 7
    .quad _class_callable_method_name_42_getfile
    .quad 7
    .quad _class_callable_method_name_42_getline
    .quad 7
    .quad _class_callable_method_name_42_getmessage
    .quad 10
    .quad _class_callable_method_name_42_getprevious
    .quad 11
    .quad _class_callable_method_name_42_gettrace
    .quad 8
    .quad _class_callable_method_name_42_gettraceasstring
    .quad 16
.globl _class_interfaces_46
_class_interfaces_46:
    .quad 2
    .quad 8
    .quad _class_interface_impl_46_8
    .quad 12
    .quad _class_interface_impl_46_12
.globl _class_interface_impl_46_8
_class_interface_impl_46_8:
    .quad 0
    .quad 0
    .quad 0
    .quad 0
    .quad 0
    .quad 0
    .quad 0
    .quad 0
.globl _class_interface_impl_46_12
_class_interface_impl_46_12:
    .quad 0
.globl _class_json_pname_46_0
_class_json_pname_46_0:
    .ascii "message"
    .p2align 3
.globl _class_json_desc_46
_class_json_desc_46:
    .quad 0
    .quad 0
    .quad 1
    .quad _class_json_pname_46_0
    .quad 7
    .quad 0
    .quad 1
    .p2align 3
.globl _class_gc_desc_46
_class_gc_desc_46:
    .byte 1, 0
    .p2align 3
.globl _class_vtable_46
_class_vtable_46:
    .quad 0
    .quad 0
    .quad 0
    .quad 0
    .quad 0
    .quad 0
    .quad 0
    .quad 0
    .quad 0
    .p2align 3
.globl _class_static_vtable_46
_class_static_vtable_46:
    .quad 0
.globl _class_callable_method_name_46__u__u_construct
_class_callable_method_name_46__u__u_construct:
    .ascii "__construct"
.globl _class_callable_method_name_46__u__u_tostring
_class_callable_method_name_46__u__u_tostring:
    .ascii "__tostring"
.globl _class_callable_method_name_46_getcode
_class_callable_method_name_46_getcode:
    .ascii "getcode"
.globl _class_callable_method_name_46_getfile
_class_callable_method_name_46_getfile:
    .ascii "getfile"
.globl _class_callable_method_name_46_getline
_class_callable_method_name_46_getline:
    .ascii "getline"
.globl _class_callable_method_name_46_getmessage
_class_callable_method_name_46_getmessage:
    .ascii "getmessage"
.globl _class_callable_method_name_46_getprevious
_class_callable_method_name_46_getprevious:
    .ascii "getprevious"
.globl _class_callable_method_name_46_gettrace
_class_callable_method_name_46_gettrace:
    .ascii "gettrace"
.globl _class_callable_method_name_46_gettraceasstring
_class_callable_method_name_46_gettraceasstring:
    .ascii "gettraceasstring"
.p2align 3
.globl _class_callable_methods_46
_class_callable_methods_46:
    .quad 9
    .quad _class_callable_method_name_46__u__u_construct
    .quad 11
    .quad _class_callable_method_name_46__u__u_tostring
    .quad 10
    .quad _class_callable_method_name_46_getcode
    .quad 7
    .quad _class_callable_method_name_46_getfile
    .quad 7
    .quad _class_callable_method_name_46_getline
    .quad 7
    .quad _class_callable_method_name_46_getmessage
    .quad 10
    .quad _class_callable_method_name_46_getprevious
    .quad 11
    .quad _class_callable_method_name_46_gettrace
    .quad 8
    .quad _class_callable_method_name_46_gettraceasstring
    .quad 16
.globl _class_interfaces_47
_class_interfaces_47:
    .quad 2
    .quad 8
    .quad _class_interface_impl_47_8
    .quad 12
    .quad _class_interface_impl_47_12
.globl _class_interface_impl_47_8
_class_interface_impl_47_8:
    .quad 0
    .quad 0
    .quad 0
    .quad 0
    .quad 0
    .quad 0
    .quad 0
    .quad 0
.globl _class_interface_impl_47_12
_class_interface_impl_47_12:
    .quad 0
.globl _class_json_pname_47_0
_class_json_pname_47_0:
    .ascii "message"
    .p2align 3
.globl _class_json_desc_47
_class_json_desc_47:
    .quad 0
    .quad 0
    .quad 1
    .quad _class_json_pname_47_0
    .quad 7
    .quad 0
    .quad 1
    .p2align 3
.globl _class_gc_desc_47
_class_gc_desc_47:
    .byte 1, 0
    .p2align 3
.globl _class_vtable_47
_class_vtable_47:
    .quad 0
    .quad 0
    .quad 0
    .quad 0
    .quad 0
    .quad 0
    .quad 0
    .quad 0
    .quad 0
    .p2align 3
.globl _class_static_vtable_47
_class_static_vtable_47:
    .quad 0
.globl _class_callable_method_name_47__u__u_construct
_class_callable_method_name_47__u__u_construct:
    .ascii "__construct"
.globl _class_callable_method_name_47__u__u_tostring
_class_callable_method_name_47__u__u_tostring:
    .ascii "__tostring"
.globl _class_callable_method_name_47_getcode
_class_callable_method_name_47_getcode:
    .ascii "getcode"
.globl _class_callable_method_name_47_getfile
_class_callable_method_name_47_getfile:
    .ascii "getfile"
.globl _class_callable_method_name_47_getline
_class_callable_method_name_47_getline:
    .ascii "getline"
.globl _class_callable_method_name_47_getmessage
_class_callable_method_name_47_getmessage:
    .ascii "getmessage"
.globl _class_callable_method_name_47_getprevious
_class_callable_method_name_47_getprevious:
    .ascii "getprevious"
.globl _class_callable_method_name_47_gettrace
_class_callable_method_name_47_gettrace:
    .ascii "gettrace"
.globl _class_callable_method_name_47_gettraceasstring
_class_callable_method_name_47_gettraceasstring:
    .ascii "gettraceasstring"
.p2align 3
.globl _class_callable_methods_47
_class_callable_methods_47:
    .quad 9
    .quad _class_callable_method_name_47__u__u_construct
    .quad 11
    .quad _class_callable_method_name_47__u__u_tostring
    .quad 10
    .quad _class_callable_method_name_47_getcode
    .quad 7
    .quad _class_callable_method_name_47_getfile
    .quad 7
    .quad _class_callable_method_name_47_getline
    .quad 7
    .quad _class_callable_method_name_47_getmessage
    .quad 10
    .quad _class_callable_method_name_47_getprevious
    .quad 11
    .quad _class_callable_method_name_47_gettrace
    .quad 8
    .quad _class_callable_method_name_47_gettraceasstring
    .quad 16
.globl _class_interfaces_54
_class_interfaces_54:
    .quad 0
    .p2align 3
.globl _class_json_desc_54
_class_json_desc_54:
    .quad 0
    .quad 0
    .quad 0
    .p2align 3
.globl _class_gc_desc_54
_class_gc_desc_54:
    .byte 1, 4
    .p2align 3
.globl _class_vtable_54
_class_vtable_54:
    .quad _method_ReflectionClass__u__u_construct
    .quad _method_ReflectionClass_getname
    .quad _method_ReflectionClass_getattributes
    .p2align 3
.globl _class_static_vtable_54
_class_static_vtable_54:
    .quad 0
.globl _class_callable_method_name_54__u__u_construct
_class_callable_method_name_54__u__u_construct:
    .ascii "__construct"
.globl _class_callable_method_name_54_getattributes
_class_callable_method_name_54_getattributes:
    .ascii "getattributes"
.globl _class_callable_method_name_54_getname
_class_callable_method_name_54_getname:
    .ascii "getname"
.p2align 3
.globl _class_callable_methods_54
_class_callable_methods_54:
    .quad 3
    .quad _class_callable_method_name_54__u__u_construct
    .quad 11
    .quad _class_callable_method_name_54_getattributes
    .quad 13
    .quad _class_callable_method_name_54_getname
    .quad 7
.globl _class_interfaces_60
_class_interfaces_60:
    .quad 0
    .p2align 3
.globl _class_json_desc_60
_class_json_desc_60:
    .quad 0
    .quad 0
    .quad 0
    .p2align 3
.globl _class_gc_desc_60
_class_gc_desc_60:
    .byte 1, 4, 0
    .p2align 3
.globl _class_vtable_60
_class_vtable_60:
    .quad _method_ReflectionAttribute_getname
    .quad _method_ReflectionAttribute_getarguments
    .quad _method_ReflectionAttribute_newinstance
    .p2align 3
.globl _class_static_vtable_60
_class_static_vtable_60:
    .quad 0
.globl _class_callable_method_name_60_getarguments
_class_callable_method_name_60_getarguments:
    .ascii "getarguments"
.globl _class_callable_method_name_60_getname
_class_callable_method_name_60_getname:
    .ascii "getname"
.globl _class_callable_method_name_60_newinstance
_class_callable_method_name_60_newinstance:
    .ascii "newinstance"
.p2align 3
.globl _class_callable_methods_60
_class_callable_methods_60:
    .quad 3
    .quad _class_callable_method_name_60_getarguments
    .quad 12
    .quad _class_callable_method_name_60_getname
    .quad 7
    .quad _class_callable_method_name_60_newinstance
    .quad 11
.globl _class_interfaces_75
_class_interfaces_75:
    .quad 2
    .quad 8
    .quad _class_interface_impl_75_8
    .quad 12
    .quad _class_interface_impl_75_12
.globl _class_interface_impl_75_8
_class_interface_impl_75_8:
    .quad 0
    .quad 0
    .quad 0
    .quad 0
    .quad 0
    .quad 0
    .quad 0
    .quad 0
.globl _class_interface_impl_75_12
_class_interface_impl_75_12:
    .quad 0
.globl _class_json_pname_75_0
_class_json_pname_75_0:
    .ascii "message"
    .p2align 3
.globl _class_json_desc_75
_class_json_desc_75:
    .quad 0
    .quad 0
    .quad 1
    .quad _class_json_pname_75_0
    .quad 7
    .quad 0
    .quad 1
    .p2align 3
.globl _class_gc_desc_75
_class_gc_desc_75:
    .byte 1, 0
    .p2align 3
.globl _class_vtable_75
_class_vtable_75:
    .quad 0
    .quad 0
    .quad 0
    .quad 0
    .quad 0
    .quad 0
    .quad 0
    .quad 0
    .quad 0
    .p2align 3
.globl _class_static_vtable_75
_class_static_vtable_75:
    .quad 0
.globl _class_callable_method_name_75__u__u_construct
_class_callable_method_name_75__u__u_construct:
    .ascii "__construct"
.globl _class_callable_method_name_75__u__u_tostring
_class_callable_method_name_75__u__u_tostring:
    .ascii "__tostring"
.globl _class_callable_method_name_75_getcode
_class_callable_method_name_75_getcode:
    .ascii "getcode"
.globl _class_callable_method_name_75_getfile
_class_callable_method_name_75_getfile:
    .ascii "getfile"
.globl _class_callable_method_name_75_getline
_class_callable_method_name_75_getline:
    .ascii "getline"
.globl _class_callable_method_name_75_getmessage
_class_callable_method_name_75_getmessage:
    .ascii "getmessage"
.globl _class_callable_method_name_75_getprevious
_class_callable_method_name_75_getprevious:
    .ascii "getprevious"
.globl _class_callable_method_name_75_gettrace
_class_callable_method_name_75_gettrace:
    .ascii "gettrace"
.globl _class_callable_method_name_75_gettraceasstring
_class_callable_method_name_75_gettraceasstring:
    .ascii "gettraceasstring"
.p2align 3
.globl _class_callable_methods_75
_class_callable_methods_75:
    .quad 9
    .quad _class_callable_method_name_75__u__u_construct
    .quad 11
    .quad _class_callable_method_name_75__u__u_tostring
    .quad 10
    .quad _class_callable_method_name_75_getcode
    .quad 7
    .quad _class_callable_method_name_75_getfile
    .quad 7
    .quad _class_callable_method_name_75_getline
    .quad 7
    .quad _class_callable_method_name_75_getmessage
    .quad 10
    .quad _class_callable_method_name_75_getprevious
    .quad 11
    .quad _class_callable_method_name_75_gettrace
    .quad 8
    .quad _class_callable_method_name_75_gettraceasstring
    .quad 16
.globl _class_interfaces_78
_class_interfaces_78:
    .quad 2
    .quad 8
    .quad _class_interface_impl_78_8
    .quad 12
    .quad _class_interface_impl_78_12
.globl _class_interface_impl_78_8
_class_interface_impl_78_8:
    .quad 0
    .quad 0
    .quad 0
    .quad 0
    .quad 0
    .quad 0
    .quad 0
    .quad 0
.globl _class_interface_impl_78_12
_class_interface_impl_78_12:
    .quad 0
.globl _class_json_pname_78_0
_class_json_pname_78_0:
    .ascii "message"
    .p2align 3
.globl _class_json_desc_78
_class_json_desc_78:
    .quad 0
    .quad 0
    .quad 1
    .quad _class_json_pname_78_0
    .quad 7
    .quad 0
    .quad 1
    .p2align 3
.globl _class_gc_desc_78
_class_gc_desc_78:
    .byte 1, 0
    .p2align 3
.globl _class_vtable_78
_class_vtable_78:
    .quad 0
    .quad 0
    .quad 0
    .quad 0
    .quad 0
    .quad 0
    .quad 0
    .quad 0
    .quad 0
    .p2align 3
.globl _class_static_vtable_78
_class_static_vtable_78:
    .quad 0
.globl _class_callable_method_name_78__u__u_construct
_class_callable_method_name_78__u__u_construct:
    .ascii "__construct"
.globl _class_callable_method_name_78__u__u_tostring
_class_callable_method_name_78__u__u_tostring:
    .ascii "__tostring"
.globl _class_callable_method_name_78_getcode
_class_callable_method_name_78_getcode:
    .ascii "getcode"
.globl _class_callable_method_name_78_getfile
_class_callable_method_name_78_getfile:
    .ascii "getfile"
.globl _class_callable_method_name_78_getline
_class_callable_method_name_78_getline:
    .ascii "getline"
.globl _class_callable_method_name_78_getmessage
_class_callable_method_name_78_getmessage:
    .ascii "getmessage"
.globl _class_callable_method_name_78_getprevious
_class_callable_method_name_78_getprevious:
    .ascii "getprevious"
.globl _class_callable_method_name_78_gettrace
_class_callable_method_name_78_gettrace:
    .ascii "gettrace"
.globl _class_callable_method_name_78_gettraceasstring
_class_callable_method_name_78_gettraceasstring:
    .ascii "gettraceasstring"
.p2align 3
.globl _class_callable_methods_78
_class_callable_methods_78:
    .quad 9
    .quad _class_callable_method_name_78__u__u_construct
    .quad 11
    .quad _class_callable_method_name_78__u__u_tostring
    .quad 10
    .quad _class_callable_method_name_78_getcode
    .quad 7
    .quad _class_callable_method_name_78_getfile
    .quad 7
    .quad _class_callable_method_name_78_getline
    .quad 7
    .quad _class_callable_method_name_78_getmessage
    .quad 10
    .quad _class_callable_method_name_78_getprevious
    .quad 11
    .quad _class_callable_method_name_78_gettrace
    .quad 8
    .quad _class_callable_method_name_78_gettraceasstring
    .quad 16
.globl _class_interfaces_80
_class_interfaces_80:
    .quad 2
    .quad 8
    .quad _class_interface_impl_80_8
    .quad 12
    .quad _class_interface_impl_80_12
.globl _class_interface_impl_80_8
_class_interface_impl_80_8:
    .quad 0
    .quad 0
    .quad 0
    .quad 0
    .quad 0
    .quad 0
    .quad 0
    .quad 0
.globl _class_interface_impl_80_12
_class_interface_impl_80_12:
    .quad 0
.globl _class_json_pname_80_0
_class_json_pname_80_0:
    .ascii "message"
    .p2align 3
.globl _class_json_desc_80
_class_json_desc_80:
    .quad 0
    .quad 0
    .quad 1
    .quad _class_json_pname_80_0
    .quad 7
    .quad 0
    .quad 1
    .p2align 3
.globl _class_gc_desc_80
_class_gc_desc_80:
    .byte 1, 0
    .p2align 3
.globl _class_vtable_80
_class_vtable_80:
    .quad 0
    .quad 0
    .quad 0
    .quad 0
    .quad 0
    .quad 0
    .quad 0
    .quad 0
    .quad 0
    .p2align 3
.globl _class_static_vtable_80
_class_static_vtable_80:
    .quad 0
.globl _class_callable_method_name_80__u__u_construct
_class_callable_method_name_80__u__u_construct:
    .ascii "__construct"
.globl _class_callable_method_name_80__u__u_tostring
_class_callable_method_name_80__u__u_tostring:
    .ascii "__tostring"
.globl _class_callable_method_name_80_getcode
_class_callable_method_name_80_getcode:
    .ascii "getcode"
.globl _class_callable_method_name_80_getfile
_class_callable_method_name_80_getfile:
    .ascii "getfile"
.globl _class_callable_method_name_80_getline
_class_callable_method_name_80_getline:
    .ascii "getline"
.globl _class_callable_method_name_80_getmessage
_class_callable_method_name_80_getmessage:
    .ascii "getmessage"
.globl _class_callable_method_name_80_getprevious
_class_callable_method_name_80_getprevious:
    .ascii "getprevious"
.globl _class_callable_method_name_80_gettrace
_class_callable_method_name_80_gettrace:
    .ascii "gettrace"
.globl _class_callable_method_name_80_gettraceasstring
_class_callable_method_name_80_gettraceasstring:
    .ascii "gettraceasstring"
.p2align 3
.globl _class_callable_methods_80
_class_callable_methods_80:
    .quad 9
    .quad _class_callable_method_name_80__u__u_construct
    .quad 11
    .quad _class_callable_method_name_80__u__u_tostring
    .quad 10
    .quad _class_callable_method_name_80_getcode
    .quad 7
    .quad _class_callable_method_name_80_getfile
    .quad 7
    .quad _class_callable_method_name_80_getline
    .quad 7
    .quad _class_callable_method_name_80_getmessage
    .quad 10
    .quad _class_callable_method_name_80_getprevious
    .quad 11
    .quad _class_callable_method_name_80_gettrace
    .quad 8
    .quad _class_callable_method_name_80_gettraceasstring
    .quad 16
.globl _class_interfaces_88
_class_interfaces_88:
    .quad 0
.globl _class_json_pname_88_0
_class_json_pname_88_0:
    .ascii "value"
.globl _class_json_pname_88_1
_class_json_pname_88_1:
    .ascii "name"
    .p2align 3
.globl _class_json_desc_88
_class_json_desc_88:
    .quad 0
    .quad 0
    .quad 2
    .quad _class_json_pname_88_0
    .quad 5
    .quad 0
    .quad 1
    .quad _class_json_pname_88_1
    .quad 4
    .quad 1
    .quad 1
    .p2align 3
.globl _class_gc_desc_88
_class_gc_desc_88:
    .byte 1, 1
    .p2align 3
.globl _class_vtable_88
_class_vtable_88:
    .quad 0
    .p2align 3
.globl _class_static_vtable_88
_class_static_vtable_88:
    .quad 0
.p2align 3
.globl _class_callable_methods_88
_class_callable_methods_88:
    .quad 0
.globl _class_interfaces_89
_class_interfaces_89:
    .quad 0
.globl _class_json_pname_89_0
_class_json_pname_89_0:
    .ascii "value"
.globl _class_json_pname_89_1
_class_json_pname_89_1:
    .ascii "name"
    .p2align 3
.globl _class_json_desc_89
_class_json_desc_89:
    .quad 0
    .quad 0
    .quad 2
    .quad _class_json_pname_89_0
    .quad 5
    .quad 0
    .quad 1
    .quad _class_json_pname_89_1
    .quad 4
    .quad 1
    .quad 1
    .p2align 3
.globl _class_gc_desc_89
_class_gc_desc_89:
    .byte 1, 1
    .p2align 3
.globl _class_vtable_89
_class_vtable_89:
    .quad 0
    .p2align 3
.globl _class_static_vtable_89
_class_static_vtable_89:
    .quad 0
.p2align 3
.globl _class_callable_methods_89
_class_callable_methods_89:
    .quad 0
.p2align 3
.globl _stdclass_class_id
_stdclass_class_id:
    .quad 65
