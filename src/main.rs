use garden::echo;

fn main() {
    build_vector();
    println!("{}", echo(12))
}

// Box：指向堆中值的拥有型指针
// Box: <Attend>

fn build_vector() -> Vec<i16> {
    let mut v: Vec<i16> = Vec::<i16>::new();
    v.push(10i16);
    v.push(20i16);
    v
}
