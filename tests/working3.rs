#[test]
fn main() {
    // using a `T` trait object
    let s = lib3::S {};
    let t = &s as &dyn lib3::T;
    t.load_and_run("target/debug/libmodule.so");
}
