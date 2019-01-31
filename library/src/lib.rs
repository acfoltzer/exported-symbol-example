use libloading::{Library, Symbol};
use std::ffi::OsStr;

pub fn load_and_run<P: AsRef<OsStr>>(filename: P) {
    // this makes it work without printing an extra message:
    // unsafe {
    //     std::ptr::read_volatile(module_callback as *const extern "C" fn());
    // }

    let module = Library::new(filename).unwrap();
    unsafe {
        let module_func: Symbol<unsafe extern "C" fn()> = module.get(b"module_func").unwrap();
        module_func();
    }
}

#[no_mangle]
pub extern "C" fn module_callback() {
    println!("Hello from the callback!");
}
