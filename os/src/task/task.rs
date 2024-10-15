//! Types related to task management

use super::TaskContext;
use crate::config::MAX_SYSCALL_NUM;

/// The task control block (TCB) of a task.
#[derive(Copy, Clone)]
pub struct TaskControlBlock {
    /// The task status in it's lifecycle
    pub task_status: TaskStatus,
    /// The task context
    pub task_cx: TaskContext,
    /// Task first dispatch time in milliseconds.
    pub start_time:usize,
    /// Syscall count
    pub syscall_times:[u32;MAX_SYSCALL_NUM],
}

/// The status of a task
#[derive(Copy, Clone, PartialEq, Debug)]
pub enum TaskStatus {
    /// uninitialized
    UnInit,
    /// ready to run
    Ready,
    /// running
    Running,
    /// exited
    Exited,
}

/// See [`crate::syscall::process::TaskInfo`], this struct is a background support
#[derive(Copy, Clone, Debug)]
pub struct TaskInfoInner {
    /// Task status in it's life cycle
    pub status: TaskStatus,
    /// count are syscall by task
    pub syscall_times: [u32; MAX_SYSCALL_NUM],
    /// Total running time of task in milliseconds
    pub time:usize
}