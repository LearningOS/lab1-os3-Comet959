//! Types related to task management

use super::TaskContext;
use core::fmt::{Display, Formatter, Result};  
use crate::config::MAX_SYSCALL_NUM;
// use alloc::boxed::Box;
#[derive(Copy, Clone)]
/// task control block structure
pub struct TaskControlBlock {
    pub task_status: TaskStatus, // 任务状态
    pub task_cx: TaskContext, // 任务状态现场
    // LAB1: Add whatever you need about the Task.
    // pub task_syscall_times: [u32; 5], // 系统调用数
    // pub task_info_ptr: &task_info,
    // pub task_syscall_times: &[u32; 500],
    // pub task_syscall_times: Box<[u32; 500]>,
    // pub task_nr: usize,
}

#[derive(Copy, Clone, PartialEq)]
/// task status: UnInit, Ready, Running, Exited
pub enum TaskStatus {
    UnInit,
    Ready,
    Running,
    Exited,
}

impl Display for TaskStatus {
    fn fmt(&self, f: &mut Formatter) -> Result {
        match self {
            TaskStatus::UnInit => write!(f, "UnInit"),
            TaskStatus::Ready => write!(f, "Ready"),
            TaskStatus::Running => write!(f, "Running"),
            TaskStatus::Exited => write!(f, "Exited"),
        }
    }
}

// impl Default for TaskControlBlock {
//     fn default() -> TaskControlBlock {
//         TaskControlBlock {
//             task_status: TaskStatus::UnInit,
//             task_cx: TaskContext {
//                 ra: 0,
//                 sp: 0,
//                 s: [0; 12],
//             },
//             task_syscall_times: [0; MAX_SYSCALL_NUM],
//         }
//     }
// }