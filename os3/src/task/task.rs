//! Types related to task management

use super::TaskContext;
use core::fmt::{Display, Formatter, Result};  
use crate::config::MAX_SYSCALL_NUM;
#[derive(Copy, Clone)]
/// task control block structure
pub struct TaskControlBlock {
    pub task_status: TaskStatus, // 任务状态
    pub task_cx: TaskContext, // 任务状态现场
    // LAB1: Add whatever you need about the Task.
    pub task_time: isize,
    pub task_start_time: isize,
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