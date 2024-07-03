//! 定义了完整的 TaskContext 结构，
//! 将协程上下文、线程上下文、trap 上下文整合到一个数据结构中
//! 
//! 从 thread/coroutine 到 process、os、hypervisor，任务的上下文逐渐增加。
//! 
//! 1. 协程上下文结构：从操作系统的角度来看，没有明确的指出使用到了哪个寄存器，由编译器来保存和恢复寄存器。
//! 可能在控制块中需要维护的就是一个 core::task::Context 结构（实际上是指向自己的指针）。
//! 但这个结构只有在协程运行时才会使用，因此在保存时，无需保存在上下文中。
//! 
//! 2. 线程上下文：保存了额外的栈指针
//! 
//! 3. 陷入上下文：根据发生 trap 前后的特权级不同，需要保存的 trap 上下文可能存在不同，
//! 区别主要体现在与特权级相关的寄存器。
//! 
//! 思路分析：
//! 
//! 在同一个地址空间内，正常的执行流的上下文只有 cx、ra、sp、s0~s11 这些寄存器。
//! 但执行流在任意时刻被打断时，需要保存其他的额外的通用寄存器（gp、tp、t0~t6、a0~a7），
//! 这些构成了基本的执行流。
//! 
//! 这些是在同一个特权级下的执行流，当需要切换特权级时，基本执行流保存的信息是否足够呢？例如从内核切换到用户态， 这里有两种思路：
//! 1. 将这种执行流的切换，视为是上一个内核的普通执行流切换到下一个处于用户态的执行流；
//! 2. 用户态执行流在内核态存在对应的内核执行流，从对应的内核执行流切换到用户态执行流；
//! 
//! 在目前大多数的系统实现中，采用的是第二种，用户态线程对应着一个内核线程，切换到用户态是从对应的内核线程再跳转的。
//! 因此，这些实现中，需要保留空闲的内核线程栈供进入内核时使用。参考 rCore-tutorial 里的内核支持的线程实现来看，
//! 每次从用户态陷入到内核时，内核栈始终是空的。
//! 
//! 当内核中的执行没有阻塞等待情况时，实际上是可以复用同一个内核栈时；
//! 而当内核执行流阻塞时，传统的做法则需要将执行流的调用关系保存在栈上。
//! 
//! 而使用协程，在内核的执行流即使阻塞了，编译器会将信息保存在堆上，从而可以重复使用同一个栈。
//! 
//! 因此，使用协程，不需要为用户态执行流额外维护对应的内核执行流上下文，
//! 无论处于什么特权级或地址空间中，都只需要保存一个执行流的上下文。
//! 即使这个执行流是在用户态执行的，也只需要保存一套通用寄存器和一些特权级寄存器信息。
//! 
//! 当在某个特权级发生中断时，需要保存在原来特权级下的执行流。
//! 原来的执行流保存到上下文中，切换到新的栈上，运行调度器，取出中断处理任务。
//! 这个过程也不需要保存上下文。因此一个执行流足够表示在各个特权级下的运行。
//! 
//! TaskContext 在任意时刻，只会有一个 CPU 访问，因此使用 Box 指针包裹即可

use core::ptr::NonNull;

#[derive(Debug)]
pub struct TaskContextRef(pub NonNull<TaskContext>);

#[repr(C)]
#[derive(Debug)]
pub struct TaskContext {
    /// 
    pub x: [usize; 31],
    ///
    pub addrspace_token: usize,
    /// 
    pub free_sp: usize,
    /// 
    pub schedule_fn: usize,
    /// 
    pub priv_info: PrivInfo,
}

#[repr(C)]
#[derive(Debug)]
pub enum PrivInfo {
    SPrivilige(SPrivilige),
    UPrivilige(UPrivilige),
    UnKnown
}

#[repr(C)]
#[derive(Debug)]
pub struct SPrivilige {
    pub sstatus: usize,
    pub sepc: usize,
    pub stvec: usize,
    pub sie: usize,
}

#[repr(C)]
#[derive(Debug)]
pub struct UPrivilige {
    pub ustatus: usize,
    pub uepc: usize,
    pub utvec: usize,
    pub uie: usize,
}