#![allow(non_snake_case)]

#[no_mangle]
pub extern "C" fn memcpy(dest: *mut u8, src: *const u8, n: usize) -> *mut u8 {
    unsafe {
        for i in 0..n {
            *dest.add(i) = *src.add(i);
        }
    }
    dest
}

#[no_mangle]
pub extern "C" fn memset(dest: *mut u8, c: i32, n: usize) -> *mut u8 {
    let byte = c as u8;
    unsafe {
        for i in 0..n {
            *dest.add(i) = byte;
        }
    }
    dest
}

#[no_mangle]
pub extern "C" fn memcmp(a: *const u8, b: *const u8, n: usize) -> i32 {
    unsafe {
        for i in 0..n {
            let av = *a.add(i);
            let bv = *b.add(i);
            if av != bv {
                return (av as i32) - (bv as i32);
            }
        }
    }
    0
}
