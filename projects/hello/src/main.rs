use std::io::Read;
use std::io::Write;
use std::net::TcpListener;

fn main() {
	let mut port: std::env::Args = std::env::args();
	let prog_name: String = port.next().expect("There is no program name.");
	let port: String = port.next().expect(&format!("Usage: {} <port>", prog_name));
	let listener: std::net::TcpListener = TcpListener::bind(format!("127.0.0.1:{}", port)).unwrap();
	let stream = listener.incoming().next().unwrap().unwrap();
	handle_connection(stream);
}

fn handle_connection(mut stream: std::net::TcpStream) {
	let mut buffer: [u8; 1024] = [0; 1024];
	stream.read(&mut buffer).unwrap();
	let request: String = (*String::from_utf8_lossy(&buffer[..])).to_string();
	println!("request = {}", request);
	let get: &str = "GET / HTTP/1.1\r\n";
	if request.starts_with(get) {
		let mut html_file: std::fs::File = std::fs::File::open("hello.html").unwrap();
		let mut html: String = String::new();
		html_file.read_to_string(&mut html).unwrap();
		let responce: String = format!("HTTP/1.1 200 OK\n\n{}", html);
		let responce: String = responce.replace("\n", "\r\n");
		stream.write(responce.as_bytes()).unwrap();
		stream.flush().unwrap();
	} else {
		let mut html_file: std::fs::File = std::fs::File::open("404.html").unwrap();
		let mut html: String = String::new();
		html_file.read_to_string(&mut html).unwrap();
		let responce: String = format!("HTTP/1.1 404 NOT FOUND\n\n{}", html);
		let responce: String = responce.replace("\n", "\r\n");
		stream.write(responce.as_bytes()).unwrap();
		stream.flush().unwrap();
	}
}

