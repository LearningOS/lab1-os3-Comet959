pub const USER_STACK_SIZE: usize = 4096; // 用户栈大小
pub const KERNEL_STACK_SIZE: usize = 4096 * 20; // 内核栈大小
pub const KERNEL_HEAP_SIZE: usize = 0x20000; // 内核堆大小
pub const MAX_APP_NUM: usize = 16; // 最大app数量
pub const APP_BASE_ADDRESS: usize = 0x80400000; // app基地址
pub const APP_SIZE_LIMIT: usize = 0x20000; // app大小限制
pub const CLOCK_FREQ: usize = 12500000; // 时钟频率
pub const MAX_SYSCALL_NUM: usize = 500; // 最大系统调用数量
