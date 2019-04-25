#![cfg_attr(feature="auto-traits", feature(optin_builtin_traits))]
#![cfg_attr(not(feature="std"), no_std)]
//! The interprocess-traits crate provides type traits to annotate types which have certain
//! properties when used in a multiprocess environment.


#[cfg(feature="std")]
use std as core;

use core::sync::atomic::*;
use core::num::*;

#[cfg(feature="auto-traits")]
macro_rules! maybe_auto_trait {
    ($doctext:literal pub unsafe trait $traitname:ident: $deps:ident {}) => {
        // NB: no supertrait, built-in traits cannot have them!
        #[doc=$doctext]
        pub unsafe auto trait $traitname {}
    }
}

#[cfg(not(feature="auto-traits"))]
macro_rules! maybe_auto_trait {
    ($doctext:literal pub unsafe trait $traitname:ident: $deps:ident {}) => {
        #[doc=$doctext]
        pub unsafe trait $traitname: $deps {}
    }
}

maybe_auto_trait! {
"Types which can be transferred across process boundary.

An example of non-`ProcSend` type would be any type that contains pointers to values in \
a process’ address space. Instead a `ProcSend` type would use indices which the \
processes would know how to convert into their own address space.

In general, for any type to implement `ProcSend`, it must be [`Send`], as threads \
conceptually are the same as processes which share the same address space.

Unlike [`Send`], `ProcSend` is not an auto-trait by default, because auto-traits are not stable. \
Unless the `auto_traits` feature is enabled, it will be necessary to implement this \
trait manually.

Finally, currently `ProcSend` is only implemented for the built-in types which have \
specified type layout and are ffi-safe. This is a conservative default as there is \
no guarantee that the two interacting processes will be compiled with a same version \
of Rust or are Rust at all."
pub unsafe trait ProcSend: Send {}
}

maybe_auto_trait! {
"Types for which it is safe to share references between processes.

Much like [`ProcSend`] is a cross-process [`Send`], [`ProcSync`] is a cross-process [`Sync`].

In general, for any type to implement `ProcSync`, it must be [`Sync`], as threads conceptually \
are the same as processes which sahre the same address space.

Unlike [`Sync`], `ProcSync` is not an auto-trait by default, because auto-traits are not stable. \
Unless the `auto_traits` feature is enabled, it will be necessary to implement this \
trait manually."
pub unsafe trait ProcSync: Sync {}
}

macro_rules! implement_marker_for {
    (unsafe impl $trait:ident for $($t:ty),*) => {
        $(unsafe impl $trait for $t {})*
    };
    (unsafe impl !$trait:ident for $($t:ty),*) => {
        $(unsafe impl !$trait for $t {})*
    };
}

implement_marker_for! {
    unsafe impl ProcSend for
    u8, u16, u32, u64, u128, usize,
    i8, i16, i32, i64, i128, isize,
    f32, f64,
    bool, char, str,
    AtomicU8, AtomicU16, AtomicU32, AtomicU64, AtomicUsize,
    AtomicI8, AtomicI16, AtomicI32, AtomicI64, AtomicIsize,
    AtomicBool,
    NonZeroU8, NonZeroU16, NonZeroU32, NonZeroU64, NonZeroU128, NonZeroUsize,
    NonZeroI8, NonZeroI16, NonZeroI32, NonZeroI64, NonZeroI128, NonZeroIsize,
    core::ffi::c_void
}

unsafe impl<T: ProcSend> ProcSend for core::cell::RefCell<T> {}
unsafe impl<T: ProcSend> ProcSend for core::cell::Cell<T> {}
unsafe impl<T: ProcSend> ProcSend for core::cell::UnsafeCell<T> {}
unsafe impl<T: ProcSend> ProcSend for core::mem::ManuallyDrop<T> {}

#[cfg(feature="auto-traits")]
impl<T> !ProcSend for *const T {}
#[cfg(feature="auto-traits")]
impl<T> !ProcSend for *mut T {}
#[cfg(feature="auto-traits")]
impl<T> !ProcSend for AtomicPtr<T> {}
#[cfg(feature="auto-traits")]
impl<T> !ProcSend for &T {}
#[cfg(feature="auto-traits")]
impl<T> !ProcSend for &mut T {}
#[cfg(all(feature="auto-traits", feature="std"))]
impl !ProcSend for std::net::TcpListener {}
#[cfg(all(feature="auto-traits", feature="std"))]
impl !ProcSend for std::net::TcpStream {}
#[cfg(all(feature="auto-traits", feature="std"))]
impl !ProcSend for std::net::UdpSocket {}
#[cfg(all(feature="auto-traits", feature="std", unix))]
impl !ProcSend for std::os::unix::net::UnixDatagram {}
#[cfg(all(feature="auto-traits", feature="std", unix))]
impl !ProcSend for std::os::unix::net::UnixListener {}
#[cfg(all(feature="auto-traits", feature="std", unix))]
impl !ProcSend for std::os::unix::net::UnixStream {}
#[cfg(all(feature="auto-traits", feature="std"))]
impl !ProcSend for std::fs::File {}
#[cfg(all(feature="auto-traits", feature="std"))]
impl !ProcSend for std::process::Child {}
#[cfg(all(feature="auto-traits", feature="std"))]
impl !ProcSend for std::process::ChildStderr {}
#[cfg(all(feature="auto-traits", feature="std"))]
impl !ProcSend for std::process::ChildStdin {}
#[cfg(all(feature="auto-traits", feature="std"))]
impl !ProcSend for std::process::ChildStdout {}
#[cfg(all(feature="auto-traits", feature="std"))]
impl !ProcSend for std::process::Stdio {}
#[cfg(all(feature="auto-traits", feature="std"))]
impl !ProcSend for std::thread::ThreadId {}


implement_marker_for! {
    unsafe impl ProcSync for
    u8, u16, u32, u64, u128, usize,
    i8, i16, i32, i64, i128, isize,
    f32, f64,
    bool, char, str,
    AtomicU8, AtomicU16, AtomicU32, AtomicU64, AtomicUsize,
    AtomicI8, AtomicI16, AtomicI32, AtomicI64, AtomicIsize,
    AtomicBool,
    NonZeroU8, NonZeroU16, NonZeroU32, NonZeroU64, NonZeroU128, NonZeroUsize,
    NonZeroI8, NonZeroI16, NonZeroI32, NonZeroI64, NonZeroI128, NonZeroIsize,
    core::ffi::c_void
}

unsafe impl<T: ProcSync> ProcSync for core::mem::ManuallyDrop<T> {}

#[cfg(feature="auto-traits")]
impl<T> !ProcSync for core::cell::Cell<T> {}
#[cfg(feature="auto-traits")]
impl<T> !ProcSync for core::cell::UnsafeCell<T> {}
#[cfg(feature="auto-traits")]
impl<T> !ProcSync for *const T {}
#[cfg(feature="auto-traits")]
impl<T> !ProcSync for *mut T {}
#[cfg(feature="auto-traits")]
impl<T> !ProcSync for AtomicPtr<T> {}
#[cfg(feature="auto-traits")]
impl<T> !ProcSync for core::ptr::NonNull<T> {}
#[cfg(feature="auto-traits")]
impl<T> !ProcSync for &T {}
#[cfg(feature="auto-traits")]
impl<T> !ProcSync for &mut T {}
#[cfg(all(feature="auto-traits", feature="std"))]
impl !ProcSync for std::net::TcpListener {}
#[cfg(all(feature="auto-traits", feature="std"))]
impl !ProcSync for std::net::TcpStream {}
#[cfg(all(feature="auto-traits", feature="std"))]
impl !ProcSync for std::net::UdpSocket {}
#[cfg(all(feature="auto-traits", feature="std", unix))]
impl !ProcSync for std::os::unix::net::UnixDatagram {}
#[cfg(all(feature="auto-traits", feature="std", unix))]
impl !ProcSync for std::os::unix::net::UnixListener {}
#[cfg(all(feature="auto-traits", feature="std", unix))]
impl !ProcSync for std::os::unix::net::UnixStream {}
#[cfg(all(feature="auto-traits", feature="std"))]
impl !ProcSync for std::fs::File {}
#[cfg(all(feature="auto-traits", feature="std"))]
impl !ProcSync for std::process::Child {}
#[cfg(all(feature="auto-traits", feature="std"))]
impl !ProcSync for std::process::ChildStderr {}
#[cfg(all(feature="auto-traits", feature="std"))]
impl !ProcSync for std::process::ChildStdin {}
#[cfg(all(feature="auto-traits", feature="std"))]
impl !ProcSync for std::process::ChildStdout {}
#[cfg(all(feature="auto-traits", feature="std"))]
impl !ProcSync for std::process::Stdio {}
#[cfg(all(feature="auto-traits", feature="std"))]
impl !ProcSync for std::thread::ThreadId {}
