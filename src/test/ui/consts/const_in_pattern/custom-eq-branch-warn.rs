// check-pass

#![feature(const_if_match)]
#![warn(indirect_structural_match)]
//~^ NOTE lint level is defined here

struct CustomEq;

impl Eq for CustomEq {}
impl PartialEq for CustomEq {
    fn eq(&self, _: &Self) -> bool {
        false
    }
}

#[derive(PartialEq, Eq)]
enum Foo {
    Bar,
    Baz,
    Qux(CustomEq),
}

// We know that `BAR_BAZ` will always be `Foo::Bar` and thus eligible for structural matching, but
// dataflow will be more conservative.
const BAR_BAZ: Foo = if 42 == 42 {
    Foo::Bar
} else {
    Foo::Qux(CustomEq)
};

fn main() {
    match Foo::Qux(CustomEq) {
        BAR_BAZ => panic!(),
        //~^ WARN must be annotated with `#[derive(PartialEq, Eq)]`
        //~| WARN this was previously accepted
        //~| NOTE see issue #62411
        _ => {}
    }
}