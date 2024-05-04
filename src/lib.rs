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

//! The implementation is based on `atomic-wait`.
//!
//! Source: '<https://github.com/m-ou-se/atomic-wait/blob/main>'
//!
//! Copyright & License:
//!   - `BSD 2-Clause License`
//!         '<https://opensource.org/license/bsd-2-clause>'

use std::sync::atomic::AtomicU32;

/// Windows 8+, Windows Server 2012+
#[cfg(target_os = "windows")]
#[path = "windows.rs"]
mod platform;

/// Unix 2.6.22+
#[cfg(any(target_os = "linux", target_os = "android"))]
#[path = "unix.rs"]
mod platform;

/// FreeBSD 11+
#[cfg(target_os = "freebsd")]
#[path = "freebsd.rs"]
mod platform;

/// macOS 11+, iOS 14+, watchOS 7+
#[cfg(any(target_os = "macos", target_os = "ios", target_os = "watchos"))]
#[path = "macos.rs"]
mod platform;

/// If the value is `value`, wait until woken up or expired.
///
/// This function might also return spuriously,
/// without a corresponding wake operation.
#[inline(always)]
pub fn wait_until(atom: &AtomicU32, expected: u32, millis: u32) -> bool {
    platform::wait_until(atom, expected, millis)
}

/// If the value is `value`, wait until woken up.
///
/// This function might also return spuriously,
/// without a corresponding wake operation.
#[inline(always)]
pub fn wait(atom: &AtomicU32, expected: u32) -> bool {
    platform::wait(atom, expected)
}

/// Wake one thread that is waiting on this atomic.
///
/// It's okay if the pointer dangles or is null.
#[inline(always)]
pub fn wake_one(atom: *const AtomicU32) -> bool {
    platform::wake_one(atom)
}

/// Wake all threads that are waiting on this atomic.
///
/// It's okay if the pointer dangles or is null.
#[inline(always)]
pub fn wake_all(atom: *const AtomicU32) -> bool {
    platform::wake_all(atom)
}

#[cfg(test)]
mod test {
    #[test]
    fn wake_null() {
        crate::wake_one(std::ptr::null::<std::sync::atomic::AtomicU32>());
        crate::wake_all(std::ptr::null::<std::sync::atomic::AtomicU32>());
    }

    #[test]
    fn wake_nothing() {
        let a = std::sync::atomic::AtomicU32::new(0);
        crate::wake_one(&a);
        crate::wake_all(&a);
    }

    #[test]
    fn wait_wake() {
        let t = std::time::Instant::now();
        let a = std::sync::atomic::AtomicU32::new(0);
        std::thread::scope(|s| {
            s.spawn(|| {
                std::thread::sleep(std::time::Duration::from_millis(100));
                a.store(1, std::sync::atomic::Ordering::Relaxed);
                crate::wake_one(&a);
            });
            while a.load(std::sync::atomic::Ordering::Relaxed) == 0 {
                crate::wait(&a, 0);
            }
            assert_eq!(a.load(std::sync::atomic::Ordering::Relaxed), 1);
            assert!((90..400).contains(&t.elapsed().as_millis()));
        });
    }

    // macOS will be blocked forever.
    #[test]
    #[cfg(all(not(target_os = "macos"), not(target_os = "ios"), not(target_os = "watchos")))]
    fn wait_unexpected() {
        let t = std::time::Instant::now();
        let a = std::sync::atomic::AtomicU32::new(0);
        crate::wait(&a, 1);
        assert!(t.elapsed().as_millis() < 100);
    }

    // macOS is not supported timeout.
    #[test]
    #[cfg(all(not(target_os = "macos"), not(target_os = "ios"), not(target_os = "watchos")))]
    fn wait_timeout() {
        let t = std::time::Instant::now();
        let a = std::sync::atomic::AtomicU32::new(0);

        crate::wait_until(&a, 0, 1000);
        assert!((900..1030).contains(&t.elapsed().as_millis()));
    }
}