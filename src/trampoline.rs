//! 执行流的切换，始终是保存当前的执行流上下文，
//! 跳转至调度器，从调度器中取出下一个任务，
//! 执行下一个任务。
//! 
//! 因此，切换的入口始终是当前的执行流上下文
//! 
//! 无论是从哪个入口进入到跳板页，需要一个默认的方式
//! 来获取当前正在运行的上下文的执行流上下文。
//! 需要一个寄存器来保存当前的执行流上下文。
//! 

use core::arch::asm;
use core::ptr::NonNull;
use crate::{context::TaskContextRef, current::{set_current_task_ctx}, TaskContext};

#[allow(unused)]
const TASKCTX_SIZE: usize = core::mem::size_of::<TaskContext>();

/// 上一个执行流因为中断/异常进入到这个函数，
/// 首先需要获取当前的上下文指针，保存寄存器现场
#[naked]
pub unsafe extern "C" fn trap_entry() {
    // 3. 进入调度函数
    asm!(
        // 1. 先将当前的寄存器保存在栈上
        "sd x1, -{taskctx_size} + 0(sp)",
        "sd x2, -{taskctx_size} + 8(sp)",
        "sd x3, -{taskctx_size} + 16(sp)",
        "sd x4, -{taskctx_size} + 24(sp)",
        "sd x5, -{taskctx_size} + 32(sp)",
        "sd x6, -{taskctx_size} + 40(sp)",
        "sd x7, -{taskctx_size} + 48(sp)",
        "sd x8, -{taskctx_size} + 56(sp)",
        "sd x9, -{taskctx_size} + 64(sp)",
        "sd x10, -{taskctx_size} + 72(sp)",
        "sd x11, -{taskctx_size} + 80(sp)",
        "sd x12, -{taskctx_size} + 88(sp)",
        "sd x13, -{taskctx_size} + 96(sp)",
        "sd x14, -{taskctx_size} + 104(sp)",
        "sd x15, -{taskctx_size} + 112(sp)",
        "sd x16, -{taskctx_size} + 120(sp)",
        "sd x17, -{taskctx_size} + 128(sp)",
        "sd x18, -{taskctx_size} + 136(sp)",
        "sd x19, -{taskctx_size} + 144(sp)",
        "sd x20, -{taskctx_size} + 152(sp)",
        "sd x21, -{taskctx_size} + 160(sp)",
        "sd x22, -{taskctx_size} + 168(sp)",
        "sd x23, -{taskctx_size} + 176(sp)",
        "sd x24, -{taskctx_size} + 184(sp)",
        "sd x25, -{taskctx_size} + 192(sp)",
        "sd x26, -{taskctx_size} + 200(sp)",
        "sd x27, -{taskctx_size} + 208(sp)",
        "sd x28, -{taskctx_size} + 216(sp)",
        "sd x29, -{taskctx_size} + 224(sp)",
        "sd x30, -{taskctx_size} + 232(sp)",
        "sd x31, -{taskctx_size} + 240(sp)",
        // 2. 
        "addi sp, sp, -{taskctx_size}",
        "mv a0, sp",
        "call {schedule_fn}",

        taskctx_size = const TASKCTX_SIZE,
        schedule_fn = sym schedule,
        options(noreturn)
    )
}

pub fn schedule(task_ctx: *const TaskContext) {
    // 1. 找到一个新的栈，并切换到新的栈上
    // 2. 更新上下文指针
    let task_ref = TaskContextRef(unsafe { NonNull::new_unchecked(task_ctx as *mut _) });
    set_current_task_ctx(task_ref);
    // let _ = current_task_ctx();
}