use std::net::TcpListener;
use std::net::TcpStream;
use std::io::prelude::*;

fn main() {
	//listener
	let listener: TcpListener =
		TcpListener::bind("127.0.0.1:7878").unwrap();
	
	//listen
	for stream in listener.incoming(){
		let stream = stream.unwrap();
		//println!("Connection established");
		handle_connection(stream);
	}
}

//handle request
fn handle_connection(mut stream: TcpStream){
	//buffer to store basic request
	let mut buffer: [u8; 1024] = [0; 1024];
	stream.read(&mut buffer).unwrap();
	println!(
		"Request: {}",
		String::from_utf8_lossy(&buffer[..])
	)
}
