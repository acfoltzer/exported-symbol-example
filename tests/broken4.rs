#[test]
#[should_panic]
fn main() {
    let s = lib4::S {};
    let t = &s as &lib4::T;
    t.load_and_run("target/debug/libmodule.so");
}
