pub mod garden;

pub use garden::echo;

fn foo2(a: &u32, b: &u32) {
    if a > b {
        println!("111");
    } else {
        println!("222");
    }
}

#[derive(Debug)]

enum MyEnum2<'ax> {
    Add,
    Subtract,
    Mix(&'ax str),
}

fn main() {
    trait TraitA {
        fn foo(&self);
    }

    trait TraitB {
        fn bar(&self);
    }

    impl<T: TraitB> TraitA for T {
        fn foo(&self) {}
    }

    impl TraitA for u32 {
        fn foo(&self) {}
    }

    let x: u32 = 99;

    x.foo();
}
