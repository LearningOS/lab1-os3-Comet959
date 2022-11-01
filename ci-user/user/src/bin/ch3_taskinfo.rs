#![no_std]
#![no_main]

extern crate user_lib;

use user_lib::{
    get_time, println, sleep, task_info, TaskInfo, TaskStatus, SYSCALL_EXIT, SYSCALL_GETTIMEOFDAY,
    SYSCALL_TASK_INFO, SYSCALL_WRITE, SYSCALL_YIELD,
};

#[no_mangle]
pub fn main() -> usize {
    let t1 = get_time() as usize;
    let info = TaskInfo::new();
    get_time();
    sleep(500);
    let t2 = get_time() as usize;
    // 注意本次 task info 调用也计入
    assert_eq!(0, task_info(&info));
    let t3 = get_time() as usize;
    assert!(3 <= info.syscall_times[SYSCALL_GETTIMEOFDAY]); // 169 get_time执行次数大于2
    assert_eq!(1, info.syscall_times[SYSCALL_TASK_INFO]); // task_info调用只执行一次
    assert_eq!(0, info.syscall_times[SYSCALL_WRITE]); // write调用不执行
    assert!(0 < info.syscall_times[SYSCALL_YIELD]); // yield调用执行至少一次
    assert_eq!(0, info.syscall_times[SYSCALL_EXIT]); // exit调用不执行
    assert!(t2 - t1 <= info.time + 1); // t2-t1是 sleep(500)的执行时间 info.time应该大于它
    // println!("t2: {}, t1: {}, t2-t1: {}, info.time+1: {}", t2, t1, t2-t1, info.time+1);
    // println!("info.time: {}, t3: {}, t1: {}, t3-t1+100:{}", info.time, t3, t1, t3-t1+100);
    assert!(info.time < t3 - t1 + 100); // t3-t1是sleep(500) + 调用task_info自身的共同时间, 应该比调用自身的时间长
    assert!(info.status == TaskStatus::Running);

    // 想想为什么 write 调用是两次
    println!("string from task info test\n");
    let t4 = get_time() as usize;
    assert_eq!(0, task_info(&info));
    let t5 = get_time() as usize;
    assert!(5 <= info.syscall_times[SYSCALL_GETTIMEOFDAY]);
    assert_eq!(2, info.syscall_times[SYSCALL_TASK_INFO]);
    assert_eq!(2, info.syscall_times[SYSCALL_WRITE]);
    assert!(0 < info.syscall_times[SYSCALL_YIELD]);
    assert_eq!(0, info.syscall_times[SYSCALL_EXIT]);
    println!("t4: {}, t1: {}, t4-t1: {}, info.time: {}", t4, t1, t4-t1, info.time);
    assert!(t4 - t1 <= info.time + 1);
    assert!(info.time < t5 - t1 + 100);
    assert!(info.status == TaskStatus::Running);

    println!("Test task info OK!");
    0
}
