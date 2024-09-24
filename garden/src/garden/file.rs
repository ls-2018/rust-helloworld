use std::io::Write;

fn say_hello(out: &mut dyn Write) -> std::io::Result<()> {
    out.write_all(b"hello world\n")?;
    out.flush()
}

#[test]
fn f() {
    use std::fs::File;
    let mut local_file = File::create("hello.txt").expect("");
    say_hello(&mut local_file); // 正常
    let mut bytes = vec![];
    say_hello(&mut bytes); // 同样正常
    assert_eq!(bytes, b"hello world\n");
}
