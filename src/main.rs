fn main() {
    // if this call is uncommented, the example works, but prints an extra message:
    // library::module_callback();

    // this makes it work without printing an extra message:
    // unsafe {
    //     std::ptr::read_volatile(library::module_callback as *const extern "C" fn());
    // }

    library::load_and_run("target/debug/libmodule.so");
}

// if this definition is uncommented, the example works:
// #[no_mangle]
// pub extern "C" fn module_callback() {
//     println!("Hello from the callback!");
// }
