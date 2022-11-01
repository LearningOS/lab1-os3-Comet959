# os/src/task/switch.S
# 任务切换

.altmacro # 开启宏命令 才可以使用.rept
.macro SAVE_SN n # 定制宏命令SAVE_SN
    sd s\n, (\n+2)*8(a0) # 将s\n的8个字节压入栈指针(a0+8)*(\n+2)的位置
.endm
.macro LOAD_SN n
    ld s\n, (\n+2)*8(a1)
.endm

    .section .text # 代码段
    .globl __switch
__switch:
    # __switch(
    #     current_task_cx_ptr: *mut TaskContext,
    #     next_task_cx_ptr: *const TaskContext
    # )
    # 保存当前任务的内核栈
    sd sp, 8(a0) # 将sp的8个字节压入a0+8
    # 保存 ra和当前执行态的s0-s11
    sd ra, 0(a0) # 压入0的位置
    .set n, 0
    .rept 12 # 重复12遍
        SAVE_SN %n
        .set n, n + 1
    .endr
    # 恢复下一个执行状态
    ld ra, 0(a1)
    .set n, 0
    .rept 12
        LOAD_SN %n
        .set n, n + 1
    .endr
    # 恢复下一个任务的内核栈
    ld sp, 8(a1)
    ret