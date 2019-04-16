use std::sync::atomic::*;
use std::num::*;

/// Types which can be transferred across process boundary.
///
/// An example of non-`ProcSend` type would be any type that contains pointers to values in a
/// process’ address space. Instead a `ProcSend` type would use indices which the processes would
/// know how to convert into their own address space.
///
/// In general, for any type to implement `ProcSend`, it must be [`Send`], as threads conceptually
/// are the same as processes which share the same address space.
///
/// Unlike [`Send`], `ProcSend` is not an auto-trait, because auto-traits are not stable. Instead,
/// use `derive(ProcSend)` to implement `ProcSend` for the types which you want to transfer across
/// a process boundary. This trait may become an auto trait in the future.
///
/// Finally, currently `ProcSend` is only implemented for types which have specified type layout
/// and are ffi-safe. This is a conservative default as there is no guarantee that the two
/// interacting processes will be compiled with a same version of Rust or are Rust at all.
pub unsafe trait ProcSend: Send {}

/// Types for which it is safe to share references between processes.
///
/// Much like [`ProcSend`] is a cross-process [`Send`], [`ProcSync`] is a cross-process [`Sync`].
///
/// In general, for any type to implement `ProcSync`, it must be [`Sync`], as threads conceptually
/// are the same as processes which sahre the same address space.
///
/// Unlike [`Sync`], `ProcSync` is not an auto-trait, because auto-traits are not stable. Instead,
/// use `derive(ProcSync)` to implement `ProcSync` for the types which you want to share references
/// to across a process boundary. This trait may become an auto trait in the future.
pub unsafe trait ProcSync: Sync {}

/// Assert that the contained type is `ProcSend`.
///
/// This type has the same representation as `T`.
#[repr(transparent)]
pub struct AssertProcSend<T>(pub T);

/// Assert that the contained type is `ProcSync`.
///
/// This type has the same representation as `T`.
#[repr(transparent)]
pub struct AssertProcSync<T>(pub T);

macro_rules! implement_marker_for {
    (unsafe impl $trait:ident for $($t:ty),*) => {
        $(unsafe impl $trait for $t {})*
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
    std::ffi::c_void
}

unsafe impl<T: ProcSend> ProcSend for std::cell::Cell<T> {}
unsafe impl<T: ProcSend> ProcSend for std::cell::UnsafeCell<T> {}
unsafe impl<T: ProcSend> ProcSend for std::mem::ManuallyDrop<T> {}
unsafe impl<T: Send> ProcSend for AssertProcSend<T> {}

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
    std::ffi::c_void
}

unsafe impl<T: ProcSync> ProcSync for std::mem::ManuallyDrop<T> {}
unsafe impl<T: Sync> ProcSync for AssertProcSync<T> {}
