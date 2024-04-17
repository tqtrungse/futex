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
    ffi::c_void,
    sync::atomic::AtomicU32,
};

// On macOS, atomic wait/wake functionality is not available through
// any public/stable C interface, but is available through libc++.
//
// The libc++ functions declared below are not publicly documented,
// but they are part of the stable ABI.
//
// These functions are used to implement C++20's std::atomic<T>::{wait,
// notify*} which are defined in libc++'s headers, resulting in C++ binaries
// that dynamically link these symbols. So, it's safe to rely on these from
// Rust as well, as long as we also link libc++.
//
// These exist since macOS 11, iOS 14, and watchOS 7.

#[link(name = "c++")]
extern "C" {
    // std::__1::__libcpp_atomic_monitor(void const volatile*)
    #[link_name = "_ZNSt3__123__libcpp_atomic_monitorEPVKv"]
    fn __libcpp_atomic_monitor(ptr: *const c_void) -> i64;

    // std::__1::__libcpp_atomic_wait(void const volatile*, long long)
    #[link_name = "_ZNSt3__120__libcpp_atomic_waitEPVKvx"]
    fn __libcpp_atomic_wait(ptr: *const c_void, monitor: i64);

    // std::__1::__cxx_atomic_notify_one(void const volatile*)
    #[link_name = "_ZNSt3__123__cxx_atomic_notify_oneEPVKv"]
    fn __cxx_atomic_notify_one(ptr: *const c_void);

    // std::__1::__cxx_atomic_notify_all(void const volatile*)
    #[link_name = "_ZNSt3__123__cxx_atomic_notify_allEPVKv"]
    fn __cxx_atomic_notify_all(ptr: *const c_void);

    // Next to the four `void const volatile*` functions above, there are also
    // overloads for `__cxx_atomic_contention_t const volatile*` (where
    // `__cxx_atomic_contention_t` is basically `AtomicI64`), specifically for
    // 64-bit atomics. Those don't use a separate futex in a table, but instead
    // use the atomic itself as the futex, which can be more efficient.
    // However, because of the monitor+wait API design here, that can result in
    // missed wakeups, due to the ABA problem. So, we simply don't use those.
    // See https://reviews.llvm.org/D114119#3193088
}

#[inline(always)]
pub(crate) fn wait_until(_: &AtomicU32, _: u32, _: u32) -> bool {
    std::panic!("Not implement yet");
}

#[inline(always)]
pub(crate) fn wait(atom: &AtomicU32, expected: u32) -> bool {
    let ptr: *const AtomicU32 = atom;
    // The 'monitor' is just the notification counter associated
    // with the address of the atomic.
    let monitor = unsafe { __libcpp_atomic_monitor(ptr.cast()) };

    unsafe { __libcpp_atomic_wait(ptr.cast(), monitor) };
    true
}

#[inline(always)]
pub(crate) fn wake_one(ptr: *const AtomicU32) -> bool {
    unsafe { __cxx_atomic_notify_one(ptr.cast()) };
    true
}

#[inline(always)]
pub(crate) fn wake_all(ptr: *const AtomicU32) -> bool {
    unsafe { __cxx_atomic_notify_all(ptr.cast()) };
    true
}