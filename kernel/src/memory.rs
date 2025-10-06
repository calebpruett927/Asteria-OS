//! Asteria OS — Memory bring-up (dual mode).
//!
//! * **devhost (default)**: no-op stubs so CI/Codespaces build & run.
//! * **OS mode (`--no-default-features`)**: exposes a tiny,
//!   lock-free bump allocator you can point at a heap region
//!   during early init. Dealloc is a no-op (first mile).
//!
//! Public API (stable):
//!   memory::init()
//!   memory::heap_stats() -> HeapStats
//!   unsafe memory::set_heap_region(base, size)   [OS mode]
//!
//! Later you can replace the bump with a proper allocator
//! and add paging/frames; keep the function names stable.

#![allow(dead_code)]
#![allow(unused_variables)]

/// Initialize memory subsystem.
/// - devhost: no-op
/// - OS mode: keep as no-op until you call `set_heap_region` from early init.
pub fn init() {
    #[cfg(feature = "devhost")]
    {
        // Nothing to do on host builds.
    }
    #[cfg(not(feature = "devhost"))]
    {
        // First-mile: real setup occurs when you call `set_heap_region`.
    }
}

/// Summary of the heap state (if configured).
#[derive(Copy, Clone, Debug, Default)]
pub struct HeapStats {
    pub base: usize,
    pub size: usize,
    pub next: usize,
    pub end: usize,
}

#[cfg(feature = "devhost")]
pub fn heap_stats() -> HeapStats {
    HeapStats::default()
}

#[cfg(feature = "devhost")]
#[inline]
fn align_up(v: usize, a: usize) -> usize {
    let m = a - 1;
    (v + m) & !m
}

#[cfg(not(feature = "devhost"))]
mod os_heap {
    use super::HeapStats;
    use core::alloc::{GlobalAlloc, Layout};
    use core::sync::atomic::{AtomicUsize, Ordering::SeqCst};

    // Heap bounds and bump pointer (shared across cores).
    static HEAP_BASE: AtomicUsize = AtomicUsize::new(0);
    static HEAP_END:  AtomicUsize = AtomicUsize::new(0);
    static NEXT:      AtomicUsize = AtomicUsize::new(0);

    #[inline]
    fn align_up(v: usize, a: usize) -> usize {
        let mask = a.saturating_sub(1);
        (v + mask) & !mask
    }

    /// Configure the bump allocator with a raw region [base, base+size).
    /// # Safety
    /// Caller must ensure the region is valid, writable, and not used elsewhere.
    pub unsafe fn set_heap_region(base: usize, size: usize) {
        HEAP_BASE.store(base, SeqCst);
        HEAP_END.store(base.saturating_add(size), SeqCst);
        NEXT.store(base, SeqCst);
    }

    /// Snapshot current heap numbers.
    pub fn heap_stats() -> HeapStats {
        let base = HEAP_BASE.load(SeqCst);
        let end  = HEAP_END.load(SeqCst);
        let next = NEXT.load(SeqCst);
        HeapStats {
            base,
            size: end.saturating_sub(base),
            next,
            end,
        }
    }

    /// Lock-free bump allocator. Dealloc is a no-op (first mile).
    pub struct BumpAlloc;

    unsafe impl GlobalAlloc for BumpAlloc {
        unsafe fn alloc(&self, layout: Layout) -> *mut u8 {
            let mut cur = NEXT.load(SeqCst);
            loop {
                let aligned = align_up(cur, layout.align().max(1));
                let new_next = aligned.saturating_add(layout.size());
                let end = HEAP_END.load(SeqCst);
                if new_next > end || aligned < cur {
                    return core::ptr::null_mut();
                }
                match NEXT.compare_exchange(cur, new_next, SeqCst, SeqCst) {
                    Ok(_) => return aligned as *mut u8,
                    Err(actual) => cur = actual,
                }
            }
        }

        unsafe fn dealloc(&self, _ptr: *mut u8, _layout: Layout) {
            // No free in first-mile bump. Fragmentation is fine for now.
        }
    }

    // Install as the global allocator only in OS mode.
    #[global_allocator]
    static GLOBAL: BumpAlloc = BumpAlloc;

    /// OOM path required for `alloc` in no_std.
    #[alloc_error_handler]
    fn on_oom(_layout: Layout) -> ! {
        loop {
            // Halt until interrupt to avoid a hot spin if available.
            #[cfg(any(target_arch = "x86_64", target_arch = "x86"))]
            unsafe { core::arch::asm!("hlt"); }
            #[cfg(not(any(target_arch = "x86_64", target_arch = "x86")))]
            core::hint::spin_loop();
        }
    }

    // Re-export for the outer module.
    pub use set_heap_region as set_heap_region_os;
}
#[cfg(not(feature = "devhost"))]
pub use os_heap::{heap_stats, set_heap_region_os as set_heap_region};

/// In devhost, expose a no-op signature so kernel code compiles unchanged.
#[cfg(feature = "devhost")]
pub unsafe fn set_heap_region(_base: usize, _size: usize) {
    // No effect on host; uses system allocator from std.
}
