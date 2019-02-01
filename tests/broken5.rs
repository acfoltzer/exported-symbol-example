#[test]
#[should_panic]
fn main() {
    lib5::load_and_run("target/debug/libmodule.so");
}
