pub mod web_socket {
	use std::{
		io::{prelude::*, BufReader},
		net::{TcpListener, TcpStream}, fs
	};

	use regex::Regex;
	use lazy_static::lazy_static;
	use sha1::{Sha1, Digest};
	use base64::encode;

	pub fn listen(uri: &String) {
		if let Ok(listener) = TcpListener::bind(uri) {
			println!("Listening on {}", uri);
			let mut did_handshake = false;

			for stream in listener.incoming() {
				let stream = stream.unwrap();

				if did_handshake {
					println!("Receiving web socket messages...");
				} else {
					did_handshake = handshake(stream);
				}
			}
		} else {
			println!("Failed to start server");
		}
	}

	fn handshake(mut stream: TcpStream) -> bool {
		let buf_reader = BufReader::new(&mut stream);
		let data = buf_reader
			.lines()
			.map(|result| result.unwrap_or(String::from("")))
			.take_while(|line| !line.is_empty())
			.collect();

		if !is_get_match(&data) {
			let status_line = "HTTP/1.1 405 Method Not Allowed";
			let path = "405.html";
			let response = format_http_response(status_line, path);

			stream.write_all(response.as_bytes()).unwrap();
			return false;
		}

		if !is_websocket_handshake(&data) {
			let status_line = "HTTP/1.1 400 Bad Request";
			let path = "400.html";
			let response = format_http_response(status_line, path);

			stream.write_all(response.as_bytes()).unwrap();
			return false;
		}

		let status_line = "HTTP/1.1 101 Switching Protocols\r\n";
		let connection_header = "Connection: Upgrade\r\n";
		let upgrade_header = "Upgrade: websocket\r\n";

		let mut hasher = Sha1::new();
		let key = get_web_socket_key(&data);
		hasher.update(key.as_bytes());
		let encoded = encode(hasher.finalize());

		let sec_web_socket_accept =
			format!("Sec-WebSocket-Accept: {}\r\n\r\n", encoded);

		let response =
			format!("{status_line}{connection_header}{upgrade_header}{sec_web_socket_accept}");


		stream.write_all(response.as_bytes()).unwrap();

		println!("completed handshake");
		return true;
	}

	fn is_get_match(text: &String) -> bool {
		lazy_static! {
			static ref RE: Regex = Regex::new(r"^GET").unwrap();
		}
		RE.is_match(text)
	}

	fn is_websocket_handshake(text: &String) -> bool {
		lazy_static! {
			static ref RE: Regex = Regex::new(r"Sec-WebSocket-Key: (.*)").unwrap();
		}
		RE.is_match(text)
	}

	fn format_http_response(status_line: &str, path: &str) -> String{
		let contents = fs::read_to_string(path).unwrap();
		let length = contents.len();
		format!("{status_line}\r\nContent-Length: {length}\r\n\r\n{contents}")
	}

	fn get_web_socket_key(text: &String) -> String {
		lazy_static! {
			static ref RE: Regex = Regex::new(r"Sec-WebSocket-Key: (.*==)").unwrap();
		}
		let caps = RE.captures(text).unwrap();
		let key = caps.get(1).map_or("", |m| m.as_str());
		format!("{key}258EAFA5-E914-47DA-95CA-C5AB0DC85B11")
	}
}
