//! Unsafe Rust: raw pointers, unsafe functions, static mut, and unsafe traits.
//! Keep unsafe blocks small and wrapped in safe abstractions when possible.

static mut COUNTER: u32 = 0;

unsafe fn inc_counter() {
    unsafe {
        COUNTER += 1;
    }
}

unsafe trait Dangerous {}
unsafe impl Dangerous for u32 {}

fn get_first_mut<T>(slice: &mut [T]) -> Option<&mut T> {
    // Safe wrapper around raw pointer manipulation.
    if slice.is_empty() { return None; }
    let ptr = slice.as_mut_ptr();
    // SAFETY: ptr came from a valid &mut [T], index 0 is in-bounds.
    Some(unsafe { &mut *ptr })
}

fn main() {
    // 1) Raw pointers: can be created in safe code, dereferenced only in unsafe.
    let mut x = 10;
    let r1 = &x as *const i32;
    let r2 = &mut x as *mut i32;
    println!("Raw pointers created: r1={:?}, r2={:?}", r1, r2);

    // SAFETY: We ensure r2 points to valid, unique memory here.
    unsafe {
        *r2 = 42;
        println!("Dereferenced raw pointer: *r2 = {}", *r2);
    }

    // 2) Calling an unsafe function and accessing static mut.
    unsafe {
        inc_counter();
        println!("COUNTER = {}", { COUNTER });
    }

    // 3) Interacting with FFI (decl only). Don't call without linking.
    unsafe extern "C" { fn abs(input: i32) -> i32; }
    let _abs_decl_only = abs as *const (); // avoid link errors by not calling

    // 4) Safe abstraction over unsafe code.
    let mut data = [1, 2, 3];
    if let Some(first) = get_first_mut(&mut data) {
        *first = 99;
    }
    println!("data = {:?}", data);

    // 5) Unsafe trait: marker that requires extra invariants from implementers.
    let _: &dyn Dangerous = &0u32;
}
