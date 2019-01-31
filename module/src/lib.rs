extern "C" {
    fn module_callback();
}

#[no_mangle]
pub extern "C" fn module_func() {
    println!("Hello from the module!");
    unsafe {
        module_callback();
    }
}
