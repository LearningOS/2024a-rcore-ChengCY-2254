//! Implementation of syscalls
//!
//! The single entry point to all system calls, [`syscall()`], is called
//! whenever userspace wishes to perform a system call using the `ecall`
//! instruction. In this case, the processor raises an 'Environment call from
//! U-mode' exception, which is handled as one of the cases in
//! [`crate::trap::trap_handler`].
//!
//! For clarity, each single syscall is implemented as its own function, named
//! `sys_` then the name of the syscall. You can find functions like this in
//! submodules, and you should also implement syscalls this way.

/// write syscall
const SYSCALL_WRITE: usize = 64;
/// exit syscall
const SYSCALL_EXIT: usize = 93;
/// yield syscall
const SYSCALL_YIELD: usize = 124;
/// gettime syscall
const SYSCALL_GET_TIME: usize = 169;
/// taskinfo syscall
const SYSCALL_TASK_INFO: usize = 410;

mod fs;
mod process;

use crate::task::TaskStatus;
use crate::{
    config::{MAX_APP_NUM, MAX_SYSCALL_NUM},
    sync::UPSafeCell,
    task::{current_task_id, current_task_status},
    timer::get_time_ms,
};
use fs::*;
use lazy_static::lazy_static;
use process::*;

/// handle syscall exception with `syscall_id` and other arguments
pub fn syscall(syscall_id: usize, args: [usize; 3]) -> isize {
    // TASK_INFO_MANAGER.update_syscall_by_task(syscall_id);
    match syscall_id {
        SYSCALL_WRITE => sys_write(args[0], args[1] as *const u8, args[2]),
        SYSCALL_EXIT => sys_exit(args[0] as i32),
        SYSCALL_YIELD => sys_yield(),
        SYSCALL_GET_TIME => sys_get_time(args[0] as *mut TimeVal, args[1]),
        SYSCALL_TASK_INFO => sys_task_info(args[0] as *mut TaskInfo),
        _ => panic!("Unsupported syscall_id: {}", syscall_id),
    }
}

/// 存储所有的进程时间和系统调用次数
pub struct TaskInfoManager {
    /// task list
    tasks: UPSafeCell<[TaskInfoInner; MAX_APP_NUM]>,
}
/// Wapper [`TaskInfo`] and provide start time
#[derive(Copy, Clone)]
pub struct TaskInfoInner {
    /// time for milliseconds
    start: usize,
    /// Task info
    task_info: TaskInfo,
}
lazy_static! {
    /// Global variable: TASK_INFO_MANAGER
    pub static ref TASK_INFO_MANAGER:TaskInfoManager = {
        let tasks:[TaskInfoInner;16] = [TaskInfoInner{
            start:0,
            task_info:TaskInfo{
                status:TaskStatus::UnInit,
                syscall_times:[0;MAX_SYSCALL_NUM],
                time:0,
            }
        };MAX_APP_NUM];
        unsafe{
          TaskInfoManager{
                tasks:UPSafeCell::new(tasks)
            }
        }
    };
}

impl TaskInfoManager {
    /// use TaskManager call, update the first run of time and status
    fn update_time_ms(&self, task_id: usize, start_time: usize) {
        let mut tasks = self.tasks.exclusive_access();
        if tasks[task_id].start == 0 {
            tasks[task_id].start = start_time;
        }
        tasks[task_id].task_info.status = current_task_status();
    }
    /// use TaskInfoManager call, save syscall call and count
    fn update_syscall_by_task(&self, syscall_id: usize) {
        let task_id = current_task_id();
        let mut tasks = self.tasks.exclusive_access();
        tasks[task_id].task_info.status = current_task_status();
        let task_info = &mut tasks[task_id].task_info;
        task_info.syscall_times[syscall_id] += 1;
    }
    /// use syscall [`crate::syscall::process::sys_task_info`] call, get current task infomation
    fn get_task_info(&self) -> *mut TaskInfo {
        let task_id = current_task_id();
        let mut tasks = self.tasks.exclusive_access();
        let start = tasks[task_id].start;
        let end = get_time_ms();
        tasks[task_id].task_info.time = end - start;
        &mut tasks[task_id].task_info as *mut TaskInfo
    }
}
/// Wapper [`TaskInfoManager::update_time_ms`]
pub fn update_task_info() {
    let task_id = current_task_id();
    let start_time_ms = get_time_ms();
    TASK_INFO_MANAGER.update_time_ms(task_id, start_time_ms)
}
/// Wapper [`TaskInfoManager::update_syscall_by_task`]
pub fn update_syscall_by_task(syscall_id: usize) {
    TASK_INFO_MANAGER.update_syscall_by_task(syscall_id)
}
/// Wapper [`TaskInfoManager::get_task_info`]
pub fn get_task_info() -> *mut TaskInfo {
    TASK_INFO_MANAGER.get_task_info()
}
