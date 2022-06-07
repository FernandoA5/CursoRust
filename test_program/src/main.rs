use std::io::prelude::*;
use std::net::TcpListener;
use std::net::TcpStream;
use std::fs;
fn main() {
    let listener = TcpListener::bind("192.168.1.79:7878").unwrap();

    for stream in listener.incoming()
    {
        let stream = stream.unwrap();
        handle_connection(stream);
    }
}
fn handle_connection(mut stream: TcpStream)
{
    let mut buffer = [0; 1024];
    stream.read(&mut buffer).unwrap();

    let get = b"GET / HTTP/1.1\r\n";
    let page2 = b"GET /page2 HTTP/1.1\r\n";
    let js= b"GET /app.js HTTP/1.1\r\n";
    let m= b"GET /manifest.json HTTP/1.1\r\n";
    let i1=b"GET /images/icons-192.png HTTP/1.1\r\n";
    
    let (status_line, filename) = if buffer.starts_with(get){
        ("HTTP/1.1 200 OK", "index.html")
    } else if buffer.starts_with(page2){
        ("HTTP/1.1 200 OK", "page2.html")
    } else if buffer.starts_with(js){
        ("HTTP/1.1 200 OK", "js/app.js")
    } else if buffer.starts_with(m){
        ("HTTP/1.1 200 OK", "manifest.json")
    } else if buffer.starts_with(i1){
        ("HTTP/1.1 200 OK", "images/incons-192.png")
    }
    else {
        ("HTTP/1.1 404 NOT FOUND", "404.html")
    };


    let contents= fs::read_to_string(filename).unwrap();

    let response = format!(
        "{}\r\nContent-Length: {}\r\n\r\n{}",
        status_line,
        contents.len(),
        contents
    );

    stream.write(response.as_bytes()).unwrap();
    stream.flush().unwrap();
    
}
