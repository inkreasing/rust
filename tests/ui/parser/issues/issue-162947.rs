struct A;

impl A {
    fn a() -> true {} //~ ERROR: expected type, found keyword `true`
    fn b(&self) {}
}

fn main() {
    let a = A;
    a.b();

    let b = B;
    // this should also error
    b.a(());
}

struct B;

impl B {
    fn a(&self) {}
    // this should error
    fn b() -> true {}
}
