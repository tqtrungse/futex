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

use omango_util::hint::unlikely;

#[inline]
pub(crate) fn wait_until(atom: &AtomicU32, expected: u32, millis: u32) -> bool {
    let ptr: *const AtomicU32 = atom;
    let code = unsafe {
        libc::_umtx_op(
            ptr as *mut libc::c_void,
            libc::UMTX_OP_WAIT_UINT_PRIVATE | libc::UMTX_OP_WAIT_UINT_PRIVATE_TIMEOUT,
            expected as libc::c_ulong,
            core::ptr::null_mut(),
            to_duration(millis),
        )
    };
    if unlikely(code < 0) {
        return false;
    }
    true
}

#[inline]
pub(crate) fn wait(atom: &AtomicU32, expected: u32) -> bool {
    let ptr: *const AtomicU32 = atom;
    let code = unsafe {
        libc::_umtx_op(
            ptr as *mut libc::c_void,
            libc::UMTX_OP_WAIT_UINT_PRIVATE,
            expected as libc::c_ulong,
            core::ptr::null_mut(),
            core::ptr::null_mut(),
        )
    };
    if unlikely(code < 0) {
        return false;
    }
    true
}

#[inline]
pub(crate) fn wake_one(ptr: *const AtomicU32) -> bool {
    let code = unsafe {
        libc::_umtx_op(
            ptr as *mut libc::c_void,
            libc::UMTX_OP_WAKE_PRIVATE,
            1 as libc::c_ulong,
            core::ptr::null_mut(),
            core::ptr::null_mut(),
        )
    };
    if unlikely(code < 0) {
        return false;
    }
    true
}

#[inline]
pub(crate) fn wake_all(ptr: *const AtomicU32) -> Option<Error> {
    let code = unsafe {
        libc::_umtx_op(
            ptr as *mut libc::c_void,
            libc::UMTX_OP_WAKE_PRIVATE,
            i32::MAX as libc::c_ulong,
            core::ptr::null_mut(),
            core::ptr::null_mut(),
        )
    };
    if unlikely(code < 0) {
        return false;
    }
    true
}

#[inline]
fn to_duration(millis: u32) -> *mut libc::c_void {
    let now = SystemTime::now();
    let duration_since_epoch = now.
        duration_since(SystemTime::UNIX_EPOCH).
        expect("Time went backwards");

    let timeout_ns = duration_since_epoch.as_nanos() + (millis as u128 * 1_000_000);
    let timeout = timeout_ns as u64; // Cast to u64

    &timeout as *const _ as *mut libc::c_void
}