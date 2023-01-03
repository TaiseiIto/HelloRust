use std::io::Read;
use std::net::TcpListener;

fn main() {
	let listener: std::net::TcpListener = TcpListener::bind("127.0.0.1:7878").unwrap();
	let stream = listener.incoming().next().unwrap().unwrap();
	handle_connection(stream);
}

fn handle_connection(mut stream: std::net::TcpStream) {
	let mut buffer: [u8; 1024] = [0; 1024];
	stream.read(&mut buffer).unwrap();
	let request: String = (*String::from_utf8_lossy(&buffer[..])).to_string();
	println!("request = {}", request);
}

