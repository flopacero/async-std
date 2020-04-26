pub(crate) use self::imp::AtomicU64;

// `AtomicU64` can only be used on targets with `target_has_atomic` is 64 or greater.
// Once `cfg_target_has_atomic` feature is stable, we can replace it with
// `#[cfg(target_has_atomic = "64")]`.
// Refs: https://github.com/rust-lang/rust/tree/master/src/librustc_target
#[cfg(not(any(target_arch = "arm", target_arch = "mips", target_arch = "powerpc")))]
mod imp {
    pub(crate) use std::sync::atomic::AtomicU64;
}

#[cfg(any(target_arch = "arm", target_arch = "mips", target_arch = "powerpc"))]
mod imp {
    use std::sync::atomic::Ordering;
    use std::sync::Mutex;

    pub(crate) struct AtomicU64(Mutex<u64>);

    impl AtomicU64 {
        pub(crate) const fn new(val: u64) -> Self {
            Self(Mutex::new(val))
        }

        pub(crate) fn load(&self, _: Ordering) -> u64 {
            *self.0.lock().unwrap()
        }

        pub(crate) fn fetch_add(&self, val: u64, _: Ordering) -> u64 {
            let lock = self.0.lock().unwrap();
            let prev = *lock;
            *lock += val;
            prev
        }

        pub(crate) fn fetch_sub(&self, val: u64, _: Ordering) -> u64 {
            let lock = self.0.lock().unwrap();
            let prev = *lock;
            *lock -= val;
            prev
        }
    }
}
