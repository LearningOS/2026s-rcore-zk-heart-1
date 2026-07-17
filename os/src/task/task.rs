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
    /// Per-task syscall invocation counters indexed by syscall id
    pub syscall_times: [u32; MAX_SYSCALL_NUM],
    /// The first time when this task is scheduled to run
    pub first_run_time: Option<usize>,
}

/// The status of a task
#[repr(C)]
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

/// Information of the current task returned to user space.
#[repr(C)]
#[derive(Copy, Clone)]
pub struct TaskInfo {
    /// Current status of the task.
    pub status: TaskStatus,
    /// Syscall invocation counters indexed by syscall id.
    pub syscall_times: [u32; MAX_SYSCALL_NUM],
    /// Milliseconds elapsed since the task was first scheduled.
    pub time: usize,
}
