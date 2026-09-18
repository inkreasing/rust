pub trait TraitWAssocConst {
    const A:   usize;
}
pub struct Demo {}

impl TraitWAssocConst for impl Demo {
    pubconst A: str = 32; //~ ERROR expected one of
}

fn foo<A: TraitWAssocConst<A=32>>() {
    foo::<Demo>()();
}

fn main<A: TraitWAssocConst<A=32>>() {
    foo::<Demo>();
}
