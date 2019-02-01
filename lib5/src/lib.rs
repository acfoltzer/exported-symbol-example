use libloading::{Library, Symbol};
use std::ffi::OsStr;

pub fn load_and_run<P: AsRef<OsStr>>(filename: P) {
    let module = Library::new(filename).unwrap();
    unsafe {
        let module_func: Symbol<unsafe extern "C" fn()> = module.get(b"module_func").unwrap();
        module_func();
    }
}

#[no_mangle]
#[used]
pub static module_callback: extern "C" fn() = module_callback_impl;

pub extern "C" fn module_callback_impl() {
    println!("Hello from the callback!");
}
