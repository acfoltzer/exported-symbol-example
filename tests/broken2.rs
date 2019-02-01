#[test]
#[should_panic]
fn main() {
    lib2::load_and_run("target/debug/libmodule.so");
}
