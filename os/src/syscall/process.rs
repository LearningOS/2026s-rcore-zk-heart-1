//! Process management syscalls
use crate::{
    task::{exit_current_and_run_next, get_current_syscall_count, suspend_current_and_run_next},
    timer::get_time_us,
};
use core::ptr::{read_volatile, write_volatile};

#[repr(C)]
#[derive(Debug)]
pub struct TimeVal {
    pub sec: usize,
    pub usec: usize,
}

/// task exits and submit an exit code
pub fn sys_exit(exit_code: i32) -> ! {
    trace!("[kernel] Application exited with code {}", exit_code);
    exit_current_and_run_next();
    panic!("Unreachable in sys_exit!");
}

/// current task gives up resources for other tasks
pub fn sys_yield() -> isize {
    trace!("kernel: sys_yield");
    suspend_current_and_run_next();
    0
}

/// get time with second and microsecond
pub fn sys_get_time(ts: *mut TimeVal, _tz: usize) -> isize {
    trace!("kernel: sys_get_time");
    let us = get_time_us();
    unsafe {
        *ts = TimeVal {
            sec: us / 1_000_000,
            usec: us % 1_000_000,
        };
    }
    0
}

/// trace current task information
///
/// trace_request:
/// - 0: read one byte from user virtual address `id` (`*const u8`)
/// - 1: write one byte (`data as u8`) to user virtual address `id` (`*mut u8`)
/// - 2: query current task syscall count for syscall id `id`
pub fn sys_trace(trace_request: usize, id: usize, data: usize) -> isize {
    trace!("kernel: sys_trace");
    match trace_request {
        0 => unsafe { read_volatile(id as *const u8) as isize },
        1 => {
            unsafe {
                write_volatile(id as *mut u8, data as u8);
            }
            0
        }
        2 => get_current_syscall_count(id).map_or(-1, |times| times as isize),
        _ => -1,
    }
}
