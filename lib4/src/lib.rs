use libloading::{Library, Symbol};

#[no_mangle]
pub extern "C" fn module_callback() {
    println!("Hello from the callback!");
}

pub trait T {
    fn ensure_linked(&self) {
        // only difference from lib3 is that this line is commented out:
        // println!("{:?}", S {});
    }

    fn load_and_run(&self, filename: &str) {
        let module = Library::new(filename).unwrap();
        unsafe {
            let module_func: Symbol<unsafe extern "C" fn()> = module.get(b"module_func").unwrap();
            module_func();
        }
    }
}

#[derive(Debug)]
pub struct S {}

impl T for S {}
