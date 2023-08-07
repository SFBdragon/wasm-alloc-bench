#![feature(allocator_api)]

use std::alloc::Layout;
use wasm_bindgen::prelude::*;

// comment out both to bench dlmalloc
// comment out one to bench the other

#[cfg(feature = "talc")]
#[global_allocator]
static TALCK: talc::TalckWasm = unsafe { talc::TalckWasm::new_global() };

#[cfg(feature = "lol_alloc")]
#[global_allocator] static ALLOC: lol_alloc::AssumeSingleThreaded<lol_alloc::FreeListAllocator> = 
    unsafe { lol_alloc::AssumeSingleThreaded::new(lol_alloc::FreeListAllocator::new()) };


#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(js_namespace = console)]
    fn log(s: &str);
}

#[wasm_bindgen]
pub fn bench() {
    console_error_panic_hook::set_once();
    
    let benches = 5;
    let mut total_time = 0f64;
    for i in 0..benches {
        let mut stats = easybench_wasm::bench(|| random_actions_10_000_000());
        // I'm guessing the Performance.now() uses us instead of ns since this crate was maintained?
        stats.ns_per_iter *= 1000.0;
        log(format!("bench {}: {}", i, stats).as_str());
        total_time += stats.ns_per_iter;
    }
    log(format!("average time: {} ms", total_time / benches as f64 / 1000000.0).as_str());
}

fn random_actions_10_000_000() {
    let mut score = 0;
    let mut v = Vec::with_capacity(10000);

    while score < 100_000 {
        for _ in 0..100 {
            let action = fastrand::usize(0..3);
    
            match action {
                0 => {
                    let size = fastrand::usize(100..=1000);
                    let align = 8 << fastrand::u16(..).trailing_zeros() / 2;
                    let layout = Layout::from_size_align(size, align).unwrap();
    
                    let allocation = unsafe { std::alloc::alloc(layout) };
    
                    if !allocation.is_null() {
                        v.push((allocation, layout));
                        score += 1;
                    }
                }
                1 => {
                    if !v.is_empty() {
                        let index = fastrand::usize(0..v.len());
                        let (ptr, layout) = v.swap_remove(index);
    
                        unsafe {
                            std::alloc::dealloc(ptr, layout);
                        }
    
                        score += 1;
                    }
                }
                2 => {
                    if !v.is_empty() {
                        let index = fastrand::usize(0..v.len());
                        if let Some((ptr, layout)) = v.get_mut(index) {
                            let new_size = fastrand::usize(100..=10000);
    
                            unsafe {
                                let realloc = std::alloc::realloc(*ptr, *layout, new_size);
    
                                if !realloc.is_null() {
                                    *ptr = realloc;
                                    *layout = Layout::from_size_align_unchecked(new_size, layout.align());
                                    score += 1;
                                }
                            }
                        }
                    }
                }
                _ => unreachable!(),
            }
        }
    }

    for (ptr, layout) in v {
        unsafe { std::alloc::dealloc(ptr, layout); }
    }
}


