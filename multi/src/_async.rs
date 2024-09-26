use async_std::task;
use std::io::prelude::*;
use std::rc::Rc;
use std::*;

async fn aa() -> io::Result<()> {
    let listener = net::TcpListener::bind("127.0.0.1")?;

    // for socket_result in listener.incoming() {
    //     let socket = socket_result?;
    //     let groups = chat_group_table.clone();
    //     thread::spawn(|| {
    //         log_error(serve(socket, groups));
    //     });
    // }

    // let mut new_connections = listener.incoming();
    // while let Some(socket_result) = new_connections.next().await {
    //     let socket = socket_result?;
    //     let groups = chat_group_table.clone();
    //     task::spawn(async {
    //         log_error(serve(socket, groups).await);
    //     });
    // }

    let response = cheapo_request("127.0.0.1", 80, "/");
    Ok(())
}

fn cheapo_request(host: &str, port: u16, path: &str) -> std::io::Result<String> {
    use std::net;
    let mut socket = net::TcpStream::connect((host, port))?;

    let request = format!("GET {} HTTP/1.1\r\nHost: {}\r\n\r\n", path, host);
    socket.write_all(request.as_bytes())?;
    socket.shutdown(net::Shutdown::Write)?;

    let mut response = String::new();
    socket.read_to_string(&mut response)?;

    Ok(response)
}

async fn cheapo_request2(host: &str, port: u16, path: &str) -> std::io::Result<String> {
    use async_std::io::prelude::*;
    use async_std::net;

    let mut socket = net::TcpStream::connect((host, port)).await?;

    let request = format!("GET {} HTTP/1.1\r\nHost: {}\r\n\r\n", path, host);
    socket.write_all(request.as_bytes()).await?;
    socket.shutdown(net::Shutdown::Write)?;

    let mut response = String::new();
    socket.read_to_string(&mut response).await?;

    Ok(response)
}

#[test]
fn main() -> std::io::Result<()> {
    use async_std::task;

    let response = task::block_on(cheapo_request2("baidu.com", 80, "/"))?;
    println!("{:#?}", response);

    let requests = vec![
        ("baidu.com".to_string(), 80, "/".to_string()),
        ("zhihu.com".to_string(), 80, "/".to_string()),
    ];

    let results = async_std::task::block_on(many_requests(requests));
    for result in results {
        match result {
            Ok(response) => println!("{}", response),
            Err(err) => eprintln!("error: {}", err),
        }
    }

    let input = async_std::io::stdin();
    let future = async {
        let mut line = String::new();

        // 这会返回`std::io::Result<usize>`
        input.read_line(&mut line).await?;

        println!("Read line: {}", line);

        Ok::<(), std::io::Error>(())
    };

    Ok(())
}

pub async fn many_requests(requests: Vec<(String, u16, String)>) -> Vec<std::io::Result<String>> {
    async fn cheapo_owning_request(
        host: String,
        port: u16,
        path: String,
    ) -> std::io::Result<String> {
        cheapo_request2(&host, port, &path).await
    }

    let mut handles = vec![];
    for (host, port, path) in requests {
        handles.push(
            task::spawn_local(async move { cheapo_request2(&host, port, &path).await }), // task::spawn_local(cheapo_owning_request(host, port, path)) // spawn_local 只会接受生命周期为 'static 的 Future
        ); // unstable
    }
    let mut results = vec![];
    for handle in handles {
        results.push(handle.await);
    }
    results
}

async fn reluctant() -> String {
    let res = {
        let string = Rc::new("ref-counted string".to_string());
        format!("Your splendid string: {}", string)
    };

    let requests = vec![("baidu.com".to_string(), 80, "/".to_string())];
    many_requests(requests).await;
    //  1、限制非send值的作用域，使其不跨越任何await表达式的作用域
    res
}

#[test]
fn mai2n() {
    use async_std::task;
    task::spawn(reluctant());
}
