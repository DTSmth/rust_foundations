use std::alloc::Layout;

/// # Safety
///
/// This is unsafe because i made the function unsafe
/// could also use a block
unsafe fn my_fn() {

}

fn main() {
    let my_vec = vec![1,2,3,4];
    unsafe {
        // if let Ok(value) = my_vec.get_unchecked(2) {
        //
        // }
    }
}

fn allocate_memory_with_rust() {
    use std::alloc::{alloc, dealloc, Layout};

    unsafe {
        let layout = Layout::new::<u16>();
        let ptr = alloc(layout);
        *ptr = 42;
        assert_eq!(42, *ptr);
        dealloc(ptr, layout);
    }
}
