use std::fs::OpenOptions;
use std::io::Write;
use actix_web::web::Path;

fn say_hello(out: &mut dyn Write) -> std::io::Result<()> {
    out.write_all(b"hello world\n")?;
    out.flush()
}


fn write_log_entry(entry: std::fmt::Arguments) {
    // 尽量保持简单，所以每次只是打开文件
    let mut log_file = OpenOptions::new()
        .append(true)
        .create(true)
        .open("log-file-name")
        .expect("failed to open log file");

    log_file.write_fmt(entry)
        .expect("failed to write to log");
}

macro_rules! log { // 在宏定义中的宏名后不需要叹号（!）
    ($format:tt, $($arg:expr),*) => (
        write_log_entry(format_args!($format, $($arg),*))
    )
}
#[test]
fn f() {
    use std::fs::File;
    let mut local_file = File::create("hello.txt").expect("");
    say_hello(&mut local_file); // 正常
    let mut bytes = vec![];
    say_hello(&mut bytes); // 同样正常
    assert_eq!(bytes, b"hello world\n");
    log!("{:?}\n","asd".to_string());







}
