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
    intrinsics::unlikely,
    sync::atomic::AtomicU32,
};

use windows_sys::{
    Win32::{
        Foundation::FALSE,
        System::Threading::{INFINITE, WaitOnAddress, WakeByAddressAll, WakeByAddressSingle}
    },
};


#[inline]
pub(crate) fn wait_until(atom: &AtomicU32, expected: u32, millis: u32) -> bool {
    let atom_ptr: *const AtomicU32 = atom;
    let expected_ptr: *const u32 = &expected;

    // Be able to spurious wakeup.
    // https://learn.microsoft.com/en-us/windows/win32/api/synchapi/nf-synchapi-waitonaddress
    let success = unsafe {
        WaitOnAddress(
            atom_ptr.cast(),
            expected_ptr.cast(),
            4,
            millis
        )
    };
    if unlikely(success == FALSE) {
        return false;
    }
    true
}

#[inline]
pub(crate) fn wait(atom: &AtomicU32, expected: u32) -> bool {
    wait_until(atom, expected, INFINITE)
}

#[inline]
pub(crate) fn wake_one(atom_ptr: *const AtomicU32) -> bool {
    unsafe { WakeByAddressSingle(atom_ptr.cast()) };
    true
}

#[inline]
pub(crate) fn wake_all(atom_ptr: *const AtomicU32) -> bool {
    unsafe { WakeByAddressAll(atom_ptr.cast()) };
    false
}