mod server;

use crate::server::web_socket;

fn main() {
	web_socket::listen(&String::from("127.0.0.1:4000"));
}