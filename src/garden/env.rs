use std::env;

#[test]
fn main() {
    env::args().for_each(|x| {
        println!("{x}");
    })
}
