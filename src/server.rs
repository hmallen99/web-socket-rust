pub mod web_socket {
	use std::{
		io::{prelude::*, BufReader},
		net::{TcpListener, TcpStream}, fs
	};

	pub fn listen(uri: &String) {
		if let Ok(listener) = TcpListener::bind(uri) {
			println!("Listening on {}", uri);

			for stream in listener.incoming() {
				let stream = stream.unwrap();

				handle_connection(stream);
			}
		} else {
			println!("Failed to start server");
		}
	}

	fn handle_connection(mut stream: TcpStream) {
		let buf_reader = BufReader::new(&mut stream);
		let http_request: Vec<_> = buf_reader
			.lines()
			.map(|result| result.unwrap())
			.take_while(|line| !line.is_empty())
			.collect();

		let status_line = "HTTP/1.1 200 OK";
		let contents = fs::read_to_string("hello.html").unwrap();
		let length = contents.len();

		let response =
			format!("{status_line}\r\nContent-Length: {length}\r\n\r\n{contents}");

		stream.write_all(response.as_bytes()).unwrap();
	}
}
