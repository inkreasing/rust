struct A;

impl A {
    fn a() -> true {} //~ ERROR: expected type, found keyword `true`
    fn b(&self) {}
}

fn main() {
    let a = A;
    a.b(); //~ ERROR E0599

    let b = B;
    b.a(()); //~ ERROR E0061
}

struct B;

impl B {
    fn a(&self) {}
    fn b() -> true {} //~ ERROR: expected type, found keyword `true`
}
