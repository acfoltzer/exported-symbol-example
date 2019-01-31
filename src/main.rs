use library::{S, T};

fn main() {
    let s = S {};
    let t = &s as &T;
    t.load_and_run("target/debug/libmodule.so");
}
