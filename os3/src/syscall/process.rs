//! Process management syscalls

use crate::config::{MAX_SYSCALL_NUM};
use crate::task::{exit_current_and_run_next, suspend_current_and_run_next, TaskStatus, get_current_task_status};
use crate::timer::{get_time_us, get_time};
use crate::syscall::{SYSCALL_WRITE, SYSCALL_EXIT, SYSCALL_YIELD, SYSCALL_GET_TIME, SYSCALL_TASK_INFO};
use crate::task::{TaskControlBlock,get_current_task_nr, get_current_task_time};

#[repr(C)]
#[derive(Debug)]
pub struct TimeVal {
    pub sec: usize,
    pub usec: usize,
}

static mut syscall_times: [u32; MAX_SYSCALL_NUM] = [0; MAX_SYSCALL_NUM];

pub fn add_syscall_times(syscall_id: usize, current_task: usize) {
    unsafe {
        
        syscall_times[syscall_id] += 1;
    }
}
pub struct TaskInfo {
    status: TaskStatus,
    syscall_times: [u32; MAX_SYSCALL_NUM],
    time: usize,
}

/// task exits and submit an exit code
pub fn sys_exit(exit_code: i32) -> ! {
    info!("[kernel] Application exited with code {}", exit_code);
    exit_current_and_run_next();
    panic!("Unreachable in sys_exit!");
}

/// current task gives up resources for other tasks
pub fn sys_yield() -> isize {
    suspend_current_and_run_next();
    0
}

/// get time with second and microsecond
pub fn sys_get_time(ts: *mut TimeVal, _tz: usize) -> isize {
    let us = get_time_us();
    unsafe {
        *ts = TimeVal {
            sec: us / 1_000_000,
            usec: us % 1_000_000,
        };
    }
    0
}



/// YOUR JOB: Finish sys_task_info to pass testcases
pub fn sys_task_info(ti: *mut TaskInfo) -> isize {
    // unsafe {
    //     // println!("syscall_times:{:?}", syscall_times);
    //     println!("SYSCALL_WRITE:{}", syscall_times[SYSCALL_WRITE]); // 64
    //     println!("SYSCALL_EXIT:{}", syscall_times[SYSCALL_EXIT]); // 93
    //     println!("SYSCALL_YIELD:{}", syscall_times[SYSCALL_YIELD]); // 124
    //     println!("SYSCALL_GET_TIME:{}", syscall_times[SYSCALL_GET_TIME]); // 169
    //     println!("SYSCALL_TASK_INFO:{}", syscall_times[SYSCALL_TASK_INFO]); // 410
    // }
    // let time: usize = get_current_task_time() / 10000;

    unsafe {
        *ti = TaskInfo {
            status: get_current_task_status(),
            syscall_times: syscall_times,
            time: get_current_task_time() as usize,
        };
    }
    0
}
