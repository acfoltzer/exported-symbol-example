use lib3::T;

#[test]
#[should_panic]
fn main() {
    // no trait object, just using concrete `S`
    let s = lib3::S {};
    s.load_and_run("target/debug/libmodule.so");
}
