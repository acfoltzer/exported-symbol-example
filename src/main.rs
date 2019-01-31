fn main() {
    // if this call is uncommented, the example works:

    // library::module_callback();

    library::load_and_run("target/debug/libmodule.so");
}

// if this definition is uncommented, the example works:

// #[no_mangle]
// pub extern "C" fn module_callback() {
//     println!("Hello from the callback!");
// }
