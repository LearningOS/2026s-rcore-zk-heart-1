//! Process management syscalls
use crate::mm::{PTEFlags, PageTable, VirtAddr};
use crate::task::{
    change_program_brk, exit_current_and_run_next, get_current_syscall_count, mmap_current,
    munmap_current, suspend_current_and_run_next,
};
use crate::{mm::translated_byte_buffer, task::current_user_token, timer::get_time_us};

use core::mem::size_of;
use core::slice::from_raw_parts;

#[repr(C)]
#[derive(Debug)]
pub struct TimeVal {
    pub sec: usize,  //多少秒
    pub usec: usize, //多少微秒
}

/// task exits and submit an exit code
pub fn sys_exit(_exit_code: i32) -> ! {
    trace!("kernel: sys_exit");
    exit_current_and_run_next();
    panic!("Unreachable in sys_exit!");
}

/// current task gives up resources for other tasks
pub fn sys_yield() -> isize {
    trace!("kernel: sys_yield");
    suspend_current_and_run_next();
    0
}

/// YOUR JOB: get time with second and microsecond
/// HINT: You might reimplement it with virtual memory management.
/// HINT: What if [`TimeVal`] is splitted by two pages ?
pub fn sys_get_time(ts: *mut TimeVal, _tz: usize) -> isize {
    trace!("kernel: sys_get_time");

    let us = get_time_us(); //get current time (microseconds)
    let time_val = TimeVal {
        sec: us / 1_000_000,
        usec: us % 1_000_000,
    };

    let token = current_user_token(); //获取当前用户任务的页表

    //内存复制通常以字节为单位进行，所以把 time_val 临时看成一个字节切片
    let time_val_bytes = unsafe {
        from_raw_parts(
            &time_val as *const TimeVal as *const u8,
            size_of::<TimeVal>(),
        )
    };

    //翻译用户缓冲区
    let user_buffers = translated_byte_buffer(token, ts as *const u8, size_of::<TimeVal>());

    //逐段复制
    let mut copied = 0; //how much bytes had been copied
    for user_buffer in user_buffers {
        let len = user_buffer.len();
        user_buffer.copy_from_slice(&time_val_bytes[copied..copied + len]);
        copied += len;
    }

    0
}

/// TODO: Finish sys_trace to pass testcases
/// HINT: You might reimplement it with virtual memory management.
pub fn sys_trace(trace_request: usize, id: usize, data: usize) -> isize {
    trace!("kernel: sys_trace");

    match trace_request {
        // read a byte from user sapce
        0 => {
            let va = VirtAddr::from(id); //把id转换成虚拟地址
            let page_table = PageTable::from_token(current_user_token()); //得到当前任务的用户页表

            //查询页表项
            let pte = match page_table.translate(va.floor()) {
                //va.floor()是虚拟页号
                Some(pte) => pte,
                None => return -1,
            };

            //检查权限
            if !pte.is_valid() || !pte.readable() || !pte.flags().contains(PTEFlags::U) {
                return -1;
            }

            //真正读byte
            let value = pte.ppn().get_bytes_array()[va.page_offset()];

            value as isize
        }
        // write a byte into user space
        1 => {
            let va = VirtAddr::from(id);
            let page_table = PageTable::from_token(current_user_token());
            let pte = match page_table.translate(va.floor()) {
                Some(pte) => pte,
                None => return -1,
            };

            if !pte.is_valid() || !pte.writable() || !pte.flags().contains(PTEFlags::U) {
                return -1;
            }

            // write only the lowest eight bits of data
            pte.ppn().get_bytes_array()[va.page_offset()] = data as u8;

            0
        }
        // count process number
        2 => match get_current_syscall_count(id) {
            Some(count) => count as isize,
            None => -1,
        },
        _ => -1,
    }
}

// YOUR JOB: Implement mmap.
pub fn sys_mmap(start: usize, len: usize, prot: usize) -> isize {
    trace!("kernel: sys_mmap");
    mmap_current(start, len, prot)
}

// YOUR JOB: Implement munmap.
pub fn sys_munmap(start: usize, len: usize) -> isize {
    trace!("kernel: sys_munmap");
    munmap_current(start, len)
}
/// change data segment size
pub fn sys_sbrk(size: i32) -> isize {
    trace!("kernel: sys_sbrk");
    if let Some(old_brk) = change_program_brk(size) {
        old_brk as isize
    } else {
        -1
    }
}
