// Copyright (c) 2024 Trung Tran <tqtrungse@gmail.com>
//
// Permission is hereby granted, free of charge, to any person obtaining a copy
// of this software and associated documentation files (the "Software"), to deal
// in the Software without restriction, including without limitation the rights
// to use, copy, modify, merge, publish, distribute, sublicense, and/or sell
// copies of the Software, and to permit persons to whom the Software is
// furnished to do so, subject to the following conditions:
//
// The above copyright notice and this permission notice shall be included in all
// copies or substantial portions of the Software.
//
// THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF ANY KIND, EXPRESS OR
// IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF MERCHANTABILITY,
// FITNESS FOR A PARTICULAR PURPOSE AND NONINFRINGEMENT. IN NO EVENT SHALL THE
// AUTHORS OR COPYRIGHT HOLDERS BE LIABLE FOR ANY CLAIM, DAMAGES OR OTHER
// LIABILITY, WHETHER IN AN ACTION OF CONTRACT, TORT OR OTHERWISE, ARISING FROM,
// OUT OF OR IN CONNECTION WITH THE SOFTWARE OR THE USE OR OTHER DEALINGS IN THE
// SOFTWARE.

use std::{
    sync::atomic::AtomicU32,
    time::{Duration, SystemTime},
};

use omango_util_rs::hint::unlikely;

#[inline]
pub(crate) fn wait_until(atom: &AtomicU32, expected: u32, millis: u32) -> bool {
    let code = unsafe {
        libc::syscall(
            libc::SYS_futex,
            atom,
            libc::FUTEX_WAIT_BITSET | libc::FUTEX_PRIVATE_FLAG | libc::FUTEX_CLOCK_REALTIME,
            expected,
            to_duration(millis),
            0,
            libc::FUTEX_BITSET_MATCH_ANY,
        )
    };

    if unlikely(code != 0) {
        return false;
    }
    true
}

#[inline]
pub(crate) fn wait(atom: &AtomicU32, expected: u32) -> bool {
    // Be able to spurious wakeup.
    // https://man7.org/linux/man-pages/man2/futex.2.html
    let code = unsafe {
        libc::syscall(
            libc::SYS_futex,
            atom,
            libc::FUTEX_WAIT | libc::FUTEX_PRIVATE_FLAG,
            expected,
            core::ptr::null::<libc::timespec>(),
        )
    };

    if unlikely(code != 0) {
        return false;
    }
    true
}

#[inline]
pub(crate) fn wake_one(ptr: *const AtomicU32) -> bool {
    let code = unsafe {
        libc::syscall(
            libc::SYS_futex,
            ptr,
            libc::FUTEX_WAKE | libc::FUTEX_PRIVATE_FLAG,
            1i32,
        )
    };
    if unlikely(code < 0) {
        return false;
    }
    true
}

#[inline]
pub(crate) fn wake_all(ptr: *const AtomicU32) -> bool {
    let code = unsafe {
        libc::syscall(
            libc::SYS_futex,
            ptr,
            libc::FUTEX_WAKE | libc::FUTEX_PRIVATE_FLAG,
            i32::MAX,
        )
    };
    if unlikely(code < 0) {
        return false;
    }
    true
}

fn to_duration(millis: u32) -> *const libc::c_void {
    // Convert timeout in milliseconds to Duration
    return match SystemTime::now().checked_add(Duration::from_millis(millis as u64)) {
        Some(time) => {
            let timespec = libc::timespec {
                tv_sec: time.duration_since(SystemTime::UNIX_EPOCH).unwrap().as_secs() as i64,
                // Convert nanoseconds to milliseconds
                tv_nsec: (time.duration_since(SystemTime::UNIX_EPOCH).unwrap().subsec_nanos() / 1_000_000) as i64,
            };
            &timespec as *const _ as *const libc::c_void
        }
        None => std::ptr::null(),
    };
}