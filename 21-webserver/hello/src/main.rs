use std::{
    fs,
    io::{self, BufRead, BufReader, Write},
    net::{TcpListener, TcpStream},
    path::Path,
    thread, time::Duration,
};
use hello::ThreadPool;

fn main() {
    let listener = TcpListener::bind("127.0.0.1:7878").unwrap();
    let pool = ThreadPool::new(5);

    for stream in listener.incoming().take(2) {
        let stream = match stream {
            Err(e) => {
                eprintln!("Error getting stream {e}");
                continue;
            }
            Ok(stream) => stream,
        };
        pool.execute(|| handle_connection(stream));
    }
}

fn handle_connection(mut stream: TcpStream) {
    let buf = BufReader::new(&stream);
    let request_line = buf.lines().next().unwrap().unwrap();
    // let http_request: Vec<_> = buf
    //     .lines()
    //     .map(|line| line.unwrap())
    //     .take_while(|line| !line.is_empty())
    //     .collect();
    // println!("Http request is {http_request:#?}");

    let (status, html_filepath) = match &request_line[..] {
        "GET / HTTP/1.1" => ("200 OK", Path::new("index.html")),
        "GET /sleep HTTP/1.1" => {
            thread::sleep(Duration::from_secs(5));
            ("200 OK", Path::new("index.html"))
        },
        _ => ("404 NOT FOUND", Path::new("404.html")),
    };

    send_html(&mut stream, status, html_filepath).unwrap()
}

fn send_html(stream: &mut TcpStream, status: &str, html_filepath: &Path) -> Result<(), io::Error> {
    let body = fs::read_to_string(html_filepath).unwrap();
    let length = body.len();

    let response = format!("HTTP/1.1 {status}\r\nContent-Length: {length}\r\n\r\n{body}");

    stream.write_all(response.as_bytes())
}
